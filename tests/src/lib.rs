#![allow(non_upper_case_globals)]
extern crate libc;
use libc::{c_char, c_int};

//FUNCTIONS
#[no_mangle]
pub fn rust_fun_print_something() {
    println!("something");
}

#[no_mangle]
pub fn rust_fun_add_one_i32(arg: i32) -> i32 {
    arg + 1
}

#[no_mangle]
pub fn rust_fun_add_one_i64(arg: i64) -> i64 {
    arg + 1
}

#[no_mangle]
pub fn rust_fun_add_one_f32(arg: f32) -> f32 {
    arg + 1.0
}

#[no_mangle]
pub fn rust_fun_add_one_f64(arg: f64) -> f64 {
    arg + 1.0
}

#[no_mangle]
pub fn rust_fun_add_all_12_i32(
    a1: i32,
    a2: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i32,
    a7: i32,
    a8: i32,
    a9: i32,
    a10: i32,
    a11: i32,
    a12: i32,
) -> i32 {
    a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12
}

#[no_mangle]
pub extern "C" fn c_fun_print_something_else() {
    println!("something else");
}

#[no_mangle]
pub extern "C" fn c_fun_add_two(arg: c_int) -> c_int {
    arg + 2
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn c_fun_variadic(txt: *const c_char) {
    //pretend to be variadic - impossible to do in Rust code
}

//STATIC DATA
#[no_mangle]
pub static mut rust_i32_mut: i32 = 42;
#[no_mangle]
pub static rust_i32: i32 = 43;

#[no_mangle]
pub static mut c_int_mut: c_int = 44;
#[no_mangle]
pub static c_int: c_int = 45;

#[repr(C)]
pub struct SomeData {
    first: c_int,
    second: c_int,
}

#[no_mangle]
pub static c_struct: SomeData = SomeData {
    first: 1,
    second: 2,
};

//STATIC STRINGS

//exporting str directly is not so easy - it is not Sized!
//you can only export a reference to str and this requires double dereference
#[no_mangle]
pub static rust_str: &str = "Hello!";

#[no_mangle]
pub static c_const_char_ptr: [u8; 4] = [b'H', b'i', b'!', 0];
