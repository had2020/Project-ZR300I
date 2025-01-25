cd iso_builder/src
mkdir temp_build_files
cd ../..
cd Bootloader_architectures/arm64
cd target

FOLDER_NAME="aarch64-unknoen-uefi"
OUTPUT_FILE="BOOTAA64.EFI"
tar -cvf "${OUTPUT_FILE}" "${FOLDER_NAME}"
mv BOOTAA64.EFI ../../../iso_builder/src/temp_build_files # TODO removal after completion
cd ../../../iso_builder
cargo run
