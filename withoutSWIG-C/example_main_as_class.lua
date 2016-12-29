#!/usr/bin/luajit

function main()
    local ffi = require ('ffi')
    ffi.cdef[[
        int Example(const char *s);
        char *Get(int id);
    ]]

    local c_example = ffi.load('./c_example.so')

    -- wrap into class like behavior
    local mt = {}
    mt.__index = mt

    function Example(...)
        local self = {super = c_example.Example(...)}
        -- ffi.gc(self.super, c_example.Example_close) -- if we have close() or free()
        return setmetatable(self, mt)
    end

    function mt.Get(self, ...)
        return ffi.string(c_example.Get(self.super))
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
