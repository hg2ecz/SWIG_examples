// One string

use std::mem::transmute;
use cpython::{Python, PyResult};

pub fn source2(_py: Python, len: i32) -> PyResult<String> {
    let mut datastr = String::new();
    for i in 0..len {
	let ch: char = unsafe { transmute::<i32, char>(i) };
	datastr.push(ch);
    }
    Ok(datastr)
}

pub fn sink2(_py: Python, datastr: String) -> PyResult<i32>{
    let mut i = 0i32;
    for ch in datastr.chars().collect::<Vec<char>>() {
	let d: i32 = unsafe { transmute::<char, i32>(ch) };                // d: i32
	if d != i { println!("Error: compare failed ({} <> {})", i, d); }  // check
	i+=1;
    }
    Ok(0)
}
