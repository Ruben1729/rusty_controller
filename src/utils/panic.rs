use core::panic::PanicInfo;
use crate::utils::qemu::*;
use crate::rc_println;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rc_println!("Error: {}\n", info);
    //   HANDLE PANIC HERE   //

    // END OF PANIC SECTION  //
    exit_qemu(QemuExitCode::Failed);
    loop {}
}