#![no_main]
#![no_std]

use core::fmt::Write;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: *mut core::ffi::c_void, st: *mut core::ffi::c_void) -> usize {
    let system_table = unsafe { &mut *(st as *mut SystemTable) };
    let stdout = unsafe { &mut *system_table.stdout };
    let _ = writeln!(stdout, "Hello, World!");
    0
}

// define UEFI structures (simplified for clarity)
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub stdout: *mut SimpleTextOutputProtocol,
    // other members omitted for brevity
}

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(*mut SimpleTextOutputProtocol, bool) -> usize,
    pub output_string:
        unsafe extern "efiapi" fn(*mut SimpleTextOutputProtocol, *const u16) -> usize,
    // other members omitted for brevity
}

impl Write for SimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // use a fixed-size buffer to store UTF-16 encoded string
        const BUF_SIZE: usize = 1024;
        let mut utf16_buf = [0u16; BUF_SIZE];
        let mut i = 0;

        for utf16_char in s.encode_utf16() {
            if i >= BUF_SIZE - 1 {
                break; // prevent overflow
            }
            utf16_buf[i] = utf16_char;
            i += 1;
        }

        utf16_buf[i] = 0; // null-terminate the string

        // call UEFI OutputString
        unsafe {
            (self.output_string)(self, utf16_buf.as_ptr());
        }

        Ok(())
    }
}
