## LINUX ONLY

aarch64-linux-gnu-as -o boot.o boot.s
aarch64-linux-gnu-ld -nostdlib --no-dynamic-linker --section-start=.text=0x100000 -o boot.efi boot.o

mv boot.efi BOOTAA64.EFI

cp BOOTAA64.EFI iso/EFI/BOOT/

truncate -s 100MB boot.img

mkfs.vfat boot.img

sudo mount boot.img mounted/

sudo mkdir -p mounted/EFI/BOOT
sudo cp BOOTAA64.EFI mounted/EFI/BOOT/

sudo umount mounted/

#qemu-system-aarch64 -machine virt -cpu cortex-a57 -drive file=boot.img,format=raw -nographic
qemu-system-aarch64 -machine virt -cpu cortex-a57 -drive file=bootable.iso,format=raw -serial stdio

## MACOS
# Install
brew install aarch64-elf-gcc

# Steps

# Assemble
aarch64-elf-as -o boot.o boot.s

# Link
aarch64-elf-ld -nostdlib --no-dynamic-linker --section-start=.text=0x100000 -o boot.efi boot.o
