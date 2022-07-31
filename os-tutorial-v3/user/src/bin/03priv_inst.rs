#![no_std]
#![no_main]

extern crate user_lib;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    user_lib::logger::init();

    log::warn!("Try to execute privileged instruction in U Mode");
    log::warn!("Kernel should kill this application!");
    unsafe {
        asm!("sret");
    }
    0
}
