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

hdiutil create -size 10m -fs fat32 -volname EFI esp.dmg
https://rust-osdev.github.io/uefi-rs/tutorial/hardware.html

# 2
xorriso -as mkisofs \
    -o output.iso \
    -b EFI/BOOT/bootaa64.efi \
    -no-emul-boot \
    -efi-boot-part \
    -efi-boot-image \
    esp

# 3

xorriso -as mkisofs \
  -isohybrid-mbr /usr/lib/syslinux/mbr/isohdpfx.bin \
  -c isolinux/boot.cat \
  -b isolinux/isolinux.bin \
  -no-emul-boot \
  -boot-load-size 4 \
  -boot-info-table \
  -eltorito-alt-boot \
  -e boot/grub/efi.img \
  -no-emul-boot \
  -isohybrid-gpt-basdat \
  -o /path/to/tmp.iso \
  /path/to/tmp

# 4

xorriso -as mkisofs \
  -e EFI/BOOT/BOOTAA64.EFI \
  -no-emul-boot \
  -isohybrid-gpt-basdat \
  -o output.iso \
  iso_root

# 4.1

xorriso -as mkisofs \
  -iso-level 3 \
  -volid "UEFI_APP" \
  -efi-boot-part --efi-boot-image \
  --no-emul-boot \
  -efi-boot iso_root/EFI/BOOT/BOOTAA64.EFI \
  -output output.iso \
  iso_root/

# check

isoinfo -l -i output.iso

# 5
genisoimage -o output.iso \
  -efi-boot esp.img \
  -no-emul-boot \
  -volid "UEFI_APP" \
  -iso-level 3 \
  iso_root/

# 6
hdiutil makehybrid -o output.iso iso -iso -joliet -eltorito-boot iso/EFI/BOOT/BOOTAA64.EFI -no-emul-boot

# 7 on linux
xorriso -as mkisofs \
  -R -J -c boot.catalog \
  -o custom-uefi.iso \
  -efi-boot-part --efi-boot-image \
  -no-emul-boot -isohybrid-gpt-basdat \
  iso/

# testing
- display
qemu-system-aarch64 -machine virt -cpu cortex-a57 \
  -m 1024 -bios /usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
  -cdrom custom-uefi.iso

- no display
qemu-system-aarch64 -machine virt -cpu cortex-a57 \
  -m 1024 -bios /usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
  -cdrom custom-uefi.iso -nographic
