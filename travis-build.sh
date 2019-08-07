#!/bin/bash
set -ev

export CFLAGS='-mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53'
export RUSTFLAGS='-C linker=arm-linux-gnueabihf-gcc -C target-cpu=cortex-a53 -C target-feature=+a53,+fp-armv8,+v8,+vfp3,+d16,+thumb2,+neon -C link-arg=-nostartfiles -C link-arg=-T./ruspiro-boot/link.ld -C opt-level=3 -C debuginfo=0'

cargo xbuild --target armv7-unknown-linux-gnueabihf --verbose --release --all