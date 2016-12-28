#!/usr/bin/pypy

import cpp_example

if __name__ == '__main__':
    a = cpp_example.Example('teszt-A')
    b = cpp_example.Example('teszt-B')

    print a.Get()
    print b.Get()
    print a.Get()
    print a.Get()
    print a.Get()
    print a.Get()
    print b.Get()
