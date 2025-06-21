{% let ty = constant.ty.to_token_stream().to_string() %}
{% let ident = constant.ident() %}

{% if ty == "&str" %}
    #[inline(never)]
    #[allow(non_snake_case)]
    fn const_{{ ident }}() {
        extern "C" {
            #[allow(non_snake_case)]
            fn __test_const_{{ ident }}() -> *const *const u8;
        }
        let val = {{ ident }};
        unsafe {
            let ptr = *__test_const_{{ ident }}();
            let c = ::std::ffi::CStr::from_ptr(ptr as *const _);
            let c = c.to_str().expect("const {{ ident }} not utf8");
            same(val, c, "{{ ident }} string");
        }
    }
{% else %}
    #[allow(non_snake_case)]
    fn const_{{ ident }}() {
        extern "C" {
            #[allow(non_snake_case)]
            fn __test_const_{{ ident }}() -> *const {{ ty }};
        }
        let val = {{ ident }};
        unsafe {
            let ptr1 = &val as *const _ as *const u8;
            let ptr2 = __test_const_{{ ident }}() as *const u8;
            for i in 0..mem::size_of::<{{ ty }}>() {
                let i = i as isize;
                same(*ptr1.offset(i), *ptr2.offset(i),
                        &format!("{{ ident }} value at byte {}", i));
            }
        }
    }
{% endif %}
