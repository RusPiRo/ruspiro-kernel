#*****************************************************************
# Makefile to build the RusPiRo kernel
# setting the necessary environment for cross compiling
#
# Copyright (c) 2019 by the authors
# 
# Author: André Borrmann 
# License: Apache License 2.0
#******************************************************************
CARGO_BIN = "$(USER_ROOT)\\.cargo\\bin"
ARM_GCC_BIN = "$(USER_ROOT)\\arm-gcc\\gcc-arm-eabi\\bin"
ARM_GCC_LIB = "$(USER_ROOT)\\arm-gcc\\gcc-arm-eabi\\lib\\gcc\\arm-eabi\\8.3.0"
PATH +=  "$(PROJECT_ROOT);$(ARM_GCC_BIN);$(ARM_GCC_LIB);$(CARGO_BIN)"
TARGET = armv7-unknown-linux-gnueabihf
TARGETDIR = target\\armv7-unknown-linux-gnueabihf\\release
# environment variables needed by cargo xbuild
export CC = arm-eabi-gcc.exe
export AR = arm-eabi-ar.exe
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = arm-eabi-gcc.exe
export CFLAGS = -mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53

# specific build step to concatenate the binarry into a file that contains marker the custom bootloader running an 
# my raspberry Pi is looking for to deploy new build version without via UART without the need to toggle the SD card each
# time.
deploy: all
#	copy /B deadbeef.txt+"$(TARGETDIR)\\$<"+deadbeef.txt bootloader.img
	cat deadbeef.txt "./$(TARGETDIR)/kernel7.img" deadbeef.txt > bootloader.img

# build the kernel7.img file
all: kernel7	
	cargo objcopy -- -O binary $(TARGETDIR)\\$< $(TARGETDIR)\\kernel7.img

# cross compile using cargo xbuild
kernel7: 
# update dependend crates to their latest version
	cargo update
# cross compile the kernel
	cargo xbuild --target $(TARGET) --release --all

doc:
	# update dependend crates to their latest version
	cargo update
	# build docu
	xargo doc --all --no-deps --target $(TARGET) --release --open
	
test:
	xargo test --doc --target $(TARGET)

clean:
	cargo clean