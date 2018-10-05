use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name="example", kind="static")]
extern {
    fn Example(s: *const c_char) -> i32;
    fn Get(i: i32) -> *const c_char;
}

fn example(s: &str) -> i32 {
    unsafe { Example(CString::new(s).unwrap().as_ptr()) }
}

fn get(fd: i32) -> String {
    unsafe { CStr::from_ptr( Get(fd) ).to_string_lossy().into_owned() }
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
