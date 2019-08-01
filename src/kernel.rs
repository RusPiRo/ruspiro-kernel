/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-kernel/0.1.0")]
#![no_std]
#![no_main]
//! # RusPiRo kernel
//! 
//! This crate demonstrates and verifies the usage of the different RusPiRo crates. The current version does nothing
//! visible. It's just able to successfully build a running baremetel kernel to be put onto the Raspberry Pi and spinning
//! all 4 cores into a usable state for further processing.
//! 
//! # Dependencies
//! 
//! To succsessfully build this crate a custom target has been defined as ``armv8-ruspiro``. Therefore
//! use ``xargo`` with ``--target armv8-ruspiro`` to run the build. As a custom target need specific config files
//! present in specific file locations it is highly recomended to use the makefile that is part of the crate.
//! Check out the [README.md](https://github.com/RusPiRo/ruspiro-kernel/blob/master/README.md) for further details
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
//! ```
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

come_alive_with!(being_alive);
run_with!(thinking);

fn being_alive(core: u32) {
    // on the first core coming alive we initialize the Uart
    if core == 0 {
        if let Ok(core_rate) = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core)) {
            let mut uart = Uart0::new();
            if uart.initialize(core_rate, 115_200).is_ok() {
                CONSOLE.take_for(|console| console.replace(uart));
                // if uart and console could be initialized lit the core 0 LED
                GPIO.take_for(|gpio| gpio.get_pin(17).unwrap().to_output().high() );
                // and also write a test string
                println!("UART ready for use from core 0...");
            }
        }
    }
    // based on the core coming alive we would lit a different LED to see all 4 are kicked-off
    // we do assume that there will be no issue in getting the pin's, so unwrap should never fail ;)
    match core {
        1 => GPIO.take_for(|gpio| gpio.get_pin(18).unwrap().to_output().high() ),
        2 => GPIO.take_for(|gpio| gpio.get_pin(20).unwrap().to_output().high() ),
        3 => GPIO.take_for(|gpio| gpio.get_pin(21).unwrap().to_output().high() ),
        _ => (), // nothing to do in case there is a core number higher than 3 running - RPi has only 4 cores ;)
    }

    println!("hello from core {}", core);
}

fn thinking(_: u32) -> ! {
    loop { }
}
