#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn efi_main(image_handle: *mut c_void, system_table: *mut c_void) -> usize {
    // Process UEFI application logic
    0 // return success status
}
