#include <stdint.h>

typedef uint8_t Byte;

typedef unsigned long gregset_t[32];

Byte byte = 0x42;

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

enum Color
{
    RED,
    BLUE,
    GREEN
};

extern void *calloc(size_t num, size_t size);
extern Byte byte;
