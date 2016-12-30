#include <iostream>
#include <dlfcn.h>

#include "cpp_example.h"
using namespace std;

// wrap into functional like behavior
#define MAXID 10
static int Examplevec_id=0;
static Example *Examplevec[MAXID];

int ExampleOpen(const char *s) {
    if (Examplevec_id >= 10) return -1;
    Examplevec[Examplevec_id] = new Example(s);
    return Examplevec_id++;
}

char *ExampleGet(int id) {
    return Examplevec[id]->Get();
}

void ExampleClose(int id) {
    delete Examplevec[id];
}
// end of wrap

int main() {
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
