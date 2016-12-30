#include "cpp_example.h"

extern "C" Example *Example_Example(const char *s) { return new Example(s); } // constructor interface as function
