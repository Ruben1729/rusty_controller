#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rusty_controller::test_runner)]

use rusty_controller::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
