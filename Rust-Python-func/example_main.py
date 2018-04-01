#!/usr/bin/python3

import rust_example as rex

if __name__ == '__main__':
    print (rex.sin(3.14159259))
    print (rex.cos(3.14159259))

    print (rex.sin(3.14159259/2.))
    print (rex.cos(3.14159259/2.))
