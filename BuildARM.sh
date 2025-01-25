mkdir build_files
cd Bootloader_architectures/arm64
cd target

FOLDER_NAME="aarch64-unknoen-uefi"
OUTPUT_FILE="BOOTAA64.EFI"
tar -cvf "${OUTPUT_FILE}" "${FOLDER_NAME}"
mv BOOTAA64.EFI ../../../build_files
cd ../../../iso_builder
cargo run
