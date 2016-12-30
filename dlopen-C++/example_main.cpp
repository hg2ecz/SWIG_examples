#include <iostream>
#include <dlfcn.h>

#include "cpp_example.h"

using namespace std;

int main() {
    // -----> cpp_example_c_wrapper.cpp
    static void *cpp_example = dlopen("./cpp_example.so", RTLD_LAZY);
    if (!cpp_example) {
	cerr << dlerror() << endl;
	exit(EXIT_FAILURE);
    }
    dlerror();

    Example *(*Example_Example)(const char *s) = (Example* (*)(const char*)) dlsym(cpp_example, "Example_Example");
    // end of function loading

    Example *a = Example_Example("teszt-A"); // new Example()
    Example *b = Example_Example("teszt-B"); // new Example()

    printf("%s\n", a->Get());
    printf("%s\n", b->Get());
    printf("%s\n", a->Get());
    printf("%s\n", a->Get());
    printf("%s\n", a->Get());
    printf("%s\n", a->Get());
    printf("%s\n", b->Get());

    dlclose(cpp_example);
    return 0;
}
