#!/usr/bin/pypy

import c_example   # native C functions


if __name__ == '__main__':
    # wrap into class like behavior
    class Example(object):
        def __init__(self, *arg):
            self.descr = c_example.Example(arg[0])

        def Get(self, *arg):
            return c_example.Get(self.descr)

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
