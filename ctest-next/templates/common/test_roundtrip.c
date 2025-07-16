{#- Requires the presence of an `ident` and `c_type` variable. +#}

#ifdef _MSC_VER
// Disable signed/unsigned conversion warnings on MSVC.
// These trigger even if the conversion is explicit.
#  pragma warning(disable:4365)
#endif

// Tests whether the struct/union/alias `x` when passed to C and back to Rust remains unchanged.
// It checks if the size is the same as well as if the padding bytes are all in the correct place.
{{ c_type }} __test_roundtrip_{{ ident }}(
    int32_t rust_size, {{ c_type }} value, int* error, unsigned char* pad
) {
    volatile unsigned char* p = (volatile unsigned char*)&value;
    int size = (int)sizeof({{ c_type }});
    if (size != rust_size) {
        fprintf(
            stderr,
            "size of {{ c_type }} is %d in C and %d in Rust\n",
            (int)size, (int)rust_size
        );
        *error = 1;
        return value;
    }
    int i = 0;
    for (i = 0; i < size; ++i) {
        if (pad[i]) { continue; }
        unsigned char c = (unsigned char)(i % 256);
        c = c == 0? 42 : c;
        if (p[i] != c) {
            *error = 1;
            fprintf(
                stderr,
                "rust[%d] = %d != %d (C): Rust \"{{ ident }}\" -> C\n",
                i, (int)p[i], (int)c
            );
        }
        unsigned char d
            = (unsigned char)(255) - (unsigned char)(i % 256);
        d = d == 0? 42: d;
        p[i] = d;
    }
    return value;
}

#ifdef _MSC_VER
#  pragma warning(default:4365)
#endif