
arm-none-eabi-as boot.s -o boot.o
arm-none-eabi-ld boot.o -o boot.elf
arm-none-eabi-objcopy -O binary boot.elf boot.bin

# Qemu
qemu-system-arm -M versatilepb -cpu cortex-a9 -nographic -kernel bootloader.bin

# /\ old | \/ Linux

aarch64-linux-gnu-as -o boot.o boot.s

aarch64-linux-gnu-ld -nostdlib --no-dynamic-linker --section-start=.text=0x100000 -o boot.efi boot.o

# Macos FAILS, use Linux
``` bash
brew install llvm
```

# Assemble boot.s into boot.o
llvm-mc -triple=aarch64-unknown-none -filetype=obj boot.s -o boot.o

# Link boot.o to produce a bootable EFI executable
ld.lld -nostdlib --no-dynamic-linker --section-start=.text=0x100000 -o boot.efi boot.o
