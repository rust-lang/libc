#include <stdbool.h>
#include <string.h>
#include <stdint.h>

typedef uint8_t u_int8_t;
typedef uint16_t u_int16_t;
typedef uint32_t u_int32_t;

#include <netinet/in.h>
#include <netinet/ip6.h>
#include <netinet/icmp6.h>

// Since the ICMP6_FILTER macros are macros instead of functions, they aren't
// available to FFI.  libc must reimplement them, which is error-prone.  This
// file provides FFI access to the actual macros so they can be tested against
// the Rust reimplementations.

bool icmp6_filter_willpass(uint8_t typ, const struct icmp6_filter *filt) {
    return ICMP6_FILTER_WILLPASS(typ, filt);
}

bool icmp6_filter_willblock(uint8_t typ, const struct icmp6_filter *filt) {
    return ICMP6_FILTER_WILLBLOCK(typ, filt);
}

void icmp6_filter_setpassall(struct icmp6_filter *filt) {
    ICMP6_FILTER_SETPASSALL(filt);
}

void icmp6_filter_setblockall(struct icmp6_filter *filt) {
    ICMP6_FILTER_SETBLOCKALL(filt);
}

void icmp6_filter_setpass(uint8_t typ, struct icmp6_filter *filt) {
    ICMP6_FILTER_SETPASS(typ, filt);
}

void icmp6_filter_setblock(uint8_t typ, struct icmp6_filter *filt) {
    ICMP6_FILTER_SETBLOCK(typ, filt);
}

