#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustyvoidos_rvos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
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
    use rustyvoidos_rvos::println;

    println!("{}", info);
    rustyvoidos_rvos::hlt_loop();
}
