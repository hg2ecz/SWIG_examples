#!/usr/bin/luajit

require ("c_example")

function main()
    a = c_example.Example('teszt-A')
    b = c_example.Example('teszt-B')

    print (c_example.Get(a))
    print (c_example.Get(b))
    print (c_example.Get(a))
    print (c_example.Get(a))
    print (c_example.Get(a))
    print (c_example.Get(a))
    print (c_example.Get(b))
end

main()
