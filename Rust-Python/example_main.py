#!/usr/bin/python3

import rust_example

if __name__ == '__main__':
    a = rust_example.Example('teszt-A')
    b = rust_example.Example('teszt-B')

    print (a.get())
    print (b.get())
    print (a.get())
    print (a.get())
    print (a.get())
    print (a.get())
    print (b.get())
