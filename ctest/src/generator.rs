//! Configuration of the test generator.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use askama::Template;
use syn::visit::Visit;
use thiserror::Error;

use crate::ffi_items::FfiItems;
use crate::template::{CTestTemplate, RustTestTemplate};
use crate::translator::translate_primitive_type;
use crate::{
    Const, Field, MapInput, Parameter, Result, Static, Struct, TranslationError, Type, Union,
    VolatileItemKind, expand,
};

/// A function that takes a mappable input and returns its mapping as `Some`, otherwise
/// use the default name if `None`.
type MappedName = Box<dyn Fn(&MapInput) -> Option<String>>;
/// A function that determines whether to skip an item or not.
type Skip = Box<dyn Fn(&MapInput) -> bool>;
/// A function that determines whether a variable or field is volatile.
type VolatileItem = Box<dyn Fn(VolatileItemKind) -> bool>;
/// A function that determines whether a function argument is an array.
type ArrayArg = Box<dyn Fn(crate::Fn, Parameter) -> bool>;
/// A function that determines whether to skip a test, taking in the identifier name.
type SkipTest = Box<dyn Fn(&str) -> bool>;
/// A function that determines whether a type alias is a c enum.
type CEnum = Box<dyn Fn(&str) -> bool>;

/// A builder used to generate a test suite.
#[derive(Default)]
#[expect(missing_debug_implementations)]
pub struct TestGenerator {
    pub(crate) headers: Vec<String>,
    pub(crate) target: Option<String>,
    pub(crate) includes: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
    pub(crate) flags: Vec<String>,
    pub(crate) defines: Vec<(String, Option<String>)>,
    cfg: Vec<(String, Option<String>)>,
    mapped_names: Vec<MappedName>,
    pub(crate) skips: Vec<Skip>,
    pub(crate) verbose_skip: bool,
    pub(crate) volatile_items: Vec<VolatileItem>,
    pub(crate) c_enums: Vec<CEnum>,
    pub(crate) array_arg: Option<ArrayArg>,
    pub(crate) skip_private: bool,
    pub(crate) skip_roundtrip: Option<SkipTest>,
    pub(crate) skip_signededness: Option<SkipTest>,
    pub(crate) skip_fn_ptrcheck: Option<SkipTest>,
}

/// An error that occurs when generating the test files.
#[derive(Debug, Error)]
pub enum GenerationError {
    /// An error that occurs when `rustc -Zunpretty=expand` fails to expand the crate.
    #[error("unable to expand crate {0}: {1}")]
    MacroExpansion(PathBuf, String),
    /// An error that occurs when `syn` is unable to parse the expanded crate due to invalid syntax.
    #[error("unable to parse expanded crate {0}: {1}")]
    RustSyntax(String, String),
    /// An error that occurs when the Rust to C translation fails.
    #[error("unable to prepare template input: {0}")]
    Translation(#[from] TranslationError),
    /// An error that occurs when there are errors in the Rust side of the test template.
    #[error("unable to render Rust template: {0}")]
    RustTemplateRender(askama::Error),
    /// An error that occurs when there are errors in the C side of the test template.
    #[error("unable to render C template: {0}")]
    CTemplateRender(askama::Error),
    #[error("unable to create or write template file: {0}")]
    OsError(std::io::Error),
    #[error("one of {0} environment variable(s) not set")]
    EnvVarNotFound(String),
}

impl TestGenerator {
    /// Creates a new blank test generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a header to be included as part of the generated C file.
    ///
    /// The generated C test will be compiled by a C compiler, and this can be
    /// used to ensure that all the necessary header files are included to test
    /// all FFI definitions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.header("foo.h")
    ///    .header("bar.h");
    /// ```
    pub fn header(&mut self, header: &str) -> &mut Self {
        self.headers.push(header.to_string());
        self
    }

    /// Configures the target to compile C code for.
    ///
    /// Note that for Cargo builds this defaults to `$TARGET` and it's not
    /// necessary to call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.target("x86_64-unknown-linux-gnu");
    /// ```
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }

    /// Set a `--cfg` option with which to expand the Rust FFI crate.
    ///
    /// By default the Rust code is run through expansion to determine what C
    /// APIs are exposed (to allow differences across platforms).
    ///
    /// The `k` argument is the `#[cfg]` value to define, while `v` is the
    /// optional value of `v`:
    ///
    /// * `k == "foo"` and `v == None` makes `#[cfg(foo)]` expand. That is,
    ///   `cfg!(foo)` expands to `true`.
    ///
    /// * `k == "bar"` and `v == Some("baz")` makes `#[cfg(bar = "baz")]`
    ///   expand. That is, `cfg!(bar = "baz")` expands to `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.cfg("foo", None) // cfg!(foo)
    ///    .cfg("bar", Some("baz")); // cfg!(bar = "baz")
    /// ```
    pub fn cfg(&mut self, k: &str, v: Option<&str>) -> &mut Self {
        self.cfg.push((k.to_string(), v.map(|s| s.to_string())));
        self
    }

