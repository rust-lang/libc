//! # ctest - an FFI binding validator
//!
//! This library is intended to be used as a build dependency in a separate
//! project from the main repo to generate tests which can be used to validate
//! FFI bindings in Rust against the headers from which they come from.
//!
//! For example usage, see the [main `README.md`][project] for how to set it
//! up.
//!
//! [project]: https://github.com/alexcrichton/ctest

#![allow(deprecated)] // connect => join in 1.3
#![deny(missing_docs)]

extern crate gcc;
extern crate syntex_syntax as syntax;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use syntax::abi::Abi;
use syntax::ast;
use syntax::attr::{self, ReprAttr};
use syntax::errors::Handler as SpanHandler;
use syntax::ext::base::{ExtCtxt, SyntaxExtension};
use syntax::ext::expand;
use syntax::parse::token::{intern, InternedString};
use syntax::parse::{self, ParseSess};
use syntax::visit::{self, Visitor};

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

/// A builder used to generate a test suite.
///
/// This builder has a number of configuration options which modify how the
/// generated tests are emitted, and it is also the main entry point for parsing
/// an FFI header crate for definitions.
pub struct TestGenerator {
    headers: Vec<String>,
    includes: Vec<PathBuf>,
    flags: Vec<String>,
    target: Option<String>,
    out_dir: Option<PathBuf>,
    defines: Vec<(String, Option<String>)>,
    cfg: Vec<(String, Option<String>)>,
    skip_fn: Box<Fn(&str) -> bool>,
    skip_fn_ptrcheck: Box<Fn(&str) -> bool>,
    skip_field: Box<Fn(&str, &str) -> bool>,
    skip_field_type: Box<Fn(&str, &str) -> bool>,
    skip_const: Box<Fn(&str) -> bool>,
    skip_signededness: Box<Fn(&str) -> bool>,
    skip_type: Box<Fn(&str) -> bool>,
    skip_struct: Box<Fn(&str) -> bool>,
    field_name: Box<Fn(&str, &str) -> String>,
    type_name: Box<Fn(&str, bool) -> String>,
    fn_cname: Box<Fn(&str, Option<&str>) -> String>,
}

struct StructFinder {
    structs: HashSet<String>,
}

struct Generator<'a> {
    target: &'a str,
    rust: Box<Write>,
    c: Box<Write>,
    sh: &'a SpanHandler,
    structs: HashSet<String>,
    files: HashSet<String>,
    abi: Abi,
    tests: Vec<String>,
    sess: &'a ParseSess,
    opts: &'a TestGenerator,
}

impl TestGenerator {
    /// Creates a new blank test generator.
    ///
    /// This won't actually be that useful until functions like `header` are
    /// called, but the main "finalization method" is the `generate` method.
    pub fn new() -> TestGenerator {
        TestGenerator {
            headers: Vec::new(),
            includes: Vec::new(),
            flags: Vec::new(),
            target: None,
            out_dir: None,
            defines: Vec::new(),
            cfg: Vec::new(),
            skip_fn: Box::new(|_| false),
            skip_fn_ptrcheck: Box::new(|_| false),
            skip_const: Box::new(|_| false),
            skip_signededness: Box::new(|_| false),
            skip_type: Box::new(|_| false),
            skip_struct: Box::new(|_| false),
            field_name: Box::new(|_, f| f.to_string()),
            skip_field: Box::new(|_, _| false),
            skip_field_type: Box::new(|_, _| false),
            fn_cname: Box::new(|a, _| a.to_string()),
            type_name: Box::new(|f, is_struct| {
                if is_struct {format!("struct {}", f)} else {f.to_string()}
            }),
        }
    }

