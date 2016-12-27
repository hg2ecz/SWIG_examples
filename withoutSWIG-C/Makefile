PROJECT=c_example

CC=gcc
#CFLAGS=-O2 -Wall `pkg-config --cflags python` -fpic
CFLAGS=-O2 -Wall -I/usr/lib/pypy/include/ -fpic
LDFLAGS=-shared -s

OBJS=$(PROJECT).o
TARGET=$(PROJECT).so
#TARGET=_$(PROJECT).pypy-41-arm-linux-gnueabihf.so

all: $(OBJS)
	$(CC) $(LDFLAGS) $(OBJS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)
