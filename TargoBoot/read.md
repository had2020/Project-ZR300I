
arm-none-eabi-as boot.s -o boot.o
arm-none-eabi-ld boot.o -o boot.elf
arm-none-eabi-objcopy -O binary boot.elf boot.bin

# Qemu
qemu-system-arm -M versatilepb -cpu cortex-a9 -nographic -kernel bootloader.bin

# /\ old | \/ Linux

aarch64-linux-gnu-as -o boot.o boot.s

aarch64-linux-gnu-ld -nostdlib --no-dynamic-linker --section-start=.text=0x100000 -o boot.efi boot.o

# -serial stdio
qemu-system-aarch64 -machine virt -cpu cortex-a57 -drive file=bootable.iso,format=raw -serial stdio
