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

union T1NoTypedefUnion {
    uint64_t a;
    uint32_t b;
};

struct T1StructWithUnion {
    union T1NoTypedefUnion u;
};

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

extern uint32_t T1static;
const uint8_t T1_static_u8;
uint8_t T1_static_mut_u8;
uint8_t (*T1_static_mut_fn_ptr)(uint8_t, uint8_t);
uint8_t (*const T1_static_const_fn_ptr_unsafe)(uint8_t, uint8_t);
void (*const T1_static_const_fn_ptr_unsafe2)(uint8_t);
void (*const T1_static_const_fn_ptr_unsafe3)(void);

const uint8_t T1_static_right;
uint8_t (*T1_static_right2)(uint8_t, uint8_t);

// T1_fn_ptr_nested: function pointer to a function, taking a uint8_t, and
// returning a function pointer to a function taking a uint16_t and returning a
// uint32_t
uint32_t (*(*T1_fn_ptr_s)(uint8_t))(uint16_t);

// T1_fn_ptr_nested: function pointer to a function, taking a function pointer
// uint8_t -> uint8_t, and returning a function pointer to a function taking a
// uint16_t and returning a uint32_t
uint32_t (*(*T1_fn_ptr_s2)(uint8_t(*)(uint8_t), uint16_t(*)(uint16_t)))(uint16_t);

const int32_t T1_arr0[2] = {0};
const int32_t T1_arr1[2][3] = {0};
const int32_t T1_arr2[1][2][3] = {0};

int32_t T1_arr3[2];
int32_t T1_arr4[2][3];
int32_t T1_arr5[1][2][3];

int32_t T1_arr42[1][2][3];
