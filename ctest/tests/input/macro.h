#include <stdint.h>

#ifndef SUPPRESS_ERROR
#error Expected SUPPRESS_ERROR to be defined (testing per-file defines)
#endif

struct VecU8
{
    uint8_t x;
    uint8_t y;
};

struct VecU16
{
    uint16_t x;
    uint16_t y;
};
