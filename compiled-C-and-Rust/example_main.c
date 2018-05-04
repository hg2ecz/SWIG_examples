#include <stdio.h>
#include <stdlib.h>

//#include "c_example.h"

extern int Example(const char *s); // return id
extern char *Get(int id);


int main() {
    int a = Example("teszt-A");
    int b = Example("teszt-B");

    printf("%s\n", Get(a));
    printf("%s\n", Get(b));
    printf("%s\n", Get(a));
    printf("%s\n", Get(a));
    printf("%s\n", Get(a));
    printf("%s\n", Get(a));
    printf("%s\n", Get(b));
    return 0;
}
