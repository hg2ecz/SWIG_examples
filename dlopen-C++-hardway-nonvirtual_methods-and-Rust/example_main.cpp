#include <iostream>
#include <dlfcn.h>

using namespace std;

// wrap into class like behavior
class Example;
static Example *(*Example_Example)(const char *s); // function from .so --> constructor as func
static void     (*Example__gc    )(Example *obj);  // function from .so --> descructor as func
static char    *(*Example_Get    )(Example *obj);  // function from .so --> example method as func


class Example {
    public:
	Example(const char *s) { this->obj = Example_Example(s); }
	~Example()             { Example__gc(this->obj);         };
	char *Get()            { return Example_Get(this->obj);  };
    private:
	Example *obj;
};
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
    Example__gc     = (void     (*)(Example   *)) dlsym(cpp_example, "Example__gc");
    Example_Get     = (char   * (*)(Example   *)) dlsym(cpp_example, "Example_Get");
    // end of function loading

    Example *a = new Example("teszt-A"); // new Example()
    Example *b = new Example("teszt-B"); // new Example()

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
