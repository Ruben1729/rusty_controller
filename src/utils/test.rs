use crate::utils::qemu::{exit_qemu, QemuExitCode};
use crate::{rc_println, serial_print};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        rc_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    rc_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
}
