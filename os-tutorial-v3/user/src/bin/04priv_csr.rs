#![no_std]
#![no_main]

extern crate user_lib;

use riscv::register::sstatus;
use riscv::register::sstatus::SPP;

#[no_mangle]
fn main() -> i32 {
    user_lib::logger::init();

    log::warn!("Try to access privileged CSR in U Mode");
    log::warn!("Kernel should kill this application!");
    unsafe {
        sstatus::set_spp(SPP::User)
    }

    0
}
