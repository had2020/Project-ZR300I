# local testing
nasm boot.asm
qemu-system-i386 boot

# building
nasm -f bin boot.asm -o boot.bin

dd if=/dev/zero of=boot.img bs=512 count=2880  # create an empty 1.44MB image
dd if=boot.bin of=boot.img conv=notrunc       # write the boot sector to the image

- testing before flashing
qemu-system-x86_64 -fda boot.img
