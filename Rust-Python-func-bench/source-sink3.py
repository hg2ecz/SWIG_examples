#!/usr/bin/python3

import time
import source_sink as sosi

if __name__ == '__main__':
    data = sosi.source3(50000);       # fix 50000 !!!
    data = data[:999] + '\0' + data[999 + 1:]
    print ("Teszt .. szándékos 999 : 0 hiba")
    sosi.sink3(data)

    timestart = time.time()
    # ---------------------
    for i in range(1000):
        data = sosi.source3(50000)     # fix 50000 !!!
        sosi.sink3(data)
    # ---------------------
    eltime = 1000*(time.time() - timestart)
    print ("Elapsed: %.1f ms"%eltime)
