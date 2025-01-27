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

# TODO make bootable iso

cargo build --target aarch64-unknown-uefi

https://rust-osdev.github.io/uefi-rs/tutorial/vm.html

cp target/aarch64-unknown-uefi/debug/boot.efi esp/efi/boot/bootaa64.efi

# run

qemu-system-aarch64-unknown-uefi -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp

# trying
