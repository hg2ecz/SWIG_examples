#!/usr/bin/pypy

import ctypes

if __name__ == '__main__':
    c_example = ctypes.CDLL('./c_example.so')

    a = c_example.Example('teszt-A')
    b = c_example.Example('teszt-B')

    print ctypes.c_char_p(c_example.Get(a)).value
    print ctypes.c_char_p(c_example.Get(b)).value
    print ctypes.c_char_p(c_example.Get(a)).value
    print ctypes.c_char_p(c_example.Get(a)).value
    print ctypes.c_char_p(c_example.Get(a)).value
    print ctypes.c_char_p(c_example.Get(a)).value
    print ctypes.c_char_p(c_example.Get(b)).value
