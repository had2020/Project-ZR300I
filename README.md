# TODO DOCS


BOOTX64.EFI
/EFI/BOOT/BOOTX64.EFI

# building iso with xorriso

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
