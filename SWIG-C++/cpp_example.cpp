#include <iostream>
#include <cstring>

#include "cpp_example.hpp"

using namespace std;

Example :: Example(const char *s) {
    strncpy(this->s, s, sizeof(this->s));
}

char *Example :: Get() {
    static char s[210];
    sprintf(s, "%2d %s", this->counter++, this->s);
    return s;
}

Example :: ~Example() { }
