#include "cpp_example.h"

extern "C" {
  Example *Example_Example(const char *s) { return new Example(s); }
  void Example__gc(Example *this_) { delete this_; } // egyúttal töröljük a referenciát
  char *Example_Get(Example *this_) { return this_->Get(); }
}
