#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(stegosauros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use stegosauros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "1.234");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nError: {}\n", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    stegosauros::test_panic_handler(info)
}