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
brew install coreutils
brew install dosfstools
# Steps

# Assemble
aarch64-elf-as -o boot.o boot.s

# Link
aarch64-elf-ld -nostdlib --no-dynamic-linker --section-start=.text=0x100000 -o boot.efi boot.o

# Move and copy EFI file
mv boot.efi BOOTAA64.EFI
mkdir -p iso/EFI/BOOT/
cp BOOTAA64.EFI iso/EFI/BOOT/

# Create a 100MB image file (install coreutils if needed):
hdiutil create -size 100m -fs FAT32 -volname "BOOT" boot.img

# Attach (mount) the image:
hdiutil attach boot.img.dmg -mountpoint mounted/

# Create directories and copy files:
sudo mkdir -p mounted/EFI/BOOT
sudo cp BOOTAA64.EFI mounted/EFI/BOOT/

# Unmount the image:
diskutil unmount mounted/

# detach disk
hdiutil detach /dev/diskN - use N for disk number seen at hdiutil info

# Convert back to img
hdiutil convert boot.img.dmg -format UDTO -o boot.img

mv boot.img.cdr boot.img

# Testing
qemu-system-aarch64 -machine virt -cpu cortex-a72 -drive file=boot.img,format=raw -serial stdio
