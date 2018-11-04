#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate lazy_static;
extern crate bootloader_precompiled;
extern crate spin;
extern crate uart_16550;
extern crate volatile;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    serial_println!("Hello Host{}", "!");

    loop {}
}
