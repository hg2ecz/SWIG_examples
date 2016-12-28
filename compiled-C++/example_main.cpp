#include <iostream>
#include <dlfcn.h>

#include "cpp_example.h"
using namespace std;

int main() {
    Example *a = new Example("teszt-A"); // new Example()
    Example *b = new Example("teszt-B"); // new Example()

    printf("%s\n", a->Get());
    printf("%s\n", b->Get());
    printf("%s\n", a->Get());
    printf("%s\n", a->Get());
    printf("%s\n", a->Get());
    printf("%s\n", a->Get());
    printf("%s\n", b->Get());
    return 0;
}
