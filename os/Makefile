# version 1.0

LOG ?= info

qemu := qemu-system-riscv64
qemu-opts :=  -machine virt \
			-nographic \
			-bios ../bootloader/rustsbi-qemu.bin \
			-device loader,file=target/riscv64gc-unknown-none-elf/release/os,addr=0x80200000

export log = $(LOG)

run: build
	@$(qemu) $(qemu-opts)

build:
	@cargo build --release

clean:
	@cargo clean
