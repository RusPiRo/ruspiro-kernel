/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-kernel/0.2.0")]
#![no_std]
#![no_main]
#![feature(llvm_asm)]
//! # RusPiRo kernel
//! 
//! This crate demonstrates and verifies the usage of the different RusPiRo crates. The current version does nothing
//! visible. It's just able to successfully build a running baremetel kernel to be put onto the Raspberry Pi and spinning
//! all 4 cores into a usable state for further processing.
//! 
//! # Dependencies
//! 
//! To succsessfully build this crate for Raspberry Pi use ``xargo build`` with ``--target armv7-unknown-linux-gnueabihf``.
//! Check out the [README.md](https://github.com/RusPiRo/ruspiro-kernel/blob/master/README.md) for further details.
//! 
//! # Expected result
//! 
//! If this version is successfully build and deployed to a Raspberry Pi 3 the connected terminal console should print
//! the following:
//! ```
//! UART ready for use from core 0...
//! hello from core 0
//! hello from core 1
//! hello from core 2
//! hello from core 3
//! Timer IRQ <- this one get's repeated in a constant time intervall
//! ```
//! 
//! In addition the 4 LED's (when connected to GPIO's 17, 18, 20 and 21) should blink.
//! 

#[macro_use]
extern crate ruspiro_boot;
// use GPIO abstraction
use ruspiro_gpio::GPIO;
// use UART0 (miniUART) abstraction
use ruspiro_uart::Uart0;
// use console to attach the uart0 as the output channel
use ruspiro_console::*;
// use the mailbox interface to get the real core clock rate
use ruspiro_mailbox::{MAILBOX, ArmClockId};
// use timer to blink the LED's
use ruspiro_timer as timer;

// The follwoing code is used to demonstrate/show case implementation of a simple interrupt handler
extern crate ruspiro_interrupt;
use ruspiro_interrupt::*;
use ruspiro_register::define_registers;

// Implement the handler for the ArmTimer interrupt
#[IrqHandler(ArmTimer)]
fn handle_timer() {
    TIMERIRQ::Register.set(1);
    println!("Timer IRQ");
}

// define required registers for the ArmTimer to raise interrupts and to acknowledge them
define_registers! [
    TIMERLOAD: ReadWrite<u32> @ 0x3F00_B400,
    TIMERCTRL: WriteOnly<u32> @ 0x3F00_B408,
    TIMERIRQ: WriteOnly<u32> @ 0x3F00_B40C
];

come_alive_with!(being_alive);
run_with!(thinking);

fn being_alive(core: u32) {
    // on the first core coming alive we initialize the Uart
    if core == 0 {
        IRQ_MANAGER.take_for(|irq_mgr| irq_mgr.initialize());
        let inialized = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core))
            .and_then(|core_rate| {
                let mut uart = Uart0::new();
                uart.initialize(core_rate, 115_200).map(|_| uart ) })
            .and_then(|uart| {
                CONSOLE.take_for(|console| console.replace(uart));
                println!("UART ready for use from core 0...");
                Ok(())
            });

        if inialized.is_err() {
            GPIO.take_for(|gpio| gpio.get_pin(17).unwrap().to_output().high() );
        } else {
            // if everything went fine so far configure the
            TIMERCTRL::Register.set(0x3E_02A2); // this should start timer, enable IRQ and set 32bit counter
            TIMERLOAD::Register.set(100_000); // set timer value            
            IRQ_MANAGER.take_for(|irq_mgr| {
                irq_mgr.activate(Interrupt::ArmTimer);
                irq_mgr.enable();
            });
        }
    }
    println!("hello from core {}", core);
}

fn thinking(core: u32) -> ! {
    // get the pin we would like to blink based on the core
    let maybe_pin = match core {
        0 => GPIO.take_for(|gpio| gpio.get_pin(17).map(|pin| pin.to_output() ) ),
        1 => GPIO.take_for(|gpio| gpio.get_pin(18).map(|pin| pin.to_output() ) ),
        2 => GPIO.take_for(|gpio| gpio.get_pin(20).map(|pin| pin.to_output() ) ),
        3 => GPIO.take_for(|gpio| gpio.get_pin(21).map(|pin| pin.to_output() ) ),
        _ => Err("well - core > 3 found :o")
    };

    loop {
        // match on the pin reference to prevent it from beein moved which makes it in-accessible in subsequent loops
        match &maybe_pin {
            Ok(pin) => {
                pin.high();
                timer::sleep(100_000);
                pin.low();
                timer::sleep(200_000);
            }
            _ => ()
        }
     }
}

