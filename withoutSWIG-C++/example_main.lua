#!/usr/bin/luajit

function main()
    local ffi = require ('ffi')
    ffi.cdef[[
        typedef struct Example Example;
        Example *Example_Example(const char *s);
        char *Example__gc(Example *);
        char *Example_Get(Example *);
    ]]

    local cpp_example = ffi.load('./cpp_example.so')

    -- wrap into class like behavior
    local mt = {}
    mt.__index = mt

    function Example(...)
        local self = {super = cpp_example.Example_Example(...)}
        ffi.gc(self.super, cpp_example.Example__gc)
        return setmetatable(self, mt)
    end

    function mt.Get(self, ...)
        return ffi.string(cpp_example.Example_Get(self.super, ...))
    end
    -- end of wrap

    d = ffi.new("char[200]")
    ffi.copy(d, 'teszt-A'); a = Example(d)
    ffi.copy(d, 'teszt-B'); b = Example(d)

    print (a:Get())
    print (b:Get())
    print (a:Get())
    print (a:Get())
    print (a:Get())
    print (a:Get())
    print (b:Get())
end

main()
