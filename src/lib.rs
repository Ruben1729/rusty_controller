#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

mod utils;
pub use self::utils::*;

mod controller;
pub use self::controller::*;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
