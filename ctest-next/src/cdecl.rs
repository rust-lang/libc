//! Conversion from a basic C type tree to string declarations.

use std::fmt::Write;

type BoxStr = Box<str>;

#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Constness {
    Const,
    Mut,
}
use Constness::{Const, Mut};

/// A basic representation of C's types.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Debug)]
pub(crate) enum CTy {
    /// `int`, `struct foo`, etc. There is only ever one basic type per decl.
    Named {
        name: BoxStr,
        constness: Constness,
    },
    Ptr {
        ty: Box<Self>,
        constness: Constness,
    },
    Array {
        ty: Box<Self>,
        len: Option<BoxStr>,
    },
    /// Functions as a declaration. If a function pointer is needed, it must be composed with `Ptr`.
    Fn {
        args: Vec<Self>,
        ret: Box<Self>,
    },
}

impl CTy {
    /// Validate that we aren't returning an array or a function without indirection, which isn't
    /// allowed in C.
    fn check_ret_ty(&self) -> Result<(), InvalidReturn> {
        let Self::Fn { ret, .. } = self else {
            return Ok(());
        };
        match **ret {
            CTy::Named { .. } | CTy::Ptr { .. } => Ok(()),
            CTy::Array { .. } | CTy::Fn { .. } => Err(InvalidReturn),
        }
    }

    /// True if this type is added to the RHS in a cdecl (arrays, function pointers).
    fn is_rhs(&self) -> bool {
        match self {
            CTy::Named { .. } | CTy::Ptr { .. } => false,
            CTy::Array { .. } | CTy::Fn { .. } => true,
        }
    }

    /// Add parentheses if we are adding something with lower precedence (on the left) after
    /// something with higher precedence (on the right).
    fn parens_if_needed(&self, s: &mut String, prev: Option<&CTy>) {
        let Some(prev) = prev else {
            return;
        };
        if self.is_rhs() && !prev.is_rhs() {
            s.insert(0, '(');
            s.push(')');
        }
    }
}

/// Attempting to return an array or function pointer.
#[derive(Clone, Copy, Debug)]
pub(crate) struct InvalidReturn;

/// Create a C declaration for a type.
///
/// Given a type `cty` (e.g. array of pointers to int) and a `name` (e.g. "foo"), turn `name` into
/// a valid declaration for that type (e.g. `int *foo[]`). `name` is taken as an owned string by
/// value to allow reusing allocations.
///
/// If needed, `name` can be empty (e.g. for function arguments).
#[cfg_attr(not(test), expect(dead_code))]
pub(crate) fn cdecl(cty: &CTy, mut name: String) -> Result<String, InvalidReturn> {
    cdecl_impl(cty, &mut name, None)?;
    Ok(name)
}

/// C declarations are read from the declaration out, left to right, switching directions when a `)`
/// is hit. So, to reverse this, we build from the declaration out adding `*`, `[]`, or `()` on
/// their natural side, and adding `(...)` when we need to something to the left after having added
/// something to the right.
///
/// Helpful description of the rules:
/// <https://web.archive.org/web/20210523053011/http://cseweb.ucsd.edu/~ricko/rt_lt.rule.html>.
fn cdecl_impl(cty: &CTy, s: &mut String, prev: Option<&CTy>) -> Result<(), InvalidReturn> {
    cty.check_ret_ty()?;
    cty.parens_if_needed(s, prev);
    match cty {
        CTy::Named { name, constness } => {
            let sp = if s.is_empty() { "" } else { " " };
            let c = if *constness == Const { "const " } else { "" };
            let to_insert = format!("{c}{name}{sp}");
            s.insert_str(0, &to_insert);
        }
        CTy::Ptr { ty, constness } => {
            match constness {
                Const => s.insert_str(0, "*const "),
                Mut => s.insert(0, '*'),
            }
            cdecl_impl(ty, s, Some(cty))?;
        }
        CTy::Array { ty, len } => {
            let len = len.as_ref().map(BoxStr::as_ref).unwrap_or_default();
            write!(s, "[{len}]").unwrap();
            cdecl_impl(ty, s, Some(cty))?;
        }
        CTy::Fn { args, ret } => {
            // Functions act as a RHS `(args...)`, then the return type is applied as normal.
            let mut tmp = String::new();
            s.push('(');
            let mut args = args.iter().peekable();
            while let Some(arg) = args.next() {
                cdecl_impl(arg, &mut tmp, None)?; // each arg is an unnamed decl
                s.push_str(&tmp);
                if args.peek().is_some() {
                    s.push_str(", ");
                    tmp.clear();
                }
            }
            s.push(')');
            cdecl_impl(ret, s, Some(cty))?;
        }
    }
    Ok(())
}

