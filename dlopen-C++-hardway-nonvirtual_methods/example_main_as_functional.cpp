#include <iostream>
#include <dlfcn.h>

#include "cpp_example.h"
using namespace std;

// wrap into functional like behavior
static Example *(*Example_Example)(const char *s);
static void     (*Example__gc    )(Example *obj);  // function from .so --> descructor as func
static char    *(*Example_Get    )(Example *obj);  // function from .so --> example method as func

#define MAXID 10
static int Examplevec_id=0;
static Example *Examplevec[MAXID];

int ExampleOpen(const char *s) {
    if (Examplevec_id >= 10) return -1;
    Examplevec[Examplevec_id] = Example_Example(s); // new Example(s)
    return Examplevec_id++;
}

char *ExampleGet(int id) {
    return Example_Get(Examplevec[id]);
}

void ExampleClose(int id) {
    Example__gc(Examplevec[id]);
}
// end of wrap

int main() {
    // -----> cpp_example_c_wrapper.cpp
    static void *cpp_example = dlopen("./cpp_example.so", RTLD_LAZY);
    if (!cpp_example) {
	cerr << dlerror() << endl;
	exit(EXIT_FAILURE);
    }
    dlerror();

    Example_Example = (Example* (*)(const char*)) dlsym(cpp_example, "Example_Example");
    Example_Get     = (char*    (*)(Example *))   dlsym(cpp_example, "Example_Get");
    Example__gc     = (void     (*)(Example *))   dlsym(cpp_example, "Example__gc");
    // end of function loading


    int a = ExampleOpen("teszt-A");
    int b = ExampleOpen("teszt-B");

    printf("%s\n", ExampleGet(a));
    printf("%s\n", ExampleGet(b));
    printf("%s\n", ExampleGet(a));
    printf("%s\n", ExampleGet(a));
    printf("%s\n", ExampleGet(a));
    printf("%s\n", ExampleGet(a));
    printf("%s\n", ExampleGet(b));

    ExampleClose(a);
    ExampleClose(b);
    return 0;
}
