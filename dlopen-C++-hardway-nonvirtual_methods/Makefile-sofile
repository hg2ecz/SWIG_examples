PROJECT=cpp_example

CXX=g++
CXXFLAGS=-O2 -Wall -fpic
LDFLAGS=-shared -s

OBJS=$(PROJECT).o $(PROJECT)_c_wrapper.o
TARGET=$(PROJECT).so

all: $(OBJS)
	$(CXX) $(LDFLAGS) $(OBJS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)
