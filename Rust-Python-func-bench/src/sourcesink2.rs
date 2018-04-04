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
    let charvec:Vec<char> = datastr.chars().collect();
    let charveclen = charvec.len();
    for ch in charvec {
	let dnum: i32 = unsafe { transmute::<char, i32>(ch) };
	if dnum != i {
	    println!("Az {}. adat eltér: {}, adathossz: {}", i, dnum, charveclen);
	}
	i+=1;
    }
    Ok(0)
}
