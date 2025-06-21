#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

{% for header in headers %}
    #include <{{ header }}>
{% endfor %}

{% for constant in ffi_items.constants() %}
    {% include "constants/test_constant.c" %}
{% endfor %}