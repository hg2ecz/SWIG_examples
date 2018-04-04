// Simple Python list

use cpython::{Python, PyResult};

pub fn source1(_py: Python, len: i32) -> PyResult<Vec<i32>> {
    let mut data = Vec::new();
    for i in 0..len {
	data.push(i);
    }
    Ok(data)
}

pub fn sink1(_py: Python, data: Vec<i32>) -> PyResult<i32>{
    let mut i = 0;
    let datalen = data.len();
    for d in data {
	if d != i {
	    println!("Az {}. adat eltér: {}, adathossz: {}", i, d, datalen);
	}
	i+=1;
    }
    Ok(0)
}
