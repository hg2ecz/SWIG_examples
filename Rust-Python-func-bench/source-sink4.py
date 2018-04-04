#!/usr/bin/python3

import time
import source_sink as sosi

if __name__ == '__main__':
    data = sosi.source4(1000);
    data = data[:999] + '\0' + data[999 + 1:]
    print ("Teszt .. szándékos 999 : 0 hiba")
    sosi.sink4(data)

    timestart = time.time()
    # ---------------------
    for i in range(1000):
        data = sosi.source4(50000)
        sosi.sink4(data)
    # ---------------------
    eltime = 1000*(time.time() - timestart)
    print ("Elapsed: %.1f ms"%eltime)
