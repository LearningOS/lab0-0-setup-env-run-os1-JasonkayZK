#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use log::info;
use user_lib::yield_;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[no_mangle]
fn main() -> i32 {

    user_lib::logger::init();

    for i in 0..HEIGHT {
        for _ in 0..WIDTH {
            print!("B");
        }
        println!(" [{}/{}]", i + 1, HEIGHT);
        yield_();
    }
    info!("Test write_b OK!");
    0
}
