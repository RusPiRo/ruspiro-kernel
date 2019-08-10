# RusPiRo Kernel (32Bit)

This crate aims to be THE RusPiRo kernel somewhen in the future. At the time beeing it's used to verify the usage 
and proper function of the different RusPiRo feature crates. The kernel is build completely with ``#![no_std]`` and ``#![no_main]`` option.
 
# Dependencies
 
To succsessfully build this crate for Raspberry Pi use ``xargo build`` with ``--target armv7-unknown-linux-gnueabihf``.
Check out the [README.md](https://github.com/RusPiRo/ruspiro-kernel/blob/master/README.md) for further details.

This crate compiles into a binary image file that could be put onto the SD card of a Raspberry Pi to run in baremetal mode. 

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-kernel.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-kernel)


## Pre-requisites

The setup to build the kernel from this repository assumes cross-compiling from a Windows machine to Raspberry Pi ARM.
Therefore several pre-requisits should be considered for a succsessful build.

### Cross-Compiler toolchain

The latest official ARM toolchain is used for cross compilation and installed somewhere on the machine. The toolchain can
be donwloaded here: https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads .
Choose the version: "i686-mingw32 hosetd: AArch32 bare-metal target (arm-eabi))".

In addition to the toolchain, mingw-make should be installed to execute the makefile that is responsible for the whole build
process.

### Rust

The latest rust nightly build should be installed as well. Nightly is needed as not all features are currently supported in the stable build as of the time writing this. Based on the host system rust will be installed the corresponding build and build targets will be installed. Giving Windows as host machine "nightly-x86_64-pc-windows-gnu" would be the right choice for this. In addition the cross compile target need to be installed as well as ``cargo-xbuild`` and the ``rust-src`` component. Use the following steps in the CLI:
```
> rustup target add armv7-unknown-linux-gnueabihf
> rustup component add rust-src
> cargo install cargo-xbuild
```

#### Using RLS (Rust Language Server)
When using the RLS (Rust Language Server) extension in Visual Studio Code the following settings in the ``settings.json``
will ensure it's properly working with the target specification we use for our usual build.

```
"rust.all_targets": false,
"rust.build_on_save": true,
"rust.target": "armv7-unknown-linux-gnueabihf"
```

## Building

As the build of the binarry requires a linker script to be present it should be downloaded from the ``ruspiro-boot`` crate.
[``linker.ld``](https://github.com/RusPiRo/ruspiro-boot/blob/master/link.ld). The path to this file should be stored inside the
``makefile`` variable ``LINKERSCRIPT``. The file also contains variables that store the path to the cross compiler toolchain. The default values are like following:
```
# makefile
[...]
CARGO_BIN = "$(USER_ROOT)\\.cargo\\bin"
ARM_GCC_BIN = "$(USER_ROOT)\\arm-gcc\\gcc-arm-eabi\\bin"
ARM_GCC_LIB = "$(USER_ROOT)\\arm-gcc\\gcc-arm-eabi\\lib\\gcc\\arm-eabi\\8.3.0"
LINKERSCRIPT = ../ruspiro-boot/link.ld
[...]
```

To build the actual binary just call
```
> make all
```
from the root directory of your project. This will create the ``kernel7.img`` in the folder ``[path to project]/target/armv7-unknown-linux-gnueabihf/release``

## Deploy to Raspberry Pi

To deploy the baremetal kernel to the Rapsberry Pi, just copy the created ``kernel7.img`` file onto the root of a FAT32 formated SD card. This SD card should also contain the files ``bootcode.bin``, ``start.elf`` and ``fixup.dat`` that could be downloaded from the official Raspberry Pi [firmware github](https://github.com/raspberrypi/firmware/tree/master/boot). In case you plan to use additional features on the Rasperry Pi in the future (e.g. GPU, Bluetooth etc.) I would recommend to use the "_x" versions of the files when available. So choose ``bootcode.bin``, ``start_x.elf`` and ``fixup_x.dat``.

## Starting your own RusPiRo based kernel ?
Check out the ``ruspiro-sdk`` crate at either [crates.io](https://crates.io/crates/ruspiro-skd) or the [github repo](https://github.com/RusPiRo/ruspiro-sdk)

## License
This crate is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)