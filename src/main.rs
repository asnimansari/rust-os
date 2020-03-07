#![no_std] // Removes the rust std lib
#![no_main] // disable the entry point which rust calls

use core::panic::PanicInfo;


#[no_mangle] // tells that do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function will act as the entrypoint as linked looks for
    // function that `_start` when by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
