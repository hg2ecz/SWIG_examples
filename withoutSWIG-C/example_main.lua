#!/usr/bin/luajit

function main()
    local ffi = require ('ffi')
    ffi.cdef[[
        int Example(char *s);
        char *Get(int id);
    ]]

    local c_example = ffi.load('./c_example.so')

    d = ffi.new("char[200]")
    ffi.copy(d, 'teszt-A'); a = c_example.Example(d)
    ffi.copy(d, 'teszt-B'); b = c_example.Example(d)

    print (ffi.string(c_example.Get(a)))
    print (ffi.string(c_example.Get(b)))
    print (ffi.string(c_example.Get(a)))
    print (ffi.string(c_example.Get(a)))
    print (ffi.string(c_example.Get(a)))
    print (ffi.string(c_example.Get(a)))
    print (ffi.string(c_example.Get(b)))
end

main()
