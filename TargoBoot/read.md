arm-none-eabi-as bootloader.s -o bootloader.o
arm-none-eabi-ld bootloader.o -o bootloader.elf
arm-none-eabi-objcopy -O binary bootloader.elf bootloader.bin

# Qemu
qemu-system-arm -M versatilepb -cpu cortex-a9 -nographic -kernel bootloader.bin
