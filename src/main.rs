#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustyvoidos_rvos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustyvoidos_rvos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    rustyvoidos_rvos::init();

    #[cfg(test)]
    test_main();

    rustyvoidos_rvos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustyvoidos_rvos::test_panic_handler(info)
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustyvoidos_rvos::hlt_loop();
}
