// One string ... no unicode ... 7 bit/char ... bytevector

use cpython::{Python, PyResult};

pub fn source5(_py: Python, len: i32) -> PyResult<String> {
    let mut datavec: Vec<u8> = Vec::new();

    for i in 0..len {
	let d = i as u32;   // if need other conversion:  d = transmute::<f32, u32>(d_orig);
	datavec.push( (d >> (35-7)) as u8);
	datavec.push( ((d >> (35-7-7)) & 0x7f) as u8);
	datavec.push( ((d >> (35-7-7-7)) & 0x7f) as u8);
	datavec.push( ((d >> (35-7-7-7-7)) & 0x7f) as u8);
	datavec.push( (d & 0x7f) as u8);
    }

    let datastr = String::from_utf8(datavec).unwrap();
    Ok(datastr)
}

pub fn sink5(_py: Python, datastr: String) -> PyResult<i32>{
    let datavec = datastr.as_bytes();

    for i in 0..datavec.len()/5 {
	let stridx = i*5;
	let s0 = datavec[stridx+0] as u32; // 4 bit
	let s1 = datavec[stridx+1] as u32; // 7 bit
	let s2 = datavec[stridx+2] as u32; // 7 bit
	let s3 = datavec[stridx+3] as u32; // 7 bit
	let s4 = datavec[stridx+4] as u32; // 7 bit
	let d: u32 = (s0 << (35-7)) | (s1 << (35-7-7)) | (s2 << (35-7-7-7)) | (s3 << (35-7-7-7-7)) | s4;

	if d != i as u32 { println!("Error: compare failed ({} <> {})", i, d); }  // check
    }
    Ok(0)
}
