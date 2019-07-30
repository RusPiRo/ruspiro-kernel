/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-kernel/0.0.1")]
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
#[macro_use]
extern crate ruspiro_boot;

use ruspiro_register::define_registers;

come_alive_with!(being_alive);
run_with!(thinking);

// depending on the target family beeing Raspberry Pi3 or not the MMIO base address is diffrent.
#[cfg(target_family="ruspiro-pi3")]
const GPIO_BASE: u32 = 0x3F20_0000;

#[cfg(not(target_family="ruspiro-pi3"))]
const GPIO_BASE: u32 = 0x2020_0000;


// define GPIO registers to be used to set a GPIO pin to lit a LED connected to it
define_registers! [
    GPFSEL1: ReadWrite<u32> @ GPIO_BASE + 0x04 => [
        FSEL17 OFFSET(21) BITS(3),
        FSEL18 OFFSET(24) BITS(3)
    ],
    GPFSEL2: ReadWrite<u32> @ GPIO_BASE + 0x08 => [
        FSEL20 OFFSET(0) BITS(3),
        FSEL21 OFFSET(3) BITS(3)
    ],
    GPSET0: WriteOnly<u32> @ GPIO_BASE + 0x1C => [],
    GPCLR0: WriteOnly<u32> @ GPIO_BASE + 0x28 => []
];

fn being_alive(core: u32) {
    // based on the core coming alive we would lit a different LED to see all 4 are kicked-off
    // this is safe as it is ensured by the ruspiro-boot crate the cores hit this function one after the other
    // the follwing steps are done on each core:
    //  1. set the pin of this core to OUTPUT using the corresponding FSEL register
    //  2. set the bit of the output register corresponding to this pin
    match core {
        0 => { // core 0 uses pin 17
            let pin = 17;
            GPFSEL1::Register.modify(GPFSEL1::FSEL17, 0x1); // set as output pin
            GPSET0::Register.set(1 << pin);
        },
        1 => { // core 1 uses pin 18
            let pin = 18;
            GPFSEL1::Register.modify(GPFSEL1::FSEL18, 0x1);
            GPSET0::Register.set(1 << pin);
        },
        2 => { // core 2 uses pin 20
            let pin = 20;
            GPFSEL2::Register.modify(GPFSEL2::FSEL20, 0x1);
            GPSET0::Register.set(1 << pin);
        },
        3 => { // core 3 uses pin 21
            let pin = 21;
            GPFSEL2::Register.modify(GPFSEL2::FSEL21, 0x1);
            GPSET0::Register.set(1 << pin);
        },
        _ => (), // nothing to do in case there is a core number higher than 3 running - RPi has only 4 cores ;)
    }
}

fn thinking(_: u32) -> ! {
    loop { }
}