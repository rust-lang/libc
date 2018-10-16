#include <stdlib.h>
#include "t1.h"

void T1a(void) {}
void* T1b(void) { return NULL; }
void* T1c(void* a) { return NULL; }
int32_t T1d(unsigned a ) { return 0; }
void T1e(unsigned a, const struct T1Bar* b) { }
void T1f(void) {}
void T1g(const int32_t a[4]) {}
void T1h(const int32_t a[4]) {}
void T1i(int32_t a[4]) {}
void T1j(int32_t a[4]) {}
unsigned T1static = 3;

const uint8_t T1_static_u8 = 42;
uint8_t T1_static_mut_u8 = 37;


uint8_t foo(uint8_t a, uint8_t b) { return a + b; }
void bar(uint8_t a) { return; }
void baz(void) { return; }

uint32_t (*nested(uint8_t arg))(uint16_t) {
  return NULL;
}

uint32_t (*nested2(uint8_t(*arg0)(uint8_t), uint16_t(*arg1)(uint16_t)))(uint16_t) {
  return NULL;
}

uint8_t (*T1_static_mut_fn_ptr)(uint8_t, uint8_t) = foo;
uint8_t (*const T1_static_const_fn_ptr_unsafe)(uint8_t, uint8_t) = foo;
void (*const T1_static_const_fn_ptr_unsafe2)(uint8_t) = bar;
void (*const T1_static_const_fn_ptr_unsafe3)(void) = baz;

const uint8_t T1_static_right = 7;
uint8_t (*T1_static_right2)(uint8_t, uint8_t) = foo;

uint32_t (*(*T1_fn_ptr_s)(uint8_t))(uint16_t) = nested;
uint32_t (*(*T1_fn_ptr_s2)(uint8_t(*arg0)(uint8_t), uint16_t(*arg1)(uint16_t)))(uint16_t) = nested2;
