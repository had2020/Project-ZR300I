#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // writing to console
    system_table.stdout().reset(false).unwrap_success();
    system_table
        .stdout()
        .write_str("Bootloader starting!\n")
        .unwrap_success();

    // return success status
    Status::SUCCESS
}
