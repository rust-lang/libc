#include <stdint.h>

#ifdef UNSIGNED
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
#else
struct VecI8
{
    int8_t x;
    int8_t y;
};

struct VecI16
{
    int16_t x;
    int16_t y;
};
#endif