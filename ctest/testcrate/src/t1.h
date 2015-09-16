#include <stdint.h>

typedef int32_t T1Foo;

struct T1Bar {
  int32_t a;
  uint32_t b;
  T1Foo c;
  uint8_t d;
};

struct T1Baz {
  uint64_t a;
  struct T1Bar b;
};

void T1a(void);
void* T1b(void);
void* T1c(void*);
int32_t T1d(unsigned);
void T1e(unsigned, const struct T1Bar*);
void T1f(void);

#define T1C 4
