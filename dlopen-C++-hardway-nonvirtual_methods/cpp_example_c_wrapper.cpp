#include "cpp_example.hpp"

extern "C" {
  Example *Example_Example(const char *s) { return new Example(s); } // constructor interface as function
  void Example__gc(Example *this_) { delete this_; }                 // in destructor we delete this class
  char *Example_Get(Example *this_) { return this_->Get(); }         // example method as function call with obj as parameter
}
