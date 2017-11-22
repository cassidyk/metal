arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

target ?= $(arch)-euclid
rust_os := target/$(target)/release/libmetal_os.a

linker_script := arch/$(arch)/linker.ld
grub_cfg := arch/$(arch)/grub.cfg
asm_src := $(wildcard arch/$(arch)/*.asm)
asm_obj := $(patsubst arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(asm_src))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build
	@cargo clean

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
build/arch/$(arch)/%.o: arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@