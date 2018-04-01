#![crate_type = "dylib"]

#[macro_use] extern crate cpython;

use cpython::{Python, PyResult};

py_module_initializer!(rust_example, initrust_example, PyInit_rust_example, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "sin", py_fn!(py, sin(arg: f64))));
    try!(m.add(py, "cos", py_fn!(py, cos(arg: f64))));
    Ok(())
});

fn sin(_py: Python, arg: f64) -> PyResult<f64> {
    Ok(arg.sin())
}

fn cos(_py: Python, arg: f64) -> PyResult<f64> {
    Ok(arg.cos())
}
