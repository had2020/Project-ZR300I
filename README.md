# TODO DOCS


BOOTX64.EFI
/EFI/BOOT/BOOTX64.EFI

# building .efi bootloader

cargo build --target aarch64-unknown-uefi

# building iso with xorriso FAILS

xorriso -as mkisofs \
    -o bootable.iso \
    -iso-level 3 \
    -J -R \
    -eltorito-alt-boot \
    -no-emul-boot \
    -b EFI/BOOT/BOOTX64.EFI \
    gen_files/


# testing with qemu

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a57 \
  -m 1024 \
  -cdrom bootable.iso

# building with iso with stuff FAILS
mkfs.fat -C uefi.img 2880
mcopy -s gen_files/* :: -i uefi.img
