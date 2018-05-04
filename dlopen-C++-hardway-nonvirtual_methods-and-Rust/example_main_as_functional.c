#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>

// wrap into functional like behavior
static void *(*Example_Example)(const char *s);
static void     (*Example__gc    )(void *obj);  // function from .so --> descructor as func
static char    *(*Example_Get    )(void *obj);  // function from .so --> example method as func

#define MAXID 10
static int Examplevec_id=0;
static void *Examplevec[MAXID];

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
    void *cpp_example = dlopen("./cpp_example.so", RTLD_LAZY);
    if (!cpp_example) {
	fprintf(stderr, "%s\n", dlerror());
	exit(EXIT_FAILURE);
    }
    dlerror();

    Example_Example = (void* (*)(const char*)) dlsym(cpp_example, "Example_Example");
    Example_Get     = (char*    (*)(void *))   dlsym(cpp_example, "Example_Get");
    Example__gc     = (void     (*)(void *))   dlsym(cpp_example, "Example__gc");
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
