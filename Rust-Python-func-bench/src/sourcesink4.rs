// One string ... no unicode ... 7 bit/char

use std::mem::transmute;
use cpython::{Python, PyResult};

pub fn source4(_py: Python, len: i32) -> PyResult<String> {
    let mut datastr = String::new();

    for i in 0..len {
	let d = i as u32;   // if need other conversion:  d = transmute::<f32, u32>(d_orig);
	unsafe {
	    datastr.push(transmute::<u32, char>(d >> (35-7)));
	    datastr.push(transmute::<u32, char>((d >> (35-7-7)) & 0x7f));
	    datastr.push(transmute::<u32, char>((d >> (35-7-7-7)) & 0x7f));
	    datastr.push(transmute::<u32, char>((d >> (35-7-7-7-7)) & 0x7f));
	    datastr.push(transmute::<u32, char>(d & 0x7f));
	}
    }
    Ok(datastr)
}

pub fn sink4(_py: Python, datastr: String) -> PyResult<i32>{
    let datavec: Vec<char> = datastr.chars().collect();                // utf8 --> unicode32 ... slow

    for i in 0..datastr.len()/5 {
	let stridx = i*5;
	let s0 = unsafe { transmute::<char, u32>(datavec[stridx+0]) }; // 4 bit
	let s1 = unsafe { transmute::<char, u32>(datavec[stridx+1]) }; // 7 bit
	let s2 = unsafe { transmute::<char, u32>(datavec[stridx+2]) }; // 7 bit
	let s3 = unsafe { transmute::<char, u32>(datavec[stridx+3]) }; // 7 bit
	let s4 = unsafe { transmute::<char, u32>(datavec[stridx+4]) }; // 7 bit
	let d: u32 = (s0 << (35-7)) | (s1 << (35-7-7)) | (s2 << (35-7-7-7)) | (s3 << (35-7-7-7-7)) | s4;

	if d != i as u32 { println!("Error: compare failed ({} <> {})", i, d); }  // check
    }
    Ok(0)
}
