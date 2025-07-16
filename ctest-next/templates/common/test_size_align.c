{#- Requires the presence of an `ident` and `c_type` variable. +#}

// Return the size of a type.
uint64_t ctest_size_of__{{ident}}(void) { return sizeof({{c_type}}); }

// Return the alignment of a type.
uint64_t ctest_align_of__{{ident}}(void) { return _Alignof({{c_type}}); }