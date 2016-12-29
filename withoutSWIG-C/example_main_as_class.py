#!/usr/bin/pypy

import ctypes

if __name__ == '__main__':
    c_example = ctypes.CDLL('./c_example.so') # native C functions

    # wrap into class like behavior
    class Example(object):
        def __init__(self, *arg):
            self.descr = c_example.Example(arg[0])

        def Get(self, *arg):
            return ctypes.c_char_p(c_example.Get(self.descr)).value

        #def __del__(self):
        #   close(), free() and other
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
