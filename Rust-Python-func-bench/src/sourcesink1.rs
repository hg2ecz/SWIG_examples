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
    for d in data {                                                        // d: i32
	if d != i { println!("Error: compare failed ({} <> {})", i, d); }  // check
	i+=1;
    }
    Ok(0)
}
