/// A macro for defining #[cfg] if-else statements.
///
/// This is similar to the `if/elif` C preprocessor macro by allowing definition
/// of a cascade of `#[cfg]` cases, emitting the implementation which matches
/// first.
///
/// This allows you to conveniently provide a long list #[cfg]'d blocks of code
/// without having to rewrite each clause multiple times.
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        cfg_if! {
            @__items
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    };

    // match if/else chains lacking a final `else`
    (
        if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
        $(
            else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
        )*
    ) => {
        cfg_if! {
            @__items
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            ( () () ),
        }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the negated `cfg`s in a list at the beginning and after the
    // semicolon is all the remaining items
    (@__items ($($not:meta,)*) ; ) => {};
    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ),
     $($rest:tt)*) => {
        // Emit all items within one block, applying an appropriate #[cfg]. The
        // #[cfg] will require all `$m` matchers specified and must also negate
        // all previous matchers.
        cfg_if! { @__apply cfg(all($($m,)* not(any($($not),*)))), $($it)* }

        // Recurse to emit all other items in `$rest`, and when we do so add all
        // our `$m` matchers to the list of `$not` matchers as future emissions
        // will have to negate everything we just matched as well.
        cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
    };

    // Internal macro to Apply a cfg attribute to a list of items
    (@__apply $m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    };
}

/// Create an internal crate prelude with `core` reexports and common types.
macro_rules! prelude {
    () => {
        mod types;

        /// Frequently-used types that are available on all platforms
        ///
        /// We need to reexport the core types so this works with `rust-dep-of-std`.
        mod prelude {
            // Exports from `core`
            #[allow(unused_imports)]
            pub(crate) use ::core::clone::Clone;
            #[allow(unused_imports)]
            pub(crate) use ::core::default::Default;
            #[allow(unused_imports)]
            pub(crate) use ::core::marker::{Copy, Send, Sync};
            #[allow(unused_imports)]
            pub(crate) use ::core::option::Option;
            #[allow(unused_imports)]
            pub(crate) use ::core::prelude::v1::derive;
            #[allow(unused_imports)]
            pub(crate) use ::core::{fmt, hash, iter, mem};
            #[allow(unused_imports)]
            pub(crate) use mem::{align_of, align_of_val, size_of, size_of_val};

            #[allow(unused_imports)]
            pub(crate) use crate::types::{CEnumRepr, Padding};
            // Commonly used types defined in this crate
            #[allow(unused_imports)]
            pub(crate) use crate::{
                c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint,
                c_ulong, c_ulonglong, c_ushort, c_void, intptr_t, size_t, ssize_t, uintptr_t,
            };
        }
    };
}

/// Implement `Clone` and `Copy` for a struct, as well as `Debug`, `Eq`, `Hash`, and
/// `PartialEq` if the `extra_traits` feature is enabled.
///
/// By default, it will mark a struct as `non_exhaustive`. To opt out, use the attribute
/// `#[@not_non_exhaustive]` as the first attribute of the struct.
///
/// Use [`s_no_extra_traits`] for structs where the `extra_traits` feature does not
/// make sense, and for unions.
macro_rules! s {
    ($(
        $(#[@$not_non_exhaustive:ident])*
        $(#[$attr:meta])*
        pub $t:ident $i:ident { $($field:tt)* }
    )*) => ($(
        s!(it: $(#[@$not_non_exhaustive])* $(#[$attr])* pub $t $i { $($field)* });
    )*);

    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        compile_error!("unions cannot derive extra traits, use s_no_extra_traits instead");
    );

    (it: #[@not_non_exhaustive] $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[cfg_attr(
                feature = "extra_traits",
                ::core::prelude::v1::derive(Debug, Eq, Hash, PartialEq)
            )]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            #[allow(deprecated)]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
    );

    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[cfg_attr(
                feature = "extra_traits",
                ::core::prelude::v1::derive(Debug, Eq, Hash, PartialEq)
            )]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            #[allow(deprecated)]
            #[non_exhaustive]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
    );
}

/// Implement `Clone` and `Copy` for a tuple struct, as well as `Debug`, `Eq`, `Hash`,
/// and `PartialEq` if the `extra_traits` feature is enabled.
///
/// By default, it will mark a struct as `non_exhaustive`. To opt out, use the attribute
/// `#[@not_non_exhaustive]` as the first attribute of the struct.
///
/// This is the same as [`s`] but works for tuple structs.
macro_rules! s_paren {
    ($(
        $(#[$attr:meta])*
        pub struct $i:ident ( $($field:tt)* );
    )*) => ($(
        __item! {
            #[cfg_attr(
                feature = "extra_traits",
                ::core::prelude::v1::derive(Debug, Eq, Hash, PartialEq)
            )]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            $(#[$attr])*
            pub struct $i ( $($field)* );
        }
    )*);
}

