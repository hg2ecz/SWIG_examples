#!/usr/bin/luajit

require ("cpp_example")

function main()
    a = cpp_example.Example('teszt-A')
    b = cpp_example.Example('teszt-B')

    print (a:Get())
    print (b:Get())
    print (a:Get())
    print (a:Get())
    print (a:Get())
    print (a:Get())
    print (b:Get())
end

main()
