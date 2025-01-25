use iso9660::{iso, IsoBuilder, IsoWriter};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // define the output ISO file
    let output_iso = "output.iso";

    // create a new ISO builder
    let mut builder = IsoBuilder::new();

    // add the EFI directory and boot file
    builder.add_file(
        "EFI/BOOT/BOOTAA64.EFI", // BOOTAA64.EFI or BOOTX64.EFI
        include_bytes!("build_files../../files/BOOTX64.EFI").as_ref(),
    )?;

    /* TODO kernal
    // add the OS directory and kernel binary
    builder.add_file(
        "OS/kernel.bin",
        include_bytes!("build_files../../files/kernel.bin").as_ref(),
    )?;
    */

    // finalize and write the ISO
    let mut iso_writer = fs::File::create(output_iso)?;
    builder.write_to(&mut iso_writer)?;

    println!("ISO file created successfully at {}", output_iso);
    Ok(())
}
