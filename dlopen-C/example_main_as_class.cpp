#include <iostream>
#include <dlfcn.h>

using namespace std;

// wrap into class like behavior
static int (*Example)(const char *s); // return id
static char *(*Get)(int id);

class Exampleclass {
    public:
	Exampleclass(const char *s) { this->descr = Example(s); }
	char *Getm()                { return Get(this->descr);  };
//	~Exampleclass()             { Example_Close(this->descr);};
    private:
	int descr;
};
// end of wrap


int main() {
    // -----> cpp_example_c_wrapper.cpp
    static void *c_example = dlopen("./c_example.so", RTLD_LAZY);
    if (!c_example) {
	cerr << dlerror() << endl;
	exit(EXIT_FAILURE);
    }
    dlerror();

    Example = (int (*)(const char *)) dlsym(c_example, "Example");
    Get     = (char *(*)(int))        dlsym(c_example, "Get");
    // end of function loading

    Exampleclass *a = new Exampleclass("teszt-A"); // new Example()
    Exampleclass *b = new Exampleclass("teszt-B"); // new Example()

    printf("%s\n", a->Getm());
    printf("%s\n", b->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", b->Getm());

    dlclose(c_example);
    return 0;
}
