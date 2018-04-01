#![crate_type = "dylib"]

#[macro_use] extern crate cpython;

use cpython::{PyResult};

py_module_initializer!(rust_example, initrust_example, PyInit_rust_example, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add_class::<Example>(py));
    Ok(())
});

py_class!(class Example |py| {
    data s: String;
    def __new__(_cls, arg: String) -> PyResult<Example> {
	Example::create_instance(py, arg)
    }
    def get(&self) -> PyResult<String> {
	let res = format!("{}", self.s(py));
        Ok(res)
    }
});