    /// Add a path to the C compiler header lookup path.
    ///
    /// This is useful for if the C library is installed to a nonstandard
    /// location to ensure that compiling the C file succeeds.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::env;
    /// use std::path::PathBuf;
    ///
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    /// cfg.include(out_dir.join("include"));
    /// ```
    pub fn include<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.includes.push(p.as_ref().to_owned());
        self
    }

    /// Configures the output directory of the generated Rust and C code.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.out_dir("path/to/output");
    /// ```
    pub fn out_dir<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.out_dir = Some(p.as_ref().to_owned());
        self
    }

    /// Non public items are not tested if `skip` is `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_private(true);
    /// ```
    pub fn skip_private(&mut self, skip: bool) -> &mut Self {
        self.skip_private = skip;
        self
    }

    /// Skipped item names are printed to `stderr` if `skip` is `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.verbose_skip(true);
    /// ```
    pub fn verbose_skip(&mut self, skip: bool) -> &mut Self {
        self.verbose_skip = skip;
        self
    }

    /// Indicate that a type alias is actually a C enum.
    ///
    /// # Examples
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.alias_is_c_enum(|e| e == "pid_type");
    /// ```
    pub fn alias_is_c_enum(&mut self, f: impl Fn(&str) -> bool + 'static) -> &mut Self {
        self.c_enums.push(Box::new(f));
        self
    }

    /// Indicate that a struct field should be marked `volatile`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::{TestGenerator, VolatileItemKind};
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.volatile_struct_field(|s, f| {
    ///     s.ident() == "foo_t" && f.ident() == "inner_t"
    /// });
    /// ```
    pub fn volatile_struct_field(
        &mut self,
        f: impl Fn(Struct, Field) -> bool + 'static,
    ) -> &mut Self {
        self.volatile_items.push(Box::new(move |item| {
            if let VolatileItemKind::StructField(s, f_) = item {
                f(s, f_)
            } else {
                false
            }
        }));
        self
    }

    /// Indicate that a static should be marked `volatile`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::{TestGenerator, VolatileItemKind};
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.volatile_static(|s| {
    ///     s.ident() == "foo_t"
    /// });
    /// ```
    pub fn volatile_static(&mut self, f: impl Fn(Static) -> bool + 'static) -> &mut Self {
        self.volatile_items.push(Box::new(move |item| {
            if let VolatileItemKind::Static(s) = item {
                f(s)
            } else {
                false
            }
        }));
        self
    }

    /// Indicate that a function argument should be marked `volatile`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::{TestGenerator, VolatileItemKind};
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.volatile_fn_arg(|f, _p| {
    ///     f.ident() == "size_of_T"
    /// });
    /// ```
    pub fn volatile_fn_arg(
        &mut self,
        f: impl Fn(crate::Fn, Box<Parameter>) -> bool + 'static,
    ) -> &mut Self {
        self.volatile_items.push(Box::new(move |item| {
            if let VolatileItemKind::FnArgument(func, param) = item {
                f(func, param)
            } else {
                false
            }
        }));
        self
    }

    /// Indicate that a function's return type should be marked `volatile`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::{TestGenerator, VolatileItemKind};
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.volatile_fn_return_type(|f| {
    ///     f.ident() == "size_of_T"
    /// });
    /// ```
    pub fn volatile_fn_return_type(
        &mut self,
        f: impl Fn(crate::Fn) -> bool + 'static,
    ) -> &mut Self {
        self.volatile_items.push(Box::new(move |item| {
            if let VolatileItemKind::FnReturnType(func) = item {
                f(func)
            } else {
                false
            }
        }));
        self
    }

    /// Indicate that a function pointer argument is an array.
    ///
    /// This closure should return true if a pointer argument to a function should be generated
    /// with `T foo[]` syntax rather than `T *foo`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.array_arg(|func, arg| {
    ///     match (func.ident(), arg.ident()) {
    ///         ("foo", "bar") => true,
    ///         _ => false,
    /// }});
    /// ```
    pub fn array_arg(&mut self, f: impl Fn(crate::Fn, Parameter) -> bool + 'static) -> &mut Self {
        self.array_arg = Some(Box::new(f));
        self
    }

    /// Configures whether the tests for a struct are emitted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_struct(|s| {
    ///     s.ident().starts_with("foo_")
    /// });
    /// ```
    pub fn skip_struct(&mut self, f: impl Fn(&Struct) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::Struct(struct_) = item {
                f(struct_)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether the tests for a union are emitted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_union(|u| {
    ///     u.ident().starts_with("foo_")
    /// });
    /// ```
    pub fn skip_union(&mut self, f: impl Fn(&Union) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::Union(union_) = item {
                f(union_)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether all tests for a struct field are skipped or not.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_struct_field(|s, f| {
    ///     s.ident() == "foo_t" || (s.ident() == "bar_t" && f.ident() == "bar")
    /// });
    /// ```
    pub fn skip_struct_field(
        &mut self,
        f: impl Fn(&Struct, &Field) -> bool + 'static,
    ) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::StructField(struct_, field) = item {
                f(struct_, field)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether all tests for a union field are skipped or not.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_union_field(|s, f| {
    ///     s.ident() == "foo_t" || (s.ident() == "bar_t" && f.ident() == "bar")
    /// });
    /// ```
    pub fn skip_union_field(&mut self, f: impl Fn(&Union, &Field) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::UnionField(union_, field) = item {
                f(union_, field)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether all tests for a typedef are skipped or not.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_alias(|a| {
    ///     a.ident().starts_with("foo_")
    /// });
    /// ```
    pub fn skip_alias(&mut self, f: impl Fn(&Type) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::Alias(alias) = item {
                f(alias)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether the tests for a constant's value are generated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_const(|s| {
    ///     s.ident().starts_with("FOO_")
    /// });
    /// ```
    pub fn skip_const(&mut self, f: impl Fn(&Const) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::Const(constant) = item {
                f(constant)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether the tests for a static definition are generated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_static(|s| {
    ///     s.ident().starts_with("foo_")
    /// });
    /// ```
    pub fn skip_static(&mut self, f: impl Fn(&Static) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::Static(static_) = item {
                f(static_)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether tests for a function definition are generated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_fn(|s| {
    ///     s.ident().starts_with("foo_")
    /// });
    /// ```
    pub fn skip_fn(&mut self, f: impl Fn(&crate::Fn) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::Fn(func) = item {
                f(func)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether tests for a C enum are generated.
    ///
    /// A C enum consists of a type alias, as well as constants that have the same type. Tests
    /// for both the alias as well as the constants are skipped.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_c_enum(|e| e == "pid_type");
    /// ```
    pub fn skip_c_enum(&mut self, f: impl Fn(&str) -> bool + 'static) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::CEnumType(e) = item {
                f(e)
            } else {
                false
            }
        }));
        self
    }

    /// Add a flag to the C compiler invocation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::env;
    /// use std::path::PathBuf;
    ///
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.flag("-Wno-type-limits");
    /// ```
    pub fn flag(&mut self, flag: &str) -> &mut Self {
        self.flags.push(flag.to_string());
        self
    }

    /// Set a `-D` flag for the C compiler being called.
    ///
    /// This can be used to define various variables to configure how header
    /// files are included or what APIs are exposed from header files.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.define("_GNU_SOURCE", None)
    ///    .define("_WIN32_WINNT", Some("0x8000"));
    /// ```
    pub fn define(&mut self, k: &str, v: Option<&str>) -> &mut Self {
        self.defines
            .push((k.to_string(), v.map(std::string::ToString::to_string)));
        self
    }

    /// Configures whether tests for the type of a field is skipped or not.
    ///
    /// The closure is given a Rust struct as well as a field within that
    /// struct. A flag indicating whether the field's type should be tested is
    /// returned.
    ///
    /// This is useful when, for some reason, a field type is intentionally not
    /// the same in C and Rust.
    ///
    /// By default all field properties are tested.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_struct_field_type(|s, field| {
    ///     s.ident() == "foo_t" || (s.ident() == "bar_t" && field.ident() == "bar")
    /// });
    /// ```
    pub fn skip_struct_field_type(
        &mut self,
        f: impl Fn(&Struct, &Field) -> bool + 'static,
    ) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::StructFieldType(struct_, field) = item {
                f(struct_, field)
            } else {
                false
            }
        }));
        self
    }

    /// Configures whether tests for the type of a field is skipped or not.
    ///
    /// The closure is given a Rust union as well as a field within that
    /// union. A flag indicating whether the field's type should be tested is
    /// returned.
    ///
    /// This is useful when, for some reason, a field type is intentionally not
    /// the same in C and Rust.
    ///
    /// By default all field properties are tested.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_union_field_type(|s, field| {
    ///     s.ident() == "foo_t" || (s.ident() == "bar_t" && field.ident() == "bar")
    /// });
    /// ```
    pub fn skip_union_field_type(
        &mut self,
        f: impl Fn(&Union, &Field) -> bool + 'static,
    ) -> &mut Self {
        self.skips.push(Box::new(move |item| {
            if let MapInput::UnionFieldType(union_, field) = item {
                f(union_, field)
            } else {
                false
            }
        }));
        self
    }

    /// Configures how Rust `const`s names are translated to C.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_constant(|c| {
    ///     (c.ident() == "FOO").then_some("BAR".to_string())
    /// });
    /// ```
    pub fn rename_constant(&mut self, f: impl Fn(&Const) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::Const(c) = item {
                f(c)
            } else {
                None
            }
        }));
        self
    }

    /// Configures how Rust type alias names are translated to C.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_alias(|c| {
    ///     (c.ident() == "sighandler_t").then_some("sig_t".to_string())
    /// });
    /// ```
    pub fn rename_alias(&mut self, f: impl Fn(&Type) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::Alias(t) = item {
                f(t)
            } else {
                None
            }
        }));
        self
    }

    /// Configures how a Rust struct field is translated to a C struct field.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_struct_field(|_s, field| {
    ///     Some(field.ident().replace("foo", "bar"))
    /// });
    /// ```
    pub fn rename_struct_field(
        &mut self,
        f: impl Fn(&Struct, &Field) -> Option<String> + 'static,
    ) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::StructField(s, c) = item {
                f(s, c)
            } else {
                None
            }
        }));
        self
    }

    /// Configures how a Rust union field is translated to a C union field.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_union_field(|_u, field| {
    ///     Some(field.ident().replace("foo", "bar"))
    /// });
    /// ```
    pub fn rename_union_field(
        &mut self,
        f: impl Fn(&Union, &Field) -> Option<String> + 'static,
    ) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::UnionField(u, c) = item {
                f(u, c)
            } else {
                None
            }
        }));
        self
    }

    /// Configures the name of a function in the generated C code.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_fn(|f| Some(format!("{}_c", f.ident())));
    /// ```
    pub fn rename_fn(&mut self, f: impl Fn(&crate::Fn) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::Fn(func) = item {
                f(func)
            } else {
                None
            }
        }));
        self
    }

    /// Configures the name of a static in the generated C code.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_static(|f| Some(format!("{}_c", f.ident())));
    /// ```
    pub fn rename_static(&mut self, f: impl Fn(&Static) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::Static(s) = item {
                f(s)
            } else {
                None
            }
        }));
        self
    }

    /// Configures how a Rust type is translated to a C type.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_type(|ty| {
    ///     Some(format!("{}_t", ty))
    /// });
    /// ```
    pub fn rename_type(&mut self, f: impl Fn(&str) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::Type(ty) = item {
                f(ty)
            } else {
                None
            }
        }));
        self
    }

    /// Configures how a Rust struct type is translated to a C struct type.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_struct_ty(|ty| {
    ///     (ty == "timeval").then(|| format!("{ty}_t"))
    /// });
    /// ```
    pub fn rename_struct_ty(&mut self, f: impl Fn(&str) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::StructType(ty) = item {
                f(ty)
            } else {
                None
            }
        }));
        self
    }

    /// Configures how a Rust union type is translated to a C union type.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.rename_struct_ty(|ty| {
    ///     (ty == "T1Union").then(|| format!("__{ty}"))
    /// });
    /// ```
    pub fn rename_union_ty(&mut self, f: impl Fn(&str) -> Option<String> + 'static) -> &mut Self {
        self.mapped_names.push(Box::new(move |item| {
            if let MapInput::UnionType(ty) = item {
                f(ty)
            } else {
                None
            }
        }));
        self
    }

    // FIXME(ctest): should arrays be handled differently?

    /// Configures whether the ABI roundtrip tests for a type are emitted.
    ///
    /// The closure is passed the name of a Rust type and returns whether the
    /// tests are generated.
    ///
    /// By default all types undergo ABI roundtrip tests. Arrays cannot undergo
    /// an ABI roundtrip because they cannot be returned by C functions, and
    /// have to be manually skipped here.
    ///
    /// # Examples
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_roundtrip(|s| {
    ///     s.starts_with("foo_")
    /// });
    /// ```
    pub fn skip_roundtrip(&mut self, f: impl Fn(&str) -> bool + 'static) -> &mut Self {
        self.skip_roundtrip = Some(Box::new(f));
        self
    }

    /// Configures whether a type's signededness is tested or not.
    ///
    /// The closure is given the name of a Rust type, and returns whether the
    /// type should be tested as having the right sign (positive or negative).
    ///
    /// By default all signededness checks are performed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_signededness(|s| {
    ///     s.starts_with("foo_")
    /// });
    /// ```
    pub fn skip_signededness(&mut self, f: impl Fn(&str) -> bool + 'static) -> &mut Self {
        self.skip_signededness = Some(Box::new(f));
        self
    }

    /// Configures whether tests for a function pointer's value are generated.
    ///
    /// The closure is given a Rust FFI function and returns whether
    /// the test will be generated.
    ///
    /// By default generated tests will ensure that the function pointer in C
    /// corresponds to the same function pointer in Rust. This can often
    /// uncover subtle symbol naming issues where a header file is referenced
    /// through the C identifier `foo` but the underlying symbol is mapped to
    /// something like `__foo_compat`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_fn_ptrcheck(|name| name == "T1p");
    /// ```
    pub fn skip_fn_ptrcheck(&mut self, f: impl Fn(&str) -> bool + 'static) -> &mut Self {
        self.skip_fn_ptrcheck = Some(Box::new(f));
        self
    }

    /// Generate the Rust and C testing files.
    ///
    /// Returns the path to the generated file.
    pub fn generate_files(
        &mut self,
        crate_path: impl AsRef<Path>,
        output_file_path: impl AsRef<Path>,
    ) -> Result<PathBuf, GenerationError> {
        let expanded = expand(&crate_path, &self.cfg).map_err(|e| {
            GenerationError::MacroExpansion(crate_path.as_ref().to_path_buf(), e.to_string())
        })?;
        let ast = syn::parse_file(&expanded)
            .map_err(|e| GenerationError::RustSyntax(expanded, e.to_string()))?;

        let mut ffi_items = FfiItems::new();
        ffi_items.visit_file(&ast);

        let output_directory = self
            .out_dir
            .clone()
            .or_else(|| env::var("OUT_DIR").ok().map(Into::into))
            .ok_or(GenerationError::EnvVarNotFound("OUT_DIR".to_string()))?;
        let output_file_path = output_directory.join(output_file_path);

        let ensure_trailing_newline = |s: &mut String| {
            s.truncate(s.trim_end().len());
            s.push('\n');
        };

        let mut rust_file = RustTestTemplate::new(&ffi_items, self)?
            .render()
            .map_err(GenerationError::RustTemplateRender)?;
        ensure_trailing_newline(&mut rust_file);

        // Generate the Rust side of the tests.
        File::create(output_file_path.with_extension("rs"))
            .map_err(GenerationError::OsError)?
            .write_all(rust_file.as_bytes())
            .map_err(GenerationError::OsError)?;

        let mut c_file = CTestTemplate::new(&ffi_items, self)?
            .render()
            .map_err(GenerationError::CTemplateRender)?;
        ensure_trailing_newline(&mut c_file);

        // Generate the C/Cxx side of the tests.
        let c_output_path = output_file_path.with_extension("c");
        File::create(&c_output_path)
            .map_err(GenerationError::OsError)?
            .write_all(c_file.as_bytes())
            .map_err(GenerationError::OsError)?;

        Ok(output_file_path)
    }

    /// Maps Rust identifiers or types to C counterparts, or defaults to the original name.
    pub(crate) fn rty_to_cty<'a>(&self, item: impl Into<MapInput<'a>>) -> String {
        let item = item.into();
        if let Some(mapped) = self.mapped_names.iter().find_map(|f| f(&item)) {
            return mapped;
        }
        match item {
            MapInput::Const(c) => c.ident().to_string(),
            MapInput::Fn(f) => f.ident().to_string(),
            MapInput::Static(s) => s.ident().to_string(),
            MapInput::Struct(s) => s.ident().to_string(),
            MapInput::Union(u) => u.ident().to_string(),
            MapInput::Alias(t) => t.ident().to_string(),
            MapInput::StructField(_, f) => f.ident().to_string(),
            MapInput::UnionField(_, f) => f.ident().to_string(),
            MapInput::StructType(ty) => format!("struct {ty}"),
            MapInput::UnionType(ty) => format!("union {ty}"),
            MapInput::CEnumType(ty) => format!("enum {ty}"),
            MapInput::StructFieldType(_, f) => f.ident().to_string(),
            MapInput::UnionFieldType(_, f) => f.ident().to_string(),
            MapInput::Type(ty) => translate_primitive_type(ty),
        }
    }
}
