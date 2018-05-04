use std::ffi::CString;
use std::ffi::CStr;

mod cpp_example;

fn example(s: &str) -> cpp_example::Example {
    unsafe { cpp_example::Example::new(CString::new(s).unwrap().as_ptr()) }
}

fn get(obj: &mut cpp_example::Example) -> String {
    unsafe { CStr::from_ptr( obj.Get() ).to_string_lossy().into_owned() }
}

fn main() {
    let mut a = example("teszt-A");
    let mut b = example("teszt-B");

    println!("{}", get(&mut b));
    println!("{}", get(&mut a));
    println!("{}", get(&mut a));
    println!("{}", get(&mut a));
    println!("{}", get(&mut a));
    println!("{}", get(&mut a));
    println!("{}", get(&mut b));
}
