// One string ... no unicode ... 7 bit/char and direct mapping

use std::ptr;
use cpython::{Python, PyResult};

pub fn source6(_py: Python, len: i32) -> PyResult<String> {
    //let datastr = String::with_capacity(5*len as usize);
    let datastr: String = String::from_utf8(vec![b'X'; 5*len as usize]).unwrap();
    let ptr_u8 = datastr.as_ptr() as *mut u8;

    for i in 0..len {
	let d = i as u32;   // if need other conversion:  d = transmute::<f32, u32>(d_orig);
	unsafe {
	    let idx: isize = 5*i as isize;
	    ptr::write(ptr_u8.offset(idx + 0), (d >> (35-7)) as u8);
	    ptr::write(ptr_u8.offset(idx + 1), ((d >> (35-7-7)) & 0x7f) as u8);
	    ptr::write(ptr_u8.offset(idx + 2), ((d >> (35-7-7-7)) & 0x7f) as u8);
	    ptr::write(ptr_u8.offset(idx + 3), ((d >> (35-7-7-7-7)) & 0x7f) as u8);
	    ptr::write(ptr_u8.offset(idx + 4), (d & 0x7f) as u8);
        }
    }
    Ok(datastr)
}

pub fn sink6(_py: Python, datastr: String) -> PyResult<i32>{
    let ptr_u8 = datastr.as_ptr() as *const u8;

    for i in 0..datastr.len()/5 {
	let stridx: isize = 5*i as isize;
	let s0 = unsafe { ptr::read( ptr_u8.offset(stridx+0) ) } as u32;
	let s1 = unsafe { ptr::read( ptr_u8.offset(stridx+1) ) } as u32;
	let s2 = unsafe { ptr::read( ptr_u8.offset(stridx+2) ) } as u32;
	let s3 = unsafe { ptr::read( ptr_u8.offset(stridx+3) ) } as u32;
	let s4 = unsafe { ptr::read( ptr_u8.offset(stridx+4) ) } as u32;
	let d: u32 = (s0 << (35-7)) | (s1 << (35-7-7)) | (s2 << (35-7-7-7)) | (s3 << (35-7-7-7-7)) | s4;

	if d != i as u32 { println!("Error: compare failed ({} <> {})", i, d); }  // check
    }
    Ok(0)
}
