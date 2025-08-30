#include <stdbool.h>
#include <stddef.h>

#ifdef SUPPRESS_ERROR
#error Expected SUPPRESS_ERROR to not be defined (testing per-file defines)
#endif

typedef unsigned int in6_addr;

#define ON true

extern void *malloc(size_t size);
extern in6_addr in6addr_any;
