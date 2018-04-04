// One string ... no unicode ... 6 bit/char

use std::mem::transmute;
use cpython::{Python, PyResult};

pub fn source4(_py: Python, len: i32) -> PyResult<String> {
    let mut datastr = String::new();

    for i in 0..len {
	let mut seven = [0u32; 5];
	seven[0] = i as u32 >> (35-7);
	seven[1] = (i as u32 >> (35-7-7)) & 0x7f;
	seven[2] = (i as u32 >> (35-7-7-7)) & 0x7f;
	seven[3] = (i as u32 >> (35-7-7-7-7)) & 0x7f;
	seven[4] = i as u32 & 0x7f;

	for j in 0..5 {
	    let ch: char = unsafe { transmute::<u32, char>(seven[j as usize]) };
	    datastr.push(ch);
	}
    }
    Ok(datastr)
}

pub fn sink4(_py: Python, datastr: String) -> PyResult<i32>{
    let mut i = 0u32;
    let mut idx = 0usize;
    let datavec: Vec<char> = datastr.chars().collect();                    // utf8 --> unicode32 ... slow

    let mut seven = [0u32; 5];
    for ch in datavec {
	seven[idx as usize] = unsafe { transmute::<char, u32>(ch) };                // d: i32
	if idx >= 4 {
	    let d: u32 = (seven[0] << (35-7)) | (seven[1] << (35-7-7)) | (seven[2] << (35-7-7-7)) | (seven[3] << (35-7-7-7-7)) | seven[4];
	    if d != i { println!("Error: compare failed ({} <> {})", i, d); }  // check
	    idx=0; i+=1;
	} else {
	    idx+=1;
	}
    }
    Ok(0)
}
