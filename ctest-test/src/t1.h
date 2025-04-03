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

typedef double T1TypedefDouble;
typedef int* T1TypedefPtr;
typedef struct T1Bar T1TypedefStruct;

void T1a(void);
void* T1b(void);
void* T1c(void*);
int32_t T1d(unsigned);
void T1e(unsigned, const struct T1Bar*);
void T1f(void);
void T1g(int32_t* a);
void T1h(const int32_t* b);
void T1i(int32_t a[4]);
void T1j(const int32_t b[4]);
void T1o(int32_t (*a)[4]);
void T1p(int32_t (*const a)[4]);

typedef int32_t (Arr)[4];
typedef int32_t Transparent;

void T1r(Arr a);
void T1s(const Arr a);
void T1t(Arr* a);
void T1v(const Arr* a);

#define T1C 4

extern uint32_t T1static;
extern const uint8_t T1_static_u8;
uint8_t T1_static_mut_u8;
uint8_t (*T1_static_mut_fn_ptr)(uint8_t, uint8_t);
extern uint8_t (*const T1_static_const_fn_ptr_unsafe)(uint8_t, uint8_t);
extern void (*const T1_static_const_fn_ptr_unsafe2)(uint8_t);
extern void (*const T1_static_const_fn_ptr_unsafe3)(void);

extern const uint8_t T1_static_right;
uint8_t (*T1_static_right2)(uint8_t, uint8_t);

// T1_fn_ptr_nested: function pointer to a function, taking a uint8_t, and
// returning a function pointer to a function taking a uint16_t and returning a
// uint32_t
uint32_t (*(*T1_fn_ptr_s)(uint8_t))(uint16_t);

// T1_fn_ptr_nested: function pointer to a function, taking a function pointer
// uint8_t -> uint8_t, and returning a function pointer to a function taking a
// uint16_t and returning a uint32_t
uint32_t (*(*T1_fn_ptr_s2)(uint8_t(*)(uint8_t), uint16_t(*)(uint16_t)))(uint16_t);

extern const int32_t T1_arr0[2];
extern const int32_t T1_arr1[2][3];
extern const int32_t T1_arr2[1][2][3];

extern int32_t T1_arr3[2];
extern int32_t T1_arr4[2][3];
extern int32_t T1_arr5[1][2][3];

extern int32_t T1_arr42[1][2][3];

extern const int16_t* T1_sref;

extern const int32_t* T1_mut_opt_ref;
extern int32_t* T1_mut_opt_mut_ref;
extern const int32_t* T1_const_opt_const_ref;

extern void (*const T1_opt_fn1)(void);
uint32_t (*(*T1_opt_fn2)(uint8_t))(uint16_t);
uint32_t (*(*T1_opt_fn3)(uint8_t(*)(uint8_t), uint16_t(*)(uint16_t)))(uint16_t);


struct Q {
  uint8_t* q0;
  uint8_t** q1;
  uint8_t q2;
};


struct T1_conflict_foo {
  int a;
};

struct T1_conflict{
  int foo;
};

// test packed structs
//
// on msvc there is only pragma pack
// on clang and gcc there is a packed attribute

# pragma pack(push,1)

struct Pack {
  uint8_t a;
  uint16_t b;
};

# pragma pack(pop)

# pragma pack(push,4)

struct Pack4 {
  uint8_t a;
  uint32_t b;
};

# pragma pack(pop)

// volatile pointers in struct fields:
struct V {
  volatile uint8_t* v;
};

// volatile pointers in externs:
extern volatile uint8_t* vol_ptr;

// volatile pointers in function arguments:
void* T1_vol0(volatile void*, void*);
volatile void* T1_vol1(void*, void*);
volatile void* T1_vol2(void*, volatile void*);

// volatile function pointers:
uint8_t (*volatile T1_fn_ptr_vol)(uint8_t, uint8_t);

#define LOG_MAX_LINE_LENGTH (1400)

typedef struct {
  long tv_sec;
  int tv_usec;
} timeval;

typedef struct
{
  long level;
  char const *file;
  long line;
  char const *module;
  timeval tv;
  char message[LOG_MAX_LINE_LENGTH];
} log_record_t;

typedef struct
{
  long double inner;
} LongDoubleWrap;
