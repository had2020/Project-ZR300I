#![no_main]
#![no_std]

use core::panic::PanicInfo;
use log::info;
use uefi::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Bootloader loaded!"); // display
    boot::stall(10_000_000); // 10 secs
    Status::SUCCESS // return success
}
