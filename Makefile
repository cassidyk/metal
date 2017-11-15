arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

target ?= $(arch)-metal_os
rust_os := target/$(target)/release/libmetal_os.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
asm_src := $(wildcard src/arch/$(arch)/*.asm)
asm_obj := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(asm_src))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-$(arch).exe -cdrom $(iso) &

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(asm_obj) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(asm_obj) $(rust_os)

kernel:
	@xargo build --release --target $(target)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@