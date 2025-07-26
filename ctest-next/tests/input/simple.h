#include <stdint.h>

typedef uint8_t Byte;

struct Person
{
    const char *name;
    uint8_t age;
    void (*job)(uint8_t, const char *);
};

union Word
{
    uint16_t word;
    Byte byte[2];
};

#define A "abc"
#define C_B "bac"
