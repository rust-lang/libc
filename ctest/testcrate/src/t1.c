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
