#![no_std] // Removes the rust std lib
#![no_main] // disable the entry point which rust calls

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World";


#[no_mangle] // tells that do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function will act as the entrypoint as linked looks for
    // function that `_start` when by default

    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
