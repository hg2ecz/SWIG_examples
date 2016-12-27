PROJECT=cpp_example

CXX=g++
#CFLAGS=-O2 -Wall `pkg-config --cflags python` -fPIC
CXXFLAGS=-O2 -Wall -I/usr/lib/pypy/include/ -fpic
LDFLAGS=-shared -s

OBJS=$(PROJECT)_wrap.o $(PROJECT).o
#TARGET=../_$(PROJECT).so
TARGET=_$(PROJECT).pypy-41-arm-linux-gnueabihf.so

%.py: %.i
	swig -c++ -python $<

%.o: %.cxx
	$(CXX) $(CXXFLAGS) -c $<

all: $(PROJECT).py
	make sofile

sofile: $(OBJS)
	$(CXX) $(LDFLAGS) $(OBJS) -o $(TARGET)


clean:
	rm -f $(OBJS) $(PROJECT)_wrap.cxx $(PROJECT).py ../$(PROJECT).py $(TARGET)
