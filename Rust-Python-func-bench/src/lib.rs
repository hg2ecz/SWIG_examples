#![crate_type = "dylib"]
#[macro_use] extern crate cpython;

mod sourcesink1;
use sourcesink1::{source1, sink1};

mod sourcesink2;
use sourcesink2::{source2, sink2};

mod sourcesink3;
use sourcesink3::{source3, sink3};

mod sourcesink4;
use sourcesink4::{source4, sink4};

mod sourcesink5;
use sourcesink5::{source5, sink5};

mod sourcesink6;
use sourcesink6::{source6, sink6};

py_module_initializer!(source_sink, initsource_sink, PyInit_source_sink, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));

    // data: Rust Vec<i32> <---> Python List
    try!(m.add(py, "source1", py_fn!(py, source1(len: i32))));
    try!(m.add(py, "sink1"  , py_fn!(py, sink1(data: Vec<i32>))));

    // data: Rust String <---> String
    try!(m.add(py, "source2", py_fn!(py, source2(len: i32))));
    try!(m.add(py, "sink2"  , py_fn!(py, sink2(data: String))));

    // data: Rust String <---> String ... block transmute
    try!(m.add(py, "source3", py_fn!(py, source3(len: i32))));
    try!(m.add(py, "sink3"  , py_fn!(py, sink3(data: String))));

    // data: Rust String <---> String ... no utf8 ... 7 bit/char ... transmute()
    try!(m.add(py, "source4", py_fn!(py, source4(len: i32))));
    try!(m.add(py, "sink4"  , py_fn!(py, sink4(data: String))));

    // data: Rust String <---> String ... no utf8 ... 7 bit/char ... bytevector
    try!(m.add(py, "source5", py_fn!(py, source5(len: i32))));
    try!(m.add(py, "sink5"  , py_fn!(py, sink5(data: String))));

    // data: Rust String <---> String ... no utf8 ... 7 bit/char & direct mapping
    try!(m.add(py, "source6", py_fn!(py, source6(len: i32))));
    try!(m.add(py, "sink6"  , py_fn!(py, sink6(data: String))));
    Ok(())
});