/// Implement `Clone` and `Copy` for a struct with no `extra_traits` feature, as well as `Debug`
/// with `extra_traits` since that can always be derived.
///
/// Most items will prefer to use [`s`].
macro_rules! s_no_extra_traits {
    ($(
        $(#[@$not_non_exhaustive:ident])*
        $(#[$attr:meta])*
        pub $t:ident $i:ident { $($field:tt)* }
    )*) => ($(
        s_no_extra_traits!(it: $(#[@$not_non_exhaustive])* $(#[$attr])* pub $t $i { $($field)* });
    )*);

    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            $(#[$attr])*
            pub union $i { $($field)* }
        }

        #[cfg(feature = "extra_traits")]
        impl ::core::fmt::Debug for $i {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($i)).finish_non_exhaustive()
            }
        }
    );

    (it: #[@not_non_exhaustive] $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            #[cfg_attr(feature = "extra_traits", ::core::prelude::v1::derive(Debug))]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
    );

    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            #[cfg_attr(feature = "extra_traits", ::core::prelude::v1::derive(Debug))]
            #[non_exhaustive]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
    );
}

/// Specify that an enum should have no traits that aren't specified in the macro
/// invocation, i.e. no `Clone` or `Copy`.
macro_rules! missing {
    ($(
        $(#[$attr:meta])*
        pub enum $i:ident {}
    )*) => ($(
        $(#[$attr])*
        #[allow(missing_copy_implementations)]
        pub enum $i { }
    )*);
}

/// Implement `Clone` and `Copy` for an enum, as well as `Debug`, `Eq`, `Hash`, and
/// `PartialEq` if the `extra_traits` feature is enabled.
// FIXME(#4419): Replace all uses of `e!` with `c_enum!`
macro_rules! e {
    ($(
        $(#[$attr:meta])*
        pub enum $i:ident { $($field:tt)* }
    )*) => ($(
        __item! {
            #[cfg_attr(
                feature = "extra_traits",
                ::core::prelude::v1::derive(Debug, Eq, Hash, PartialEq)
            )]
            #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
            $(#[$attr])*
            pub enum $i { $($field)* }
        }
    )*);
}

/// Represent a C enum as Rust constants and a type.
///
/// C enums can't soundly be mapped to Rust enums since C enums are allowed to have duplicates or
/// unlisted values, but this is UB in Rust. This enum doesn't implement any traits, its main
/// purpose is to calculate the correct enum values.
///
/// See <https://github.com/rust-lang/libc/issues/4419> for more.
macro_rules! c_enum {
    ($(
        $(#[repr($repr:ty)])?
        pub enum $ty_name:ident {
            $($variant:ident $(= $value:expr)?,)+
        }
    )+) => {
        $(c_enum!(@expand;
            $(#[repr($repr)])?
            pub enum $ty_name {
                $($variant $(= $value)?,)+
            }
        );)+
    };

    (@expand;
        $(#[repr($repr:ty)])?
        pub enum $ty_name:ident {
            $($variant:ident $(= $value:expr)?,)+
        }
    ) => {
        pub type $ty_name = c_enum!(@ty $($repr)?);
        c_enum!(@one; $ty_name; 0; $($variant $(= $value)?,)+);
    };

    // Matcher for a single variant
    (@one; $_ty_name:ident; $_idx:expr;) => {};
    (
        @one; $ty_name:ident; $default_val:expr;
        $variant:ident $(= $value:expr)?,
        $($tail:tt)*
    ) => {
        pub const $variant: $ty_name = {
            #[allow(unused_variables)]
            let r = $default_val;
            $(let r = $value;)?
            r
        };

        // The next value is always one more than the previous value, unless
        // set explicitly.
        c_enum!(@one; $ty_name; $variant + 1; $($tail)*);
    };

    // Use a specific type if provided, otherwise default to `CEnumRepr`
    (@ty $repr:ty) => { $repr };
    (@ty) => { $crate::prelude::CEnumRepr };
}

// This is a pretty horrible hack to allow us to conditionally mark some functions as 'const',
// without requiring users of this macro to care "libc_const_extern_fn".
//
// When 'libc_const_extern_fn' is enabled, we emit the captured 'const' keyword in the expanded
// function.
//
// When 'libc_const_extern_fn' is disabled, we always emit a plain 'pub unsafe extern fn'.
// Note that the expression matched by the macro is exactly the same - this allows
// users of this macro to work whether or not 'libc_const_extern_fn' is enabled
//
// Unfortunately, we need to duplicate most of this macro between the 'cfg_if' blocks.
// This is because 'const unsafe extern fn' won't even parse on older compilers,
// so we need to avoid emitting it at all of 'libc_const_extern_fn'.
//
// Specifically, moving the 'cfg_if' into the macro body will *not* work. Doing so would cause the
// '#[cfg(libc_const_extern_fn)]' to be emitted into user code. The 'cfg' gate will not stop Rust
// from trying to parse the 'pub const unsafe extern fn', so users would get a compiler error even
// when the 'libc_const_extern_fn' feature is disabled.

// FIXME(ctest): ctest can't handle `const extern` functions, we should be able to remove this
// cfg completely.
// FIXME(ctest): ctest can't handle `$(,)?` so we use `$(,)*` which isn't quite correct.
cfg_if! {
    if #[cfg(libc_const_extern_fn)] {
        /// Define an `unsafe` function that is const as long as `libc_const_extern_fn` is enabled.
        macro_rules! f {
            ($(
                $(#[$attr:meta])*
                pub $({$constness:ident})* fn $i:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty
                    $body:block
            )*) => ($(
                #[inline]
                $(#[$attr])*
                pub $($constness)* unsafe extern "C" fn $i($($arg: $argty),*) -> $ret
                    $body
            )*)
        }

        /// Define a safe function that is const as long as `libc_const_extern_fn` is enabled.
        macro_rules! safe_f {
            ($(
                $(#[$attr:meta])*
                pub $({$constness:ident})* fn $i:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty
                    $body:block
            )*) => ($(
                #[inline]
                $(#[$attr])*
                pub $($constness)* extern "C" fn $i($($arg: $argty),*) -> $ret
                    $body
            )*)
        }

        /// A nonpublic function that is const as long as `libc_const_extern_fn` is enabled.
        macro_rules! const_fn {
            ($(
                $(#[$attr:meta])*
                $({$constness:ident})* fn $i:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty
                    $body:block
            )*) => ($(
                #[inline]
                $(#[$attr])*
                $($constness)* fn $i($($arg: $argty),*) -> $ret
                    $body
            )*)
        }
    } else {
        /// Define an `unsafe` function that is const as long as `libc_const_extern_fn` is enabled.
        macro_rules! f {
            ($(
                $(#[$attr:meta])*
                pub $({$constness:ident})* fn $i:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty
                    $body:block
            )*) => ($(
                #[inline]
                $(#[$attr])*
                pub unsafe extern "C" fn $i($($arg: $argty),*) -> $ret
                    $body
            )*)
        }

        /// Define a safe function that is const as long as `libc_const_extern_fn` is enabled.
        macro_rules! safe_f {
            ($(
                $(#[$attr:meta])*
                pub $({$constness:ident})* fn $i:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty
                    $body:block
            )*) => ($(
                #[inline]
                $(#[$attr])*
                pub extern "C" fn $i($($arg: $argty),*) -> $ret
                    $body
            )*)
        }

        /// A nonpublic function that is const as long as `libc_const_extern_fn` is enabled.
        macro_rules! const_fn {
            ($(
                $(#[$attr:meta])*
                $({$constness:ident})* fn $i:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty
                    $body:block
            )*) => ($(
                #[inline]
                $(#[$attr])*
                fn $i($($arg: $argty),*) -> $ret
                    $body
            )*)
        }
    }
}

macro_rules! __item {
    ($i:item) => {
        $i
    };
}

#[cfg(test)]
mod tests {
    use crate::types::CEnumRepr;

    #[test]
    fn c_enumbasic() {
        // By default, variants get sequential values.
        c_enum! {
            pub enum e {
                VAR0,
                VAR1,
                VAR2,
            }
        }

        assert_eq!(VAR0, 0 as CEnumRepr);
        assert_eq!(VAR1, 1 as CEnumRepr);
        assert_eq!(VAR2, 2 as CEnumRepr);
    }

    #[test]
    fn c_enumrepr() {
        // By default, variants get sequential values.
        c_enum! {
            #[repr(u16)]
            pub enum e {
                VAR0,
            }
        }

        assert_eq!(VAR0, 0_u16);
    }

    #[test]
    fn c_enumset_value() {
        // Setting an explicit value resets the count.
        c_enum! {
            pub enum e {
                VAR2 = 2,
                VAR3,
                VAR4,
            }
        }

        assert_eq!(VAR2, 2 as CEnumRepr);
        assert_eq!(VAR3, 3 as CEnumRepr);
        assert_eq!(VAR4, 4 as CEnumRepr);
    }

    #[test]
    fn c_enummultiple_set_value() {
        // C enums always take one more than the previous value, unless set to a specific
        // value. Duplicates are allowed.
        c_enum! {
            pub enum e {
                VAR0,
                VAR2_0 = 2,
                VAR3_0,
                VAR4_0,
                VAR2_1 = 2,
                VAR3_1,
                VAR4_1,
            }
        }

        assert_eq!(VAR0, 0 as CEnumRepr);
        assert_eq!(VAR2_0, 2 as CEnumRepr);
        assert_eq!(VAR3_0, 3 as CEnumRepr);
        assert_eq!(VAR4_0, 4 as CEnumRepr);
        assert_eq!(VAR2_1, 2 as CEnumRepr);
        assert_eq!(VAR3_1, 3 as CEnumRepr);
        assert_eq!(VAR4_1, 4 as CEnumRepr);
    }
}
