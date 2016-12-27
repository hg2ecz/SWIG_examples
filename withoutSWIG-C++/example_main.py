#!/usr/bin/pypy

import ctypes

if __name__ == '__main__':
    cpp_example = ctypes.CDLL('./cpp_example.so')

    # wrap into class like behavior
    class Example(object):
        def __init__(self, *arg):
            self.obj = cpp_example.Example_Example(*arg)

        def Get(self, *arg):
            return ctypes.c_char_p(cpp_example.Example_Get(self.obj, *arg)).value
    # end of wrap

    a = Example('teszt-A')
    b = Example('teszt-B')

    print a.Get()
    print b.Get()
    print a.Get()
    print a.Get()
    print a.Get()
    print a.Get()
    print b.Get()
