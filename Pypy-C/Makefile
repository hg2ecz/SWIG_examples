PROJECT=c_example

CC=gcc
#CFLAGS=-O2 -Wall `pkg-config --cflags python` -fpic
CFLAGS=-O2 -Wall -I/usr/lib/pypy/include/ -fpic
LDFLAGS=-shared -s

OBJS=$(PROJECT)_wrap.o $(PROJECT).o
#TARGET=../_$(PROJECT).so
TARGET=_$(PROJECT).pypy-41-arm-linux-gnueabihf.so

%.py: %.i
	swig -python $<

all: $(PROJECT).py
	make sofile

sofile: $(OBJS)
	$(CC) $(LDFLAGS) $(OBJS) -o $(TARGET)


clean:
	rm -f $(OBJS) $(PROJECT)_wrap.c $(PROJECT).py ../$(PROJECT).py $(TARGET)
