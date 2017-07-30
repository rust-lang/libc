#include <stdint.h>

typedef int32_t T1Foo;

#define T1N 5
#define T1S "foo"

struct T1Bar {
  int32_t a;
  uint32_t b;
  T1Foo c;
  uint8_t d;
  int64_t e[T1N];
  int64_t f[T1N][2];
};

struct T1Baz {
  uint64_t a;
  struct T1Bar b;
};

typedef union {
  uint64_t a;
  uint32_t b;
} T1Union;

void T1a(void);
void* T1b(void);
void* T1c(void*);
int32_t T1d(unsigned);
void T1e(unsigned, const struct T1Bar*);
void T1f(void);
void T1g(const int32_t a[4]);
void T1h(const int32_t a[4]);
void T1i(int32_t a[4]);
void T1j(int32_t a[4]);

#define T1C 4

