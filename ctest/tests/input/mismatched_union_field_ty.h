#include <stdint.h>

// The union must match the size and alignment of the Rust side,
// otherwise different warnings are emitted that hide -Wincompatible-pointer-types.
union Bar {
    int64_t x;
    int32_t y;
};
