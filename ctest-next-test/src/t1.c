#include "t1.h"
#include <stdint.h>
#include <stdlib.h>

void T1a(void) {}
void *T1b(void) { return NULL; }
void *T1c(void *a) { return NULL; }
int32_t T1d(unsigned a) { return 0; }
void T1e(unsigned a, const struct T1Bar *b) {}
void T1f(void) {}
void T1g(int32_t *a) {}
void T1h(const int32_t *b) {}
void T1i(int32_t a[4]) {}
void T1j(const int32_t b[4]) {}
void T1o(int32_t (*a)[4]) {}
void T1p(int32_t (*const a)[4]) {}

void T1r(Arr a) {}
void T1s(const Arr a) {}
void T1t(Arr *a) {}
void T1v(const Arr *a) {}

unsigned T1static = 3;

const uint8_t T1_static_u8 = 42;
uint8_t T1_static_mut_u8 = 37;

uint8_t foo(uint8_t a, uint8_t b) { return a + b; }
void bar(uint8_t a) { return; }
void baz(void) { return; }

uint32_t (*nested(uint8_t arg))(uint16_t)
{
  return NULL;
}

uint32_t (*nested2(uint8_t (*arg0)(uint8_t), uint16_t (*arg1)(uint16_t)))(uint16_t)
{
  return NULL;
}

uint8_t (*T1_static_mut_fn_ptr)(uint8_t, uint8_t) = foo;
uint8_t (*const T1_static_const_fn_ptr_unsafe)(uint8_t, uint8_t) = foo;
void (*const T1_static_const_fn_ptr_unsafe2)(uint8_t) = bar;
void (*const T1_static_const_fn_ptr_unsafe3)(void) = baz;

const uint8_t T1_static_right = 7;
uint8_t (*T1_static_right2)(uint8_t, uint8_t) = foo;

uint32_t (*(*T1_fn_ptr_s)(uint8_t))(uint16_t) = nested;
uint32_t (*(*T1_fn_ptr_s2)(uint8_t (*arg0)(uint8_t), uint16_t (*arg1)(uint16_t)))(uint16_t) = nested2;

const int32_t T1_arr0[2] = {0, 0};
const int32_t T1_arr1[2][3] = {{0, 0, 0}, {0, 0, 0}};
const int32_t T1_arr2[1][2][3] = {{{0, 0, 0}, {0, 0, 0}}};

int32_t T1_arr3[2] = {0, 0};
int32_t T1_arr4[2][3] = {{0, 0, 0}, {0, 0, 0}};
int32_t T1_arr5[1][2][3] = {{{0, 0, 0}, {0, 0, 0}}};

int32_t T1_arr42[1][2][3] = {{{0, 0, 0}, {0, 0, 0}}};
const int16_t *T1_sref = (void *)(1337);

volatile uint8_t *vol_ptr = NULL;
void *T1_vol0(volatile void *x, void *a) { return a ? a : (void *)x; }
volatile void *T1_vol1(void *x, void *b) { return b ? (volatile void *)x : (volatile void *)x; }
volatile void *T1_vol2(void *c, volatile void *x) { return c ? x : x; }

// FIXME(#4365): duplicate symbol errors when enabled
// uint8_t (*volatile T1_fn_ptr_vol)(uint8_t, uint8_t) = foo;
