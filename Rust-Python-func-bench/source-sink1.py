#!/usr/bin/python3

import time
import source_sink as sosi

if __name__ == '__main__':
    data = sosi.source1(1000);
    data[-1]=0
    print ("Teszt .. szándékos 999 : 0 hiba")
    sosi.sink1(data)

    timestart = time.time()
    # ---------------------
    for i in range(1000):
        data = sosi.source1(50000)
        sosi.sink1(data)
    # ---------------------
    eltime = 1000*(time.time() - timestart)
    print ("Elapsed: %.1f ms"%eltime)
