use std::ffi::CString;
use std::ffi::CStr;

mod c_example;

fn example(s: &str) -> i32 {
    unsafe { c_example::Example(CString::new(s).unwrap().as_ptr()) }
}

fn get(fd: i32) -> String {
    unsafe { CStr::from_ptr( c_example::Get(fd) ).to_string_lossy().into_owned() }
}

fn main() {
    let a = example("teszt-A");
    let b = example("teszt-B");

    println!("{}", get(a));
    println!("{}", get(b));
    println!("{}", get(a));
    println!("{}", get(a));
    println!("{}", get(a));
    println!("{}", get(a));
    println!("{}", get(b));
}
