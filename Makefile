arch ?= x86_64
kernel := build/kernel-$(arch).bin
vm_iso := percos-$(arch).iso
iso := build/$(vm_iso)
target ?= $(arch)-PercOS
rust_os := target/$(target)/debug/libpercos.a

qemu := shell_utils/qemu_percos_osx/Programs/qemu-system-x86_64

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso xargo

all: $(kernel)

clean:
	@rm -r build

up:
	@vagrant up

halt:
	@vagrant halt

run: $(iso)
	@$(qemu) -k es -cdrom $(iso) -s

debug: $(iso)
	@$(qemu) -k es -cdrom $(iso) -s -S

lldb:
	@make run &
	@./wait_for_qemu.sh
	@rust-lldb -s "connect.lldb"

lldb-d:
	@make debug &
	@./wait_for_qemu.sh
	@rust-lldb -s "connect.lldb"

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@vagrant scp build/isofiles :/vagrant/isofiles > /dev/null 2> /dev/null
	@vagrant scp $(grub_cfg) :/vagrant/grub.cfg > /dev/null 2> /dev/null
	@vagrant ssh -- 'grub-mkrescue -o /vagrant/$(vm_iso) /vagrant/isofiles 2> /dev/null'
	@vagrant scp :/vagrant/$(vm_iso) $(iso) > /dev/null 2> /dev/null
	@vagrant ssh -- 'rm -r /vagrant/*'
	@rm -r build/isofiles

$(kernel): xargo $(rust_os) $(assembly_object_files) $(linker_script)
	@ld.lld --gc-sections -T $(linker_script) -o $(kernel) \
	    $(assembly_object_files) $(rust_os)

xargo:
  @RUST_TARGET_PATH=$(shell pwd) xargo build --target=$(target)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@