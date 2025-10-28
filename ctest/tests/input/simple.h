#include <stdint.h>

typedef uint8_t Byte;

typedef unsigned long gregset_t[32];

Byte byte = 0x42;

enum Color
{
    RED,
    BLUE,
    GREEN
};

struct Person
{
    const char *name;
    uint8_t age;
    void (*job)(uint8_t, const char *);
    enum Color favorite_color;
};

union Word
{
    uint16_t word;
    Byte byte[2];
};

#define A "abc"
#define C_B "bac"

extern void *calloc(size_t num, size_t size);
extern Byte byte;
