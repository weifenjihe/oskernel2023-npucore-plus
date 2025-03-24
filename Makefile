PROJECT_DIR := $(shell pwd)
MODE := release

MUSL_TOOLCHAIN_PREFIX := riscv64-linux-musl
MUSL_TOOLCHAIN_DIR := $(PROJECT_DIR)/$(MUSL_TOOLCHAIN_PREFIX)-cross/bin
MUSL_CC := $(MUSL_TOOLCHAIN_PREFIX)-gcc
MUSL_AR := $(MUSL_TOOLCHAIN_PREFIX)-ar
MUSL_OBJCOPY := $(MUSL_TOOLCHAIN_PREFIX)-objcopy

BASH_DIR := $(PROJECT_DIR)/bash-5.1.16
BASH := $(BASH_DIR)/bash

USER_DIR := $(PROJECT_DIR)/user
INITPROC_SRC := $(USER_DIR)/src/bin/initproc.rs
INITPROC := $(USER_DIR)/target/riscv64gc-unknown-none-elf/$(MODE)/initproc
BOOTLOADER := bootloader/fw_jump.bin

OS_DIR := $(PROJECT_DIR)/os
KERNEL := $(OS_DIR)/target/riscv64gc-unknown-none-elf/$(MODE)/os

export PATH := $(PATH):$(MUSL_TOOLCHAIN_DIR)

all: $(KERNEL)

$(INITPROC): $(INITPROC_SRC)
	cd $(USER_DIR) && make

$(BASH):
	cd $(BASH_DIR) && make 
	$(MUSL_OBJCOPY) --strip-debug $(BASH)

$(KERNEL): $(INITPROC) $(BASH)
	cd $(OS_DIR) && make comp BOARD=qemu COMP=true MODE=$(MODE)

clean:
	cd $(OS_DIR) && make clean
	cd $(USER_DIR) && make clean

run: all
	qemu-system-riscv64 -machine virt -kernel kernel-qemu -m 128M -nographic -smp 2 -bios sbi-qemu -drive file=sdcard.img,if=none,format=raw,id=x0  -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -device virtio-net-device,netdev=net -netdev user,id=net

dbg: 
	make all MODE=debug
	qemu-system-riscv64 -machine virt -kernel kernel-qemu -m 128M -nographic -smp 2 -bios sbi-qemu -drive file=sdcard.img,if=none,format=raw,id=x0  -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -device virtio-net-device,netdev=net -netdev user,id=net -S -s