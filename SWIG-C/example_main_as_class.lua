#!/usr/bin/luajit

require ("c_example") -- native C functions

function main()
    -- wrap into class like behavior
    local mt = {}
    mt.__index = mt

    function Example(...)
        local self = {super = c_example.Example(...)}
        -- ffi.gc(self.super, c_example.Example_close) -- if we have close() or free()
        return setmetatable(self, mt)
    end

    function mt.Get(self, ...)
        return c_example.Get(self.super)
    end
    -- end of wrap


    a = Example('Teszt-A')
    b = Example('Teszt-B')

    print (a:Get())
    print (b:Get())
    print (a:Get())
    print (a:Get())
    print (a:Get())
    print (a:Get())
    print (b:Get())
end

main()
