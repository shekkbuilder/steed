#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::time::SystemTime;

#[no_mangle]
pub fn main() -> i32 {
    let now = SystemTime::now();

    println!("{:?}", now);
    0
}
