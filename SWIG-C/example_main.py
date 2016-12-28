#!/usr/bin/pypy

import c_example

if __name__ == '__main__':
    a = c_example.Example('teszt-A')
    b = c_example.Example('teszt-B')

    print c_example.Get(a)
    print c_example.Get(b)
    print c_example.Get(a)
    print c_example.Get(a)
    print c_example.Get(a)
    print c_example.Get(a)
    print c_example.Get(b)
