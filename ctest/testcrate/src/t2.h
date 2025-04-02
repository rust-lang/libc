#include <stdint.h>

typedef int32_t T2Foo;
typedef int8_t T2Bar;

typedef T2Foo T2TypedefFoo;
typedef unsigned T2TypedefInt;

struct T2Baz {
  int8_t _a;
  int64_t a;
  uint32_t b;
};

typedef struct {
  uint32_t a;
  int64_t b;
} T2Union;

static void T2a(void) {}

#define T2C 4
#define T2S "a"
