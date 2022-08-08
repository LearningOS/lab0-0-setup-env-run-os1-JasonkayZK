#![no_std]
#![no_main]

extern crate user_lib;

use log::info;
use user_lib::{print, println, yield_};

const WIDTH: usize = 10;
const HEIGHT: usize = 5;

#[no_mangle]
fn main() -> i32 {

    user_lib::logger::init();

    for i in 0..HEIGHT {
        for _ in 0..WIDTH {
            print!("A");
        }
        println!(" [{}/{}]", i + 1, HEIGHT);
        yield_();
    }
    info!("Test write_a OK!");
    0
}
