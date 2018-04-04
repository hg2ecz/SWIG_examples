// One string ... block transmute

use std::mem::transmute;
use std::ptr::read;
use cpython::{Python, PyResult};

pub fn source3(_py: Python, len: i32) -> PyResult<String> {
    let mut datastr = String::new();
    for i in 0..len {
	let ch: char = unsafe { transmute::<i32, char>(i) };
	datastr.push(ch);
    }
    Ok(datastr)
}

pub fn sink3(_py: Python, datastr: String) -> PyResult<i32>{
    let datavec: Vec<char> = datastr.chars().collect();                          // utf8 --> unicode32 ... slow

    if datavec.len() != 50000 {
	println!("This test work only fix buffersize (50.000), the bufferlen is: {}", datavec.len());
	return Ok(-1);
    }

    let ptr_i32_arr = datavec.as_ptr() as *const [i32; 50000];                   // fix buffer (50000)
    let array = unsafe { read( ptr_i32_arr.offset(0 as isize) ) };
    for i in 0..datavec.len() {
	let d = array[i];                                                         // d: i32
	if d != i as i32 { println!("Error: compare failed ({} <> {})", i, d); }  // check
    }
    Ok(0)
}
