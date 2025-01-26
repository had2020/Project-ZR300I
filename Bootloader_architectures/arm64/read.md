as bootloader.s -o bootloader.o
ld bootloader.o

# other
arm-none-eabi-as -o bootloader.o bootloader.s
ld -T linker.ld -o bootloader.efi bootloader.o --subsystem=10 --entry=efi_main

aarch64-linux-gnu-as -o bootloader.o bootloader.s
aarch64-linux-gnu-ld -T linker.ld -o bootloader.efi bootloader.o --entry=efi_main
