mkdir -p temp_build_files
mkdir -p test/EFI/BOOT

cd kernal
mkdir test

cargo build --release --target aarch64-unknown-uefi -Z build-std

cp target/aarch64-unknown-uefi/release/kernal.efi ../temp_build_files/BOOTAA64.EFI

cd ../

cp temp_build_files/BOOTAA64.EFI test/EFI/BOOT/BOOTAA64.EFI

# create the ISO using tool
xorriso \
  -as mkisofs \
  -o output.iso \
  -V BOOTLOADER \
  -J \
  -R \
  -input-charset utf-8 \
  -eltorito-platform efi \
  -eltorito-boot EFI/BOOT/BOOTAA64.EFI \
  -no-emul-boot \
  -eltorito-catalog boot.catalog \
  test

# check ISO creation pass
if [ $? -eq 0 ]; then
  echo "ISO created successfully: ${OUTPUT_ISO}"
else
  echo "Failed to create ISO"
fi