    /// Add a header to be included as part of the generated C file.
    ///
    /// The generate C test will be compiled by a C compiler, and this can be
    /// used to ensure that all the necessary header files are included to test
    /// all FFI definitions.
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
    /// cfg.header("foo.h")
    ///    .header("bar.h");
    /// ```
    pub fn header(&mut self, header: &str) -> &mut TestGenerator {
        self.headers.push(header.to_string());
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
    pub fn include<P: AsRef<Path>>(&mut self, p: P) -> &mut TestGenerator {
        self.includes.push(p.as_ref().to_owned());
        self
    }

    /// Add a flag to the C compiler invocation.
    ///
    /// This can be useful for tweaking the warning settings of the underlying
    /// compiler.
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
    ///
    /// // if msvc
    /// cfg.flag("/wd4820");
    ///
    /// // if gnu
    /// cfg.flag("-Wno-type-limits");
    /// ```
    pub fn flag(&mut self, flag: &str) -> &mut TestGenerator {
        self.flags.push(flag.to_string());
        self
    }

    /// Configures the output directory of the generated Rust and C code.
    ///
    /// Note that for Cargo builds this defaults to `$OUT_DIR` and it's not
    /// necessary to call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.out_dir("path/to/output");
    /// ```
    pub fn out_dir<P: AsRef<Path>>(&mut self, p: P) -> &mut TestGenerator {
        self.out_dir = Some(p.as_ref().to_owned());
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
    pub fn target(&mut self, target: &str) -> &mut TestGenerator {
        self.target = Some(target.to_string());
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
    pub fn define(&mut self, k: &str, v: Option<&str>) -> &mut TestGenerator {
        self.defines.push((k.to_string(), v.map(|s| s.to_string())));
        self
    }

    /// Set a `--cfg` option with which to expand the Rust FFI crate.
    ///
    /// By default the Rust code is run through expansion to determine what C
    /// APIs are exposed (to allow differences across platforms). The `k`
    /// argument is the `#[cfg]` value to define, and `v` is an optional value
    /// for differentiating between `#[cfg(foo)]` and `#[cfg(foo = "bar")]`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.cfg("foo", None)
    ///    .cfg("bar", Some("baz"));
    pub fn cfg(&mut self, k: &str, v: Option<&str>) -> &mut TestGenerator {
        self.cfg.push((k.to_string(), v.map(|s| s.to_string())));
        self
    }

    /// Configures how a Rust type name is translated to a C type name.
    ///
    /// The closure is given a Rust type name as well as a boolean indicating
    /// wehther it's a struct or not.
    ///
    /// The default behavior is that `struct foo` in Rust is translated to
    /// `struct foo` in C, and `type foo` in Rust is translated to `foo` in C.
    /// Some header files, however, have the convention that `struct foo_t` in
    /// Rust should be `foo_t` in C, for example.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.type_name(|ty, is_struct| {
    ///     if is_struct {
    ///         format!("{}_t", ty)
    ///     } else {
    ///         ty.to_string()
    ///     }
    /// });
    pub fn type_name<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str, bool) -> String + 'static
    {
        self.type_name = Box::new(f);
        self
    }

    /// Configures how a Rust struct field is translated to a C struct field.
    ///
    /// The closure is given a Rust struct name as well as a field within that
    /// struct. The name of the corresponding field in C is then returned.
    ///
    /// By default the field name in C just matches the field name in Rust, but
    /// this is useful for fields which otherwise are named after keywords in
    /// Rust (such as a field name of `type`).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.field_name(|_s, field| {
    ///     field.replace("foo", "bar")
    /// });
    pub fn field_name<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str, &str) -> String + 'static
    {
        self.field_name = Box::new(f);
        self
    }

    /// Configures whether all tests for a field are skipped or not.
    ///
    /// The closure is given a Rust struct name as well as a field within that
    /// struct. A flag indicating whether the field should be tested for type,
    /// size, offset, and alignment should be skipped or not.
    ///
    /// By default all field properties are tested.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_field(|s, field| {
    ///     s == "foo_t" || (s == "bar_t" && field == "bar")
    /// });
    pub fn skip_field<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str, &str) -> bool + 'static
    {
        self.skip_field = Box::new(f);
        self
    }

