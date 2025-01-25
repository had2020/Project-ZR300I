use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the output ISO file
    let output_iso = "output.iso";
    let mut iso_file = File::create(output_iso)?;

    // write ISO9660 Primary Volume Descriptor
    iso_file.write_all(b"CD001")?; // magic number ISO9660
    iso_file.write_all(b"BOOTLOADER")?; // volume label

    // add EFI directory and BOOTAA64.EFI
    let efi_boot_data = include_bytes!("temp_build_files/BOOTAA64.EFI"); // current build file stored
    iso_file.write_all(efi_boot_data)?;

    println!("ISO created successfully: {}", output_iso);
    Ok(())
}
