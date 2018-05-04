#include <iostream>
#include <dlfcn.h>

using namespace std;

// wrap into class like behavior
extern "C" {
    extern int Example(const char *s); // return id
    extern char *Get(int id);
}

class Exampleclass {
    public:
	Exampleclass(const char *s) { this->descr = Example(s); }
	char *Getm()                { return Get(this->descr);  };
//	~Example()             { Example_Close(this->descr);};
    private:
	int descr;
};
// end of wrap


int main() {
    Exampleclass *a = new Exampleclass("teszt-A"); // new Example()
    Exampleclass *b = new Exampleclass("teszt-B"); // new Example()

    printf("%s\n", a->Getm());
    printf("%s\n", b->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", a->Getm());
    printf("%s\n", b->Getm());

    return 0;
}