    /// Configures whether tests for the type of a field is skipped or not.
    ///
    /// The closure is given a Rust struct name as well as a field within that
    /// struct. A flag indicating whether the field's type should be tested is
    /// returned.
    ///
    /// By default all field properties are tested.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_field_type(|s, field| {
    ///     s == "foo_t" || (s == "bar_t" && field == "bar")
    /// });
    /// ```
    pub fn skip_field_type<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str, &str) -> bool + 'static
    {
        self.skip_field_type = Box::new(f);
        self
    }

    /// Configures whether a types signededness is tested or not.
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
    pub fn skip_signededness<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str) -> bool + 'static
    {
        self.skip_signededness = Box::new(f);
        self
    }

    /// Configures whether tests for a function definition are generated.
    ///
    /// The closure is given the name of a Rust FFI function and returns whether
    /// test will be generated.
    ///
    /// By default a functions signature is checked along with the function
    /// pointer pointing to the same location as well.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_fn(|s| {
    ///     s.starts_with("foo_")
    /// });
    /// ```
    pub fn skip_fn<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str) -> bool + 'static
    {
        self.skip_fn = Box::new(f);
        self
    }

    /// Configures whether tests for a function pointer's value are generated.
    ///
    /// The closure is given the name of a Rust FFI function and returns whether
    /// the test will be generated.
    ///
    /// By default generated tests will ensure that the function pointer in C
    /// corresponds to the same function pointer in Rust. This can often
    /// unconver subtle symbol naming issues where a header file is referenced
    /// through the C identifier `foo` but the underlying symbol is mapped to
    /// something like `__foo_compat`.
    pub fn skip_fn_ptrcheck<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str) -> bool + 'static
    {
        self.skip_fn_ptrcheck = Box::new(f);
        self
    }

    /// Configures whether the tests for a constant's value are generated.
    ///
    /// The closure is given the name of a Rust constant and returns whether the
    /// test will be generated.
    ///
    /// By default all constant values are verified.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_const(|s| {
    ///     s.starts_with("FOO_")
    /// });
    /// ```
    pub fn skip_const<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str) -> bool + 'static
    {
        self.skip_const = Box::new(f);
        self
    }

    /// Configures whether the tests for a typedef are emitted.
    ///
    /// The closure is passed the name of a Rust typedef and returns whether the
    /// tests are generated.
    ///
    /// By default existence of a typedef is checked.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_type(|s| {
    ///     s.starts_with("foo_")
    /// });
    /// ```
    pub fn skip_type<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str) -> bool + 'static
    {
        self.skip_type = Box::new(f);
        self
    }

    /// Configures whether the tests for a struct are emitted.
    ///
    /// The closure is passed the name of a Rust struct and returns whether the
    /// tests are generated.
    ///
    /// By default structs undergo tests such as size, alignment, existence,
    /// field offset, etc.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.skip_struct(|s| {
    ///     s.starts_with("foo_")
    /// });
    /// ```
    pub fn skip_struct<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str) -> bool + 'static
    {
        self.skip_struct = Box::new(f);
        self
    }

    /// Configures the name of a function in the generate C code.
    ///
    /// The closure is passed the Rust name of a function as well as any
    /// optional `#[link_name]` specified.
    ///
    /// By default the name of the generated C reference is the same as the Rust
    /// function. This is useful, however, if different naming conventions are
    /// used in Rust than are present in C (which is discouraged, however).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.fn_cname(|rust, link_name| link_name.unwrap_or(rust).to_string());
    /// ```
    pub fn fn_cname<F>(&mut self, f: F) -> &mut TestGenerator
        where F: Fn(&str, Option<&str>) -> String + 'static
    {
        self.fn_cname = Box::new(f);
        self
    }

    /// Generate all tests.
    ///
    /// This function is first given the path to the `*-sys` crate which is
    /// being tested along with an output file from where to generate the Rust
    /// side of the tests.
    ///
    /// This function does not consume the builder, but it is expected that all
    /// configuration has happened prior to calling this function.
    ///
    /// This will also generate the corresponding C side of the tests and
    /// compile it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ctest::TestGenerator;
    ///
    /// let mut cfg = TestGenerator::new();
    /// cfg.generate("../path/to/libfoo-sys/lib.rs", "all.rs");
    /// ```
    pub fn generate<P: AsRef<Path>>(&mut self, krate: P, out_file: &str) {
        self._generate(krate.as_ref(), out_file)
    }

    fn _generate(&mut self, krate: &Path, out_file: &str) {
        let out = self.generate_files(krate, out_file);

        let target = self.target.clone().unwrap_or_else(|| {
            env::var("TARGET").unwrap()
        });

        // Compile our C shim to be linked into tests
        let mut cfg = gcc::Config::new();
        cfg.file(&out.with_extension("c"));
        if target.contains("msvc") {
            cfg.flag("/W3").flag("/Wall").flag("/WX")
                // ignored warnings
               .flag("/wd4820")  // warning about adding padding?
               .flag("/wd4100")  // unused parameters
               .flag("/wd4996")  // deprecated functions
               .flag("/wd4296")  // '<' being always false
               .flag("/wd4255")  // converting () to (void)
               .flag("/wd4668")  // using an undefined thing in preprocessor?
                ;
        } else {
            cfg.flag("-Wall").flag("-Wextra").flag("-Werror")
               .flag("-Wno-unused-parameter")
               .flag("-Wno-type-limits");
        }

        for flag in self.flags.iter() {
            cfg.flag(flag);
        }

        for &(ref a, ref b) in self.defines.iter() {
            cfg.define(a, b.as_ref().map(|s| &s[..]));
        }
        for p in self.includes.iter() {
            cfg.include(p);
        }

        let stem = out.file_stem().unwrap().to_str().unwrap();
        cfg.target(&target)
           .out_dir(out.parent().unwrap())
           .compile(&format!("lib{}.a", stem));
    }

    #[doc(hidden)] // TODO: needs docs
    pub fn generate_files<P: AsRef<Path>>(&mut self, krate: P, out_file: &str)
                                          -> PathBuf {
        self._generate_files(krate.as_ref(), out_file)
    }

    fn _generate_files(&mut self, krate: &Path, out_file: &str)
                       -> PathBuf {
        // Prep the test generator
        let out_dir = self.out_dir.clone().unwrap_or_else(|| {
            PathBuf::from(env::var_os("OUT_DIR").unwrap())
        });
        let out_file = out_dir.join(out_file);
        let c_file = out_file.with_extension("c");
        let rust_out = BufWriter::new(t!(File::create(&out_file)));
        let c_out = BufWriter::new(t!(File::create(&c_file)));
        let sess = ParseSess::new();

        // Parse the libc crate
        let krate = parse::parse_crate_from_file(krate, Vec::new(), &sess);

        // expand macros
        let mut ecfg = expand::ExpansionConfig::default("crate_name".to_string());
        ecfg.recursion_limit = 128;
        let exts = vec![
            (intern("macro_rules"), SyntaxExtension::MacroRulesTT),
        ];
        let mut feature_gated_cfgs = Vec::new();
        let ecx = ExtCtxt::new(&sess, Vec::new(), ecfg, &mut feature_gated_cfgs);
        let mut krate = expand::expand_crate(ecx, Vec::new(), exts, krate);

        // Strip the crate down to just what's configured for our target
        let target = self.target.clone().unwrap_or_else(|| {
            env::var("TARGET").unwrap()
        });
        for (k, v) in default_cfg(&target).into_iter().chain(self.cfg.clone()) {
            let s = |s: &str| InternedString::new_from_name(intern(s));
            krate.0.config.push(match v {
                Some(v) => attr::mk_name_value_item_str(s(&k), s(&v)),
                None => attr::mk_word_item(s(&k)),
            });
        }
        let krate = syntax::config::strip_unconfigured_items(&sess.span_diagnostic,
                                                             krate.0,
                                                             &mut Vec::new());

        // Probe the crate to find all structs (used to convert type names to
        // names in C).
        let mut structs = StructFinder {
            structs: HashSet::new(),
        };
        visit::walk_crate(&mut structs, &krate);

        let mut gen = Generator {
            target: &target,
            rust: Box::new(rust_out),
            c: Box::new(c_out),
            sh: &sess.span_diagnostic,
            structs: structs.structs,
            abi: Abi::C,
            tests: Vec::new(),
            files: HashSet::new(),
            sess: &sess,
            opts: self,
        };
        t!(writeln!(gen.c, "#include <stdint.h>"));
        t!(writeln!(gen.c, "#include <stddef.h>"));
        for header in self.headers.iter() {
            t!(writeln!(gen.c, "#include <{}>", header));
        }

        t!(gen.rust.write_all(br#"
            use std::any::{Any, TypeId};
            use std::mem;
            use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
            use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

            fn main() {
                println!("RUNNING ALL TESTS");
                run_all();
                if FAILED.load(Ordering::SeqCst) {
                    panic!("some tests failed");
                } else {
                    println!("PASSED {} tests", NTESTS.load(Ordering::SeqCst));
                }
            }

            trait Pretty {
                fn pretty(&self) -> String;
            }

            impl<'a> Pretty for &'a str {
                fn pretty(&self) -> String { format!("{:?}", self) }
            }
            impl<T> Pretty for *const T {
                fn pretty(&self) -> String { format!("{:?}", self) }
            }
            impl<T> Pretty for *mut T {
                fn pretty(&self) -> String { format!("{:?}", self) }
            }
            macro_rules! p {
                ($($i:ident)*) => ($(
                    impl Pretty for $i {
                        fn pretty(&self) -> String {
                            format!("{} ({:#x})", self, self)
                        }
                    }
                )*)
            }
            p! { i8 i16 i32 i64 u8 u16 u32 u64 usize isize }

            static FAILED: AtomicBool = ATOMIC_BOOL_INIT;
            static NTESTS: AtomicUsize = ATOMIC_USIZE_INIT;

            fn same<T: Eq + Pretty>(rust: T, c: T, attr: &str) {
                if rust != c {
                    println!("bad {}: rust: {} != c {}", attr, rust.pretty(),
                             c.pretty());
                    FAILED.store(true, Ordering::SeqCst);
                } else {
                    NTESTS.fetch_add(1, Ordering::SeqCst);
                }
            }

            #[allow(deprecated)] // min_align_of is correct, but deprecated
            fn align<T: Any>() -> u64 {
                // TODO: apparently these three types have less alignment in
                //       Rust on x86 than they do in C this difference
                //       should.. probably be reconciled.
                //
                //       Perhaps #27195?
                if cfg!(target_pointer_width = "32") {
                    if TypeId::of::<T>() == TypeId::of::<f64>() ||
                       TypeId::of::<T>() == TypeId::of::<i64>() ||
                       TypeId::of::<T>() == TypeId::of::<u64>() {
                        return 8
                    }
                }
                mem::min_align_of::<T>() as u64
            }

            macro_rules! offset_of {
                ($ty:ident, $field:ident) => (
                    (&((*(0 as *const $ty)).$field)) as *const _ as u64
                )
            }

        "#));

        // Walk the crate, emitting test cases for everything found
        visit::walk_crate(&mut gen, &krate);
        gen.emit_run_all();

        return out_file
    }
}

fn default_cfg(target: &str) -> Vec<(String, Option<String>)> {
    let mut ret = Vec::new();
    let (arch, width) = if target.starts_with("x86_64") {
        ("x86_64", "64")
    } else if target.starts_with("i386") ||
        target.starts_with("i586") ||
        target.starts_with("i686") {
        ("x86", "32")
    } else if target.starts_with("arm") {
        ("arm", "32")
    } else if target.starts_with("aarch64") {
        ("aarch64", "64")
    } else if target.starts_with("mips64") {
        ("mips64", "64")
    } else if target.starts_with("mips") {
        ("mips", "32")
    } else if target.starts_with("powerpc64") {
        ("powerpc64", "64")
    } else if target.starts_with("powerpc") {
        ("powerpc", "32")
    } else if target.starts_with("s390x") {
        ("s390x", "64")
    } else if target.starts_with("sparc64") {
        ("sparc64", "64")
    } else if target.starts_with("asmjs") {
        ("asmjs", "32")
    } else if target.starts_with("wasm32") {
        ("wasm32", "32")
    } else {
        panic!("unknown arch/pointer width: {}", target)
    };
    let (os, family, env) = if target.contains("unknown-linux-gnu") {
        ("linux", "unix", "gnu")
    } else if target.contains("unknown-linux-musl") {
        ("linux", "unix", "musl")
    } else if target.contains("unknown-linux-uclibc") {
        ("linux", "unix", "uclibc")
    } else if target.contains("apple-darwin") {
        ("macos", "unix", "")
    } else if target.contains("apple-ios") {
        ("ios", "unix", "")
    } else if target.contains("windows-msvc") {
        ("windows", "windows", "msvc")
    } else if target.contains("windows-gnu") {
        ("windows", "windows", "gnu")
    } else if target.contains("android") {
        ("android", "unix", "")
    } else if target.contains("unknown-freebsd") {
        ("freebsd", "unix", "")
    } else if target.contains("netbsd") {
        ("netbsd", "unix", "")
    } else if target.contains("openbsd") {
        ("openbsd", "unix", "")
    } else if target.contains("dragonfly") {
        ("dragonfly", "unix", "")
    } else if target.contains("emscripten") {
        ("emscripten", "unix", "")
    } else {
        panic!("unknown os/family width: {}", target)
    };

    // TODO: endianness
    ret.push((family.to_string(), None));
    ret.push(("target_os".to_string(), Some(os.to_string())));
    ret.push(("target_family".to_string(), Some(family.to_string())));
    ret.push(("target_arch".to_string(), Some(arch.to_string())));
    ret.push(("target_pointer_width".to_string(), Some(width.to_string())));
    ret.push(("target_env".to_string(), Some(env.to_string())));

    return ret
}

impl<'a> Generator<'a> {
    fn rust2c(&self, ty: &str) -> String {
        match ty {
            t if t.starts_with("c_") => {
                match &ty[2..].replace("long", " long")[..] {
                    s if s.starts_with("u") => format!("unsigned {}", &s[1..]),
                    "short" => format!("short"),
                    s if s.starts_with("s") => format!("signed {}", &s[1..]),
                    s => s.to_string(),
                }
            }

            "usize" => "size_t".to_string(),
            "isize" => "ssize_t".to_string(),
            "u8" => "uint8_t".to_string(),
            "u16" => "uint16_t".to_string(),
            "u32" => "uint32_t".to_string(),
            "u64" => "uint64_t".to_string(),
            "i8" => "int8_t".to_string(),
            "i16" => "int16_t".to_string(),
            "i32" => "int32_t".to_string(),
            "i64" => "int64_t".to_string(),

            s => (self.opts.type_name)(s, self.structs.contains(s)),
        }
    }

    fn rust2cfield(&self, struct_: &str, field: &str) -> String {
        (self.opts.field_name)(struct_, field)
    }

    fn test_type(&mut self, ty: &str) {
        if (self.opts.skip_type)(ty) {
            return
        }
        let c = self.rust_ty_to_c_ty(ty);
        self.test_size_align(ty, &c);
        self.test_sign(ty, &c);
    }

    fn test_struct(&mut self, ty: &str, s: &ast::VariantData) {
        if (self.opts.skip_struct)(ty) {
            return
        }

        let cty = self.rust_ty_to_c_ty(ty);
        self.test_size_align(ty, &cty);

        self.tests.push(format!("field_offset_size_{}", ty));
        t!(writeln!(self.rust, r#"
            fn field_offset_size_{ty}() {{
        "#, ty = ty));
        for field in s.fields() {
            let name = match field.node.kind {
                ast::NamedField(name, ast::Visibility::Public) => name,
                ast::NamedField(_, ast::Visibility::Inherited) => continue,
                ast::UnnamedField(..) => panic!("no tuple structs in FFI"),
            };
            let name = name.to_string();

            if (self.opts.skip_field)(ty, &name) {
                continue
            }

            let cfield = self.rust2cfield(ty, &name);

            t!(writeln!(self.c, r#"
                uint64_t __test_offset_{ty}_{rust_field}(void) {{
                    return offsetof({cstructty}, {c_field});
                }}
                uint64_t __test_size_{ty}_{rust_field}(void) {{
                    {cstructty}* foo = NULL;
                    return sizeof(foo->{c_field});
                }}
            "#, ty = ty, cstructty = cty, rust_field = name, c_field = cfield));

            t!(writeln!(self.rust, r#"
                extern {{
                    fn __test_offset_{ty}_{field}() -> u64;
                    fn __test_size_{ty}_{field}() -> u64;
                }}
                unsafe {{
                    let foo = 0 as *mut {ty};
                    same(offset_of!({ty}, {field}),
                         __test_offset_{ty}_{field}(),
                         "field offset {field} of {ty}");
                    same(mem::size_of_val(&(*foo).{field}) as u64,
                         __test_size_{ty}_{field}(),
                         "field size {field} of {ty}");
                }}
            "#, ty = ty, field = name));

            if (self.opts.skip_field_type)(ty, &name.to_string()) {
                continue
            }

            let sig = format!("__test_field_type_{}_{}({}* b)", ty, name, cty);
            let sig = self.csig_returning_ptr(&field.node.ty, &sig);
            t!(writeln!(self.c, r#"
                {sig} {{
                    return &b->{c_field};
                }}
            "#, sig = sig, c_field = cfield));
            t!(writeln!(self.rust, r#"
                extern {{
                    fn __test_field_type_{ty}_{field}(a: *mut {ty})
                                                      -> *mut u8;
                }}
                unsafe {{
                    let foo = 0 as *mut {ty};
                    same(&(*foo).{field} as *const _ as *mut _,
                         __test_field_type_{ty}_{field}(foo),
                         "field type {field} of {ty}");
                }}
            "#, ty = ty, field = name));
        }
        t!(writeln!(self.rust, r#"
            }}
        "#));
    }

    fn test_size_align(&mut self, rust: &str, c: &str) {
        let align_of = if self.target.contains("msvc") {
            "__alignof"
        } else {
            "__alignof__"
        };
        t!(writeln!(self.c, r#"
            uint64_t __test_size_{ty}(void) {{ return sizeof({cty}); }}
            uint64_t __test_align_{ty}(void) {{ return {align_of}({cty}); }}
        "#, ty = rust, cty = c, align_of = align_of));
        t!(writeln!(self.rust, r#"
            fn size_align_{ty}() {{
                extern {{
                    fn __test_size_{ty}() -> u64;
                    fn __test_align_{ty}() -> u64;
                }}
                unsafe {{
                    same(mem::size_of::<{ty}>() as u64,
                         __test_size_{ty}(), "{ty} size");
                    same(align::<{ty}>() as u64,
                         __test_align_{ty}(), "{ty} align");
                }}
            }}
        "#, ty = rust));
        self.tests.push(format!("size_align_{}", rust));
    }

    fn test_sign(&mut self, rust: &str, c: &str) {
        match c {
            "float" | "double" => return, // nope, never has a sign
            _ => {}
        }
        if (self.opts.skip_signededness)(rust) {
            return
        }
        t!(writeln!(self.c, r#"
            uint32_t __test_signed_{ty}(void) {{
                return ((({cty}) -1) < 0);
            }}
        "#, ty = rust, cty = c));
        t!(writeln!(self.rust, r#"
            fn sign_{ty}() {{
                extern {{
                    fn __test_signed_{ty}() -> u32;
                }}
                unsafe {{
                    same(((!(0 as {ty})) < (0 as {ty})) as u32,
                         __test_signed_{ty}(), "{ty} signed");
                }}
            }}
        "#, ty = rust));
        self.tests.push(format!("sign_{}", rust));
    }

    fn rust_ty_to_c_ty(&self, mut rust_ty: &str) -> String {
        if rust_ty == "&str" {
            return "char*".to_string();
        }
        let mut cty = self.rust2c(&rust_ty.replace("*mut ", "")
                                          .replace("*const ", ""));
        while rust_ty.starts_with("*") {
            if rust_ty.starts_with("*const") {
                cty = format!("const {}*", cty);
                rust_ty = &rust_ty[7..];
            } else {
                cty = format!("{}*", cty);
                rust_ty = &rust_ty[5..];
            }
        }
        return cty
    }

    fn test_const(&mut self, name: &str, rust_ty: &str) {
        if (self.opts.skip_const)(name) {
            return
        }

        let cty = self.rust_ty_to_c_ty(rust_ty);
        t!(writeln!(self.c, r#"
            static {cty} __test_const_{name}_val = {name};
            {cty}* __test_const_{name}(void) {{
                return &__test_const_{name}_val;
            }}
        "#, name = name, cty = cty));

        if rust_ty == "&str" {
            t!(writeln!(self.rust, r#"
                fn const_{name}() {{
                    extern {{
                        fn __test_const_{name}() -> *const *const u8;
                    }}
                    let val = {name};
                    unsafe {{
                        let ptr = *__test_const_{name}();
                        let c = ::std::ffi::CStr::from_ptr(ptr as *const _);
                        let c = c.to_str().expect("const {name} not utf8");
                        same(val, c, "{name} string");
                    }}
                }}
            "#, name = name));
        } else {
            t!(writeln!(self.rust, r#"
                fn const_{name}() {{
                    extern {{
                        fn __test_const_{name}() -> *const {ty};
                    }}
                    let val = {name};
                    unsafe {{
                        let ptr1 = &val as *const _ as *const u8;
                        let ptr2 = __test_const_{name}() as *const u8;
                        for i in 0..mem::size_of::<{ty}>() {{
                            let i = i as isize;
                            same(*ptr1.offset(i), *ptr2.offset(i),
                                 &format!("{name} value at byte {{}}", i));
                        }}
                    }}
                }}
            "#, ty = rust_ty, name = name));
        }
        self.tests.push(format!("const_{}", name));
    }

    fn test_extern_fn(&mut self, name: &str, cname: Option<String>,
                      args: &[String], ret: &str,
                      variadic: bool, abi: Abi) {
        if (self.opts.skip_fn)(name) {
            return
        }
        let cname = (self.opts.fn_cname)(name, cname.as_ref().map(|s| &**s));
        let args = if args.len() == 0 && !variadic {
            "void".to_string()
        } else {
            args.iter().map(|a| self.rust_ty_to_c_ty(a)).collect::<Vec<_>>()
                .connect(", ") + if variadic {", ..."} else {""}
        };
        let cret = self.rust_ty_to_c_ty(ret);
        let abi = self.abi2str(abi);
        t!(writeln!(self.c, r#"
            {ret} ({abi}*__test_fn_{name}(void))({args}) {{
                return {cname};
            }}
        "#, name = name, cname = cname, args = args, ret = cret, abi = abi));
        t!(writeln!(self.rust, r#"
            fn fn_{name}() {{
                extern {{
                    fn __test_fn_{name}() -> *mut u32;
                }}
                unsafe {{
                    if !{skip} {{
                        same({name} as usize,
                             __test_fn_{name}() as usize,
                             "{name} function pointer");
                    }}
                }}
            }}
        "#, name = name, skip = (self.opts.skip_fn_ptrcheck)(name)));
        self.tests.push(format!("fn_{}", name));
    }

    fn assert_no_generics(&self, _i: ast::Ident, generics: &ast::Generics) {
        assert!(generics.lifetimes.len() == 0);
        assert!(generics.ty_params.len() == 0);
        assert!(generics.where_clause.predicates.len() == 0);
    }

    fn ty2name(&self, ty: &ast::Ty, rust: bool) -> String {
        match ty.node {
            ast::TyKind::Path(_, ref path) => {
                let last = path.segments.last().unwrap();
                if last.identifier.to_string() == "Option" {
                    match last.parameters {
                        ast::PathParameters::AngleBracketed(ref p) => {
                            self.ty2name(&p.types[0], rust)
                        }
                        _ => panic!(),
                    }
                } else if rust {
                    last.identifier.to_string()
                } else {
                    self.rust2c(&last.identifier.to_string())
                }
            }
            ast::TyKind::Ptr(ref t) => {
                if rust {
                    format!("*{} {}", match t.mutbl {
                        ast::Mutability::Immutable => "const",
                        ast::Mutability::Mutable => "mut",
                    }, self.ty2name(&t.ty, rust))
                } else {
                    let modifier = match t.mutbl {
                        ast::Mutability::Immutable => "const ",
                        ast::Mutability::Mutable => "",
                    };
                    match t.ty.node {
                        ast::TyKind::BareFn(..) => self.ty2name(&t.ty, rust),
                        ast::TyKind::Ptr(..) => {
                            format!("{} {}*", self.ty2name(&t.ty, rust),
                                    modifier)
                        }
                        ast::TyKind::FixedLengthVec(ref t, _) => {
                            format!("{}{}*", modifier, self.ty2name(t, rust))
                        }
                        _ => {
                            format!("{}{}*", modifier, self.ty2name(&t.ty, rust))
                        }
                    }
                }
            }
            ast::TyKind::BareFn(ref t) => {
                if rust {
                    let args = t.decl.inputs.iter().map(|a| {
                        self.ty2name(&a.ty, rust)
                    }).collect::<Vec<_>>().connect(", ");
                    let ret = match t.decl.output {
                        ast::FunctionRetTy::None(..) => "!".to_string(),
                        ast::FunctionRetTy::Default(..) => "()".to_string(),
                        ast::FunctionRetTy::Ty(ref t) => self.ty2name(t, rust),
                    };
                    format!("extern fn({}) -> {}", args, ret)
                } else {
                    assert!(t.lifetimes.len() == 0);
                    let (ret, mut args, variadic) = self.decl2rust(&t.decl);
                    assert!(!variadic);
                    if args.len() == 0 {
                        args.push("void".to_string());
                    }
                    format!("{}(*)({})", ret, args.connect(", "))
                }
            }
            ast::TyKind::FixedLengthVec(ref t, ref e) => {
                assert!(rust);
                format!("[{}; {}]", self.ty2name(t, rust), self.expr2str(e))
            }
            ast::TyKind::Rptr(_, ast::MutTy {
                ref ty,
                mutbl,
            }) => {
                let path = match ty.node {
                    ast::TyKind::Path(_, ref p) => p,
                    ast::TyKind::FixedLengthVec(ref t, _) => {
                        assert!(!rust);
                        return format!("{}{}*",
                                       match mutbl {
                                           ast::Mutability::Immutable => "const ",
                                           ast::Mutability::Mutable => "",
                                       },
                                       self.ty2name(t, rust))
                    }
                    _ => panic!("unknown ty {:?}", ty),
                };
                if path.segments.len() != 1 {
                    panic!("unknown ty {:?}", ty)
                }
                if &*path.segments[0].identifier.name.as_str() != "str" {
                    panic!("unknown ty {:?}", ty)
                }
                if mutbl != ast::Mutability::Immutable {
                    panic!("unknown ty {:?}", ty)
                }
                if rust {
                    format!("&str")
                } else {
                    format!("char*")
                }
            }
            _ => panic!("unknown ty {:?}", ty),
        }
    }

    fn csig_returning_ptr(&self, ty: &ast::Ty, sig: &str) -> String {
        match ty.node {
            ast::TyKind::Path(_, ref path) if path.segments.last().unwrap()
                                            .identifier.to_string() == "Option"
            => {
                let last = path.segments.last().unwrap();
                match last.parameters {
                    ast::PathParameters::AngleBracketed(ref p) => {
                        self.csig_returning_ptr(&p.types[0], sig)
                    }
                    _ => panic!(),
                }
            }
            ast::TyKind::BareFn(ref t) => {
                assert!(t.lifetimes.len() == 0);
                let (ret, mut args, variadic) = self.decl2rust(&t.decl);
                let abi = self.abi2str(t.abi);
                if variadic {
                    args.push("...".to_string());
                } else if args.len() == 0 {
                    args.push("void".to_string());
                }
                format!("{}({}**{})({})", ret, abi, sig, args.connect(", "))
            }
            ast::TyKind::FixedLengthVec(ref t, ref e) => {
                match t.node {
                    ast::TyKind::FixedLengthVec(ref t2, ref e2) => {
                        format!("{}(*{})[{}][{}]",
                                self.ty2name(t2, false),
                                sig,
                                self.expr2str(e),
                                self.expr2str(e2))
                    }
                    _ => {
                        format!("{}(*{})[{}]", self.ty2name(t, false), sig,
                                self.expr2str(e))
                    }
                }
            }
            _ => format!("{}* {}", self.ty2name(ty, false), sig)
        }
    }

    fn expr2str(&self, e: &ast::Expr) -> String {
        match e.node {
            ast::ExprKind::Lit(ref l) => {
                match l.node {
                    ast::LitKind::Int(a, _) => a.to_string(),
                    _ => panic!("unknown literal: {:?}", l),
                }
            }
            ast::ExprKind::Path(_, ref path) => {
                path.segments.last().unwrap().identifier.to_string()
            }
            ast::ExprKind::Cast(ref e, _) => self.expr2str(e),
            ast::ExprKind::Binary(ref op, ref e1, ref e2) => {
                let e1 = self.expr2str(e1);
                let e2 = self.expr2str(e2);
                match op.node {
                    ast::BinOpKind::Add => format!("{} + {}", e1, e2),
                    _ => panic!("unknown op: {:?}", op),
                }
            }
            _ => panic!("unknown expr: {:?}", e),
        }
    }

    fn abi2str(&self, abi: Abi) -> &'static str {
        match abi {
            Abi::C => "",
            Abi::Stdcall => "__stdcall ",
            Abi::System if self.target.contains("i686-pc-windows") => {
                "__stdcall "
            }
            Abi::System => "",
            a => panic!("unknown ABI: {}", a),
        }
    }

    fn decl2rust(&self, decl: &ast::FnDecl) -> (String, Vec<String>, bool) {
        let args = decl.inputs.iter().map(|arg| {
            self.ty2name(&arg.ty, false)
        }).collect::<Vec<_>>();
        let ret = match decl.output {
            ast::FunctionRetTy::None(..) |
            ast::FunctionRetTy::Default(..) => "void".to_string(),
            ast::FunctionRetTy::Ty(ref t) => {
                match t.node {
                    ast::TyKind::Tup(ref t) if t.len() == 0 => {
                        "void".to_string()
                    }
                    _ => self.ty2name(t, false),
                }
            }
        };
        (ret, args, decl.variadic)
    }

    fn emit_run_all(&mut self) {
        t!(writeln!(self.rust, "
            fn run_all() {{
        "));
        for test in self.tests.iter() {
            t!(writeln!(self.rust, "{}();", test));
        }
        t!(writeln!(self.rust, "
            }}
        "));
    }
}

impl<'a, 'v> Visitor<'v> for Generator<'a> {
    fn visit_item(&mut self, i: &'v ast::Item) {
        let prev_abi = self.abi;
        let public = i.vis == ast::Visibility::Public;
        match i.node {
            ast::ItemKind::Ty(_, ref generics) if public => {
                self.assert_no_generics(i.ident, generics);
                self.test_type(&i.ident.to_string());
            }

            ast::ItemKind::Struct(ref s, ref generics) if public => {
                self.assert_no_generics(i.ident, generics);
                let is_c = i.attrs.iter().any(|a| {
                    attr::find_repr_attrs(self.sh, a).iter().any(|a| {
                        *a == ReprAttr::ReprExtern
                    })
                });
                if !is_c {
                    panic!("{} is not marked #[repr(C)]", i.ident);
                }
                self.test_struct(&i.ident.to_string(), s);
            }

            ast::ItemKind::Const(ref ty, _) if public => {
                let ty = self.ty2name(ty, true);
                self.test_const(&i.ident.to_string(), &ty);
            }

            ast::ItemKind::ForeignMod(ref fm) => {
                self.abi = fm.abi;
            }

            _ => {}
        }
        let file = self.sess.codemap().span_to_filename(i.span);
        if self.files.insert(file.clone()) {
            println!("cargo:rerun-if-changed={}", file);
        }
        visit::walk_item(self, i);
        self.abi = prev_abi;
    }

    fn visit_foreign_item(&mut self, i: &'v ast::ForeignItem) {
        match i.node {
            ast::ForeignItemKind::Fn(ref decl, ref generics) => {
                self.assert_no_generics(i.ident, generics);
                let (ret, args, variadic) = self.decl2rust(decl);
                let cname = attr::first_attr_value_str_by_name(&i.attrs, "link_name")
                                 .map(|i| i.to_string());
                let abi = self.abi;
                self.test_extern_fn(&i.ident.to_string(), cname, &args, &ret,
                                    variadic, abi);
            }
            ast::ForeignItemKind::Static(_, _) => {
            }
        }
        visit::walk_foreign_item(self, i)
    }

    fn visit_mac(&mut self, _mac: &'v ast::Mac) { }
}

impl<'v> Visitor<'v> for StructFinder {
    fn visit_item(&mut self, i: &'v ast::Item) {
        match i.node {
            ast::ItemKind::Struct(..) => {
                self.structs.insert(i.ident.to_string());
            }
            ast::ItemKind::Enum(..) => {
                self.structs.insert(i.ident.to_string());
            }

            _ => {}
        }
        visit::walk_item(self, i)
    }
    fn visit_mac(&mut self, _mac: &'v ast::Mac) { }
}
