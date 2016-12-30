#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>

int main() {
    // -----> c_example.h
    //int Example(const char *s); // return id
    //char *Get(int id);
    int (*Example)(const char *s); // return id
    char *(*Get)(int id);

    void *c_example = dlopen("./c_example.so", RTLD_LAZY);
    if (!c_example) {
	fprintf(stderr, "%s\n", dlerror());
        exit(EXIT_FAILURE);
    }
    dlerror();

    Example = (int (*)(const char *)) dlsym(c_example, "Example");
    Get     = (char *(*)(int))        dlsym(c_example, "Get");
    // end of function loading


    int a = Example("teszt-A");
    int b = Example("teszt-B");

    printf("%s\n", Get(a));
    printf("%s\n", Get(b));
    printf("%s\n", Get(a));
    printf("%s\n", Get(a));
    printf("%s\n", Get(a));
    printf("%s\n", Get(a));
    printf("%s\n", Get(b));

    dlclose(c_example);
    return 0;
}