/// Checked with <https://cdecl.org/>.
#[cfg(test)]
mod tests {
    use super::*;

    /// Check that a decl named "foo" matches `expected`.
    #[track_caller]
    fn assert_decl(ty: &CTy, expected: &str) {
        assert_eq!(cdecl(ty, "foo".to_owned()).unwrap(), expected);
    }

    /* Helpful constructors */

    fn mut_int() -> CTy {
        named("int", Mut)
    }

    fn const_int() -> CTy {
        named("int", Const)
    }

    fn named(name: &str, constness: Constness) -> CTy {
        CTy::Named {
            name: name.into(),
            constness,
        }
    }

    fn ptr(inner: CTy, constness: Constness) -> CTy {
        CTy::Ptr {
            ty: Box::new(inner),
            constness,
        }
    }

    fn array(inner: CTy, len: Option<&str>) -> CTy {
        CTy::Array {
            ty: Box::new(inner),
            len: len.map(Into::into),
        }
    }

    /// Function type (not a pointer)
    fn func(args: Vec<CTy>, ret: CTy) -> CTy {
        CTy::Fn {
            args,
            ret: Box::new(ret),
        }
    }

    /// Function pointer
    fn func_ptr(args: Vec<CTy>, ret: CTy) -> CTy {
        ptr(
            CTy::Fn {
                args,
                ret: Box::new(ret),
            },
            Mut,
        )
    }

    #[test]
    fn basic() {
        assert_decl(&const_int(), "const int foo");
        assert_decl(&mut_int(), "int foo");
    }

    #[test]
    fn test_ptr() {
        assert_decl(&ptr(const_int(), Mut), "const int *foo");
        assert_decl(&ptr(const_int(), Const), "const int *const foo");
        assert_decl(&ptr(mut_int(), Mut), "int *foo");
        assert_decl(&ptr(mut_int(), Const), "int *const foo");
        assert_decl(&ptr(ptr(mut_int(), Mut), Mut), "int **foo");
        assert_decl(&ptr(ptr(mut_int(), Const), Mut), "int *const *foo");
        assert_decl(&ptr(ptr(mut_int(), Mut), Const), "int **const foo");
        assert_decl(&ptr(ptr(mut_int(), Const), Const), "int *const *const foo");
        assert_decl(
            &ptr(ptr(const_int(), Const), Const),
            "const int *const *const foo",
        );
    }

    #[test]
    fn test_array() {
        assert_decl(&array(const_int(), None), "const int foo[]");
        assert_decl(&array(const_int(), Some("20")), "const int foo[20]");
        let ty = array(
            array(
                array(
                    array(
                        array(array(mut_int(), Some("BLASTOFF")), Some("1")),
                        Some("2"),
                    ),
                    Some("3"),
                ),
                Some("4"),
            ),
            Some("5"),
        );
        assert_decl(&ty, "int foo[5][4][3][2][1][BLASTOFF]");
    }

