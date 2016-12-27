#include <iostream>
#include <cstring>

#include "cpp_example.h"

using namespace std;

Example :: Example(char *s) {
    strncpy(this->s, s, sizeof(this->s));
}

char *Example :: Get() {
    static char s[210];
    sprintf(s, "%2d %s", this->counter++, this->s);
    return s;
}
