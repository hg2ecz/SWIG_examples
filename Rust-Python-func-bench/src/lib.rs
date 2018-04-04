#![crate_type = "dylib"]
#[macro_use] extern crate cpython;

mod sourcesink1;
use sourcesink1::{source1, sink1};

mod sourcesink2;
use sourcesink2::{source2, sink2};



py_module_initializer!(source_sink, initsource_sink, PyInit_source_sink, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));

    // data: Rust Vec<i32> <---> Python List
    try!(m.add(py, "source1", py_fn!(py, source1(len: i32))));
    try!(m.add(py, "sink1"  , py_fn!(py, sink1(data: Vec<i32>))));

    // data: Rust String <---> String
    try!(m.add(py, "source2", py_fn!(py, source2(len: i32))));
    try!(m.add(py, "sink2"  , py_fn!(py, sink2(data: String))));
    Ok(())
});
