use cfi_types::{c_int, c_long};
use std::mem;

#[link(name = "foo")]
extern "C" {
    fn do_twice(f: unsafe extern "C" fn(c_int) -> c_int, arg: i32) -> i32;
}

unsafe extern "C" fn add_one(x: c_int) -> c_int {
    c_int(x.0 + 1)
}

unsafe extern "C" fn add_two(x: c_long) -> c_long {
    c_long(x.0 + 2)
}

fn main() {
    let answer = unsafe { do_twice(add_one, 5) };

    println!("The answer is: {}", answer);

    println!("With CFI enabled, you should not see the next answer");
    let f: unsafe extern "C" fn(c_int) -> c_int = unsafe {
        mem::transmute::<*const u8, unsafe extern "C" fn(c_int) -> c_int>(add_two as *const u8)
    };
    let next_answer = unsafe { do_twice(f, 5) };

    println!("The next answer is: {}", next_answer);
}
