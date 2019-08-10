/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-kernel/0.2.0")]
#![no_std]
#![no_main]
#![feature(asm)]
//! # RusPiRo kernel
//! 
//! This crate aims to be THE RusPiRo kernel somewhen in the future. At the time beeing it's used to verify the usage 
//! and proper function of the different RusPiRo feature crates.
//! 
//! # Dependencies
//! 
//! To succsessfully build this crate for Raspberry Pi use ``xargo build`` with ``--target armv7-unknown-linux-gnueabihf``.
//! Check out the [README.md](https://github.com/RusPiRo/ruspiro-kernel/blob/master/README.md) for further details.
//! 

use ruspiro_sdk::*;

come_alive_with!(being_alive);
run_with!(thinking);

fn being_alive(core: u32) {
    // on the first core coming alive we initialize the Uart
    if core == 0 {
        let init_ok = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core))
            // when we know the core clock rate we can do further initializations
            // start with Uart
            .and_then(|core_rate| {
                
                let mut uart = Uart0::new();
                uart.initialize(core_rate, 115_200)
                    // if uart went fine do console
                    .and_then(|_| {
                        CONSOLE.take_for(|console| console.replace(uart));
                        println!("UART ready for use from core 0...");
                        Ok(()) 
                    })
                    // if this went fine do the I²C bus
                    .and_then(|_| {
                        I2C.take_for(|i2c| i2c.initialize(core_rate, true))
                    })
            });

        if init_ok.is_err() {
            GPIO.take_for(|gpio| gpio.get_pin(17).unwrap().to_output().high() );
        } else {
            // if init went fine we will have access to I2C...scan for devices
            I2C.take_for(|i2c| {
                info!("scan I2C devices...");
                i2c.scan();
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

