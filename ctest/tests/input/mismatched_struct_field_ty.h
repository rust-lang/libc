#include <stdint.h>

// The struct and union must match the size and alignment of the Rust side,
// otherwise different warnings are emitted that hide -Wincompatible-pointer-types.
struct Bar {
    int16_t x;
    int16_t y;
};

union Baz {
    int32_t x;
    float y;
};

struct Foo {
    struct Bar a;
    union Baz b;
};
