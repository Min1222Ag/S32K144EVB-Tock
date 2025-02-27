# Licensed under the Apache License, Version 2.0 or the MIT License.
# SPDX-License-Identifier: Apache-2.0 OR MIT

include ../Makefile.common

TOCKLOADER=tockloader
JLINK=JLinkExe
JLINK_SCRIPT=jlink_script.jlink

TARGET=thumbv7em-none-eabihf
PLATFORM ?= s32k144evb
TOOLCHAIN ?= arm-none-eabi
OBJCOPY ?= arm-none-eabi-objcopy

# Default target for installing the kernel.
.PHONY: install
install: flash

# Build configurations
.PHONY: flash-debug
flash-debug: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
	$(JLINK) -CommanderScript $(JLINK_SCRIPT)

.PHONY: flash
flash: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	$(Q)$(CARGO) flash --chip S32K144 --verify --path $<


.PHONY: program
program: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).bin
	$(TOCKLOADER) flash $< --address 0x00000000 --page-size 512

.PHONY: flash-app
flash-app:
	$(error Use tockloader to load applications)

.PHONY: clean
clean:
	rm -rf $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
	rm -rf $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).bin