    #[test]
    fn test_func() {
        // Function types (not pointers)
        assert_decl(&func(vec![], mut_int()), "int foo()");
        assert_decl(
            &func(vec![const_int()], const_int()),
            "const int foo(const int)",
        );
        assert_decl(
            &func(vec![const_int(), mut_int()], mut_int()),
            "int foo(const int, int)",
        );
    }

    #[test]
    fn test_func_invalid_ret() {
        // Can't return an array
        assert!(cdecl(&func(vec![], array(mut_int(), None)), "foo".to_owned(),).is_err(),);
        // Can't return a function
        assert!(cdecl(&func(vec![], func(vec![], mut_int()),), "foo".to_owned(),).is_err(),);
    }

    #[test]
    fn test_func_ptr() {
        assert_decl(&func_ptr(vec![mut_int()], mut_int()), "int (*foo)(int)");
        assert_decl(&func_ptr(vec![mut_int()], mut_int()), "int (*foo)(int)");
        assert_decl(&array(const_int(), Some("20")), "const int foo[20]");

        // declare foo as pointer to function (pointer to function (pointer to function (pointer
        // to function (char) returning char) returning pointer to function (short) returning short) returning
        // pointer to function (long) returning long, pointer to function (long long) returning long long)
        // returning pointer to function (int) returning int
        let make_func_ptr = |ty: &str| func_ptr(vec![named(ty, Mut)], named(ty, Mut));
        let inception = func_ptr(
            vec![
                func_ptr(
                    vec![func_ptr(
                        vec![make_func_ptr("char")],
                        make_func_ptr("short"),
                    )],
                    make_func_ptr("long"),
                ),
                make_func_ptr("long long"),
            ],
            make_func_ptr("int"),
        );
        assert_decl(
            &inception,
            "int (*(*foo)(long (*(*)(short (*(*)(\
                char (*)(char)))(short)))(long), \
                long long (*)(long long)\
            ))(int)",
        );
    }

    /// Check that parens are added where needed
    #[test]
    fn test_precedence() {
        // pointer to an array of ints
        assert_decl(&ptr(array(mut_int(), None), Mut), "int (*foo)[]");
        // array of pointers of ints
        assert_decl(&array(ptr(mut_int(), Mut), None), "int *foo[]");
        // pointer to a function returning an int
        assert_decl(&func_ptr(vec![], named("int", Mut)), "int (*foo)()");
    }

    #[test]
    fn test_unnamed() {
        // Function args are usually unnamed
        assert_eq!(cdecl(&mut_int(), String::new()).unwrap(), "int");
        assert_eq!(
            cdecl(&ptr(array(mut_int(), None), Mut), String::new()).unwrap(),
            "int (*)[]"
        );
        assert_eq!(
            cdecl(&array(ptr(mut_int(), Mut), None), String::new()).unwrap(),
            "int *[]"
        );
    }

    #[test]
    fn test_compose() {
        assert_decl(&array(ptr(const_int(), Mut), None), "const int *foo[]");
        let ty = ptr(
            func(
                vec![
                    array(named("int", Mut), Some("ARR_LEN")),
                    ptr(named("short", Const), Mut),
                ],
                ptr(named("long", Const), Mut),
            ),
            Mut,
        );
        assert_decl(&ty, "const long *(*foo)(int [ARR_LEN], const short *)");

        // function returning a pointer to a function returning an int
        let ty = func(vec![], func_ptr(vec![], named("int", Mut)));
        assert_decl(&ty, "int (*foo())()");

        let ty = array(
            func_ptr(vec![], ptr(array(named("char", Mut), Some("5")), Mut)),
            Some("3"),
        );
        assert_decl(&ty, "char (*(*foo[3])())[5]");

        // declare foo as pointer to function (pointer to const void) returning pointer to array
        // 3 of int
        let ty = func_ptr(
            vec![ptr(named("void", Const), Mut)],
            ptr(array(named("int", Mut), Some("3")), Mut),
        );
        assert_decl(&ty, "int (*(*foo)(const void *))[3]");
    }
}
