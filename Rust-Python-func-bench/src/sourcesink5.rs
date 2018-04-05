// One string ... no unicode ... 6 bit/char and direct mapping

use std::ptr;
use cpython::{Python, PyResult};

pub fn source5(_py: Python, len: i32) -> PyResult<String> {
    //let datastr = String::with_capacity(5*len as usize);
    let datastr: String = String::from_utf8(vec![b'X'; 5*len as usize]).unwrap();
    let ptr_u8 = datastr.as_ptr() as *mut u8;

    for i in 0..len {
	let d = i as u32;   // if need other conversion:  d = transmute::<f32, u32>(d_orig);
	unsafe {
	    ptr::write(ptr_u8.offset((5*i + 0) as isize), (d >> (35-7)) as u8);
	    ptr::write(ptr_u8.offset((5*i + 1) as isize), ((d >> (35-7-7)) & 0x7f) as u8);
	    ptr::write(ptr_u8.offset((5*i + 2) as isize), ((d >> (35-7-7-7)) & 0x7f) as u8);
	    ptr::write(ptr_u8.offset((5*i + 3) as isize), ((d >> (35-7-7-7-7)) & 0x7f) as u8);
	    ptr::write(ptr_u8.offset((5*i + 4) as isize), (d & 0x7f) as u8);
        }
    }
    Ok(datastr)
}

pub fn sink5(_py: Python, datastr: String) -> PyResult<i32>{
    let mut i = 0u32;
    let mut idx = 0usize;
    let ptr_u8 = datastr.as_ptr() as *const u8;

    let mut seven = [0u32; 5];
    for stridx in 0..datastr.len() {
	seven[idx as usize] = unsafe { ptr::read( ptr_u8.offset(stridx as isize) ) } as u32;

	if idx >= 4 {
	    let d: u32 = (seven[0] << (35-7)) | (seven[1] << (35-7-7)) | (seven[2] << (35-7-7-7)) | (seven[3] << (35-7-7-7-7)) | seven[4];
	    if d != i as u32 { println!("Error: compare failed ({} <> {})", i, d); }  // check
	    idx=0; i+=1;
	} else {
	    idx+=1;
	}
    }
    Ok(0)
}