{% let c_type = Translator::new().translate_type(constant.ty) %}
{% let ident = constant.ident() %}

static const {{ c_type }} __test_const_{{ ident }}_val = {{ ident }};

const {{ c_type }}* __test_const_{{ ident }}(void) {
    return &__test_const_{{ ident }}_val;
}