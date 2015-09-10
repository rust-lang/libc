extern crate gcc;
extern crate syntex_syntax as syntax;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::iter;
use std::path::{Path, PathBuf};

use syntax::ast;
use syntax::diagnostic::SpanHandler;
use syntax::parse::token::InternedString;
use syntax::attr::{self, ReprAttr};
use syntax::parse::{self, ParseSess};
use syntax::visit::{self, Visitor};

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

struct TestGenerator<'a> {
    target: String,
    rust: Box<Write>,
    c: Box<Write>,
    sh: &'a SpanHandler,
    structs: HashSet<String>,
}

struct StructFinder {
    structs: HashSet<String>,
}

impl<'a> TestGenerator<'a> {
    fn defines(&self) -> Vec<&'static str> {
        let mut ret = Vec::new();
        if self.target.contains("unknown-linux") {
            ret.push("_GNU_SOURCE");
        }
        return ret
    }

    fn headers(&self) -> Vec<&'static str> {
        let mut base = vec![
            "errno.h",
            "fcntl.h",
            "glob.h",
            "ifaddrs.h",
            "limits.h",
            "net/if.h",
            "netdb.h",
            "netinet/in.h",
            "netinet/ip.h",
            "netinet/tcp.h",
            "pthread.h",
            "signal.h",
            "stdalign.h",
            "stddef.h",
            "stdint.h",
            "stdio.h",
            "stdlib.h",
            "sys/mman.h",
            "sys/resource.h",
            "sys/socket.h",
            "sys/stat.h",
            "sys/time.h",
            "sys/types.h",
            "sys/un.h",
            "time.h",
            "unistd.h",
            "utime.h",
            "wchar.h",
        ];

        if self.target.contains("apple-darwin") {
            base.push("mach/mach_time.h");
        }
        if self.target.contains("unknown-linux") {
            base.push("linux/if_packet.h");
            base.push("net/ethernet.h");
        }

        return base
    }

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
            "ip6_mreq" => "struct ipv6_mreq".to_string(),
            "glob_t" => "glob_t".to_string(),
            t if t.starts_with("pthread") => t.to_string(),

            t if self.structs.contains(t) => format!("struct {}", t),

            t => t.to_string(),
        }
    }

    fn rust2cfield(&self, struct_: &str, field: &str) -> String {
        match field {
            s if s.ends_with("_nsec") && struct_ == "stat" => {
                if self.target.contains("apple-darwin") {
                    s.replace("_nsec", "spec.tv_nsec")
                } else {
                    s.replace("e_nsec", ".tv_nsec")
                }
            }
            s => s.to_string(),
        }
    }

    fn cfg_list(&self) -> Vec<(&'static str, Option<&'static str>)> {
        let mut ret = Vec::new();
        let (arch, target_pointer_width) = if self.target.starts_with("x86_64") {
            ("x86_64", "64")
        } else if self.target.starts_with("i686") {
            ("x86", "32")
        } else {
            panic!("unknown arch/pointer width: {}", self.target)
        };
        let (os, family, env) = if self.target.contains("unknown-linux") {
            ("linux", "unix", "gnu")
        } else if self.target.contains("apple-darwin") {
            ("macos", "unix", "")
        } else {
            panic!("unknown os/family width: {}", self.target)
        };

        ret.push((family, None));
        ret.push(("target_os", Some(os)));
        ret.push(("target_family", Some(family)));
        ret.push(("target_arch", Some(arch)));
        // skip endianness
        ret.push(("target_pointer_width", Some(target_pointer_width)));
        ret.push(("target_env", Some(env)));

        return ret
    }
}

fn main() {
    // Prep the test generator
    let target = t!(env::var("TARGET"));
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let rust_out = BufWriter::new(t!(File::create(out.join("all.rs"))));
    let c_out = BufWriter::new(t!(File::create(out.join("all.c"))));
    let sess = ParseSess::new();
    let mut tg = TestGenerator {
        target: target,
        rust: Box::new(rust_out),
        c: Box::new(c_out),
        sh: &sess.span_diagnostic,
        structs: HashSet::new(),
    };

    // Parse the libc crate
    let src = Path::new("../src/lib.rs");
    let cfg = Vec::new();
    let mut krate = parse::parse_crate_from_file(src, cfg, &sess);

    // Strip the crate down to just what's configured for our target
    for (k, v) in tg.cfg_list() {
        let s = InternedString::new;
        krate.config.push(match v {
            Some(v) => attr::mk_name_value_item_str(s(k), s(v)),
            None => attr::mk_word_item(s(k)),
        });
    }
    let mut gated_cfgs = Vec::new();
    let krate = syntax::config::strip_unconfigured_items(&sess.span_diagnostic,
                                                         krate,
                                                         &mut gated_cfgs);

    // Probe the crate to find all structs (used to convert type names to names
    // in C).
    let mut structs = StructFinder {
        structs: HashSet::new(),
    };
    visit::walk_crate(&mut structs, &krate);
    tg.structs = structs.structs;

    // Prep the C file by emitting header stuff
    for define in tg.defines() {
        t!(writeln!(tg.c, "#define {}", define));
    }
    for header in tg.headers() {
        t!(writeln!(tg.c, "#include <{}>", header));
    }

    // Walk the crate, emitting test cases for everything found
    visit::walk_crate(&mut tg, &krate);
    drop(tg);

    // Compile our C shim to be linked into tests
    gcc::Config::new()
                .file(out.join("all.c"))
                .flag("-Wall")
                .flag("-Wextra")
                .flag("-Werror")
                .compile("liball.a");
}

impl<'a> TestGenerator<'a> {
    fn test_type(&mut self, ty: &str) {
        match ty {
            "sighandler_t" => return,
            _ => {}
        }
        let c = self.rust2c(ty);
        self.test_size_align(ty, &c);
    }

    fn test_struct(&mut self, ty: &str, s: &ast::StructDef) {
        let cty = self.rust2c(ty);
        self.test_size_align(ty, &cty);

        t!(writeln!(self.rust, r#"
            #[test]
            fn field_offset_size_{ty}() {{
        "#, ty = ty));
        for field in s.fields.iter() {
            let name = match field.node.kind {
                ast::NamedField(name, ast::Public) => name,
                ast::NamedField(_, ast::Inherited) => continue,
                ast::UnnamedField(..) => panic!("no tuple structs in FFI"),
            };

            let cfield = self.rust2cfield(ty, &name.to_string());

            t!(writeln!(self.c, r#"
                uint64_t __test_offset_{ty}_{rust_field}() {{
                    return offsetof({cty}, {c_field});
                }}
                uint64_t __test_size_{ty}_{rust_field}() {{
                    {cty}* foo = NULL;
                    return sizeof(foo->{c_field});
                }}
            "#, ty = ty, cty = cty, rust_field = name, c_field = cfield));
            t!(writeln!(self.rust, r#"
                extern {{
                    fn __test_offset_{ty}_{field}() -> u64;
                    fn __test_size_{ty}_{field}() -> u64;
                }}
                unsafe {{
                    let foo = 0 as *const {ty};
                    same(offset_of!({ty}, {field}),
                         __test_offset_{ty}_{field}(),
                         "field offset {field} of {ty}");
                    same(mem::size_of_val(&(*foo).{field}) as u64,
                         __test_size_{ty}_{field}(),
                         "field size {field} of {ty}");
                }}
            "#, ty = ty, field = name));
        }
        t!(writeln!(self.rust, r#"
            }}
        "#));
    }

    fn test_size_align(&mut self, rust: &str, c: &str) {
        t!(writeln!(self.c, r#"
            uint64_t __test_size_{ty}() {{ return sizeof({cty}); }}
            uint64_t __test_align_{ty}() {{ return alignof({cty}); }}
        "#, ty = rust, cty = c));
        t!(writeln!(self.rust, r#"
            #[test]
            fn size_align_{ty}() {{
                extern {{
                    fn __test_size_{ty}() -> u64;
                    fn __test_align_{ty}() -> u64;
                }}
                unsafe {{
                    same(mem::size_of::<{ty}>() as u64,
                         __test_size_{ty}(), "size");
                    same(align::<{ty}>() as u64,
                         __test_align_{ty}(), "align");
                }}
            }}
        "#, ty = rust));
    }

    fn test_const(&mut self, name: &str, rust_ty: &str) {
        let cty = self.rust2c(&rust_ty.replace("*mut ", "")
                                      .replace("*const ", ""));
        let ptrs = rust_ty.matches("*").count();
        let cty = format!("{}{}", cty,
                          iter::repeat("*").take(ptrs).collect::<String>());
        let cast = if name == "SIG_IGN" {"(size_t)"} else {""};
        t!(writeln!(self.c, r#"
            {cty} __test_const_{name}() {{ return {cast}({name}); }}
        "#, name = name, cast = cast, cty = cty));
        t!(writeln!(self.rust, r#"
            #[test]
            fn const_{name}() {{
                extern {{
                    fn __test_const_{name}() -> {ty};
                }}
                unsafe {{
                    same({name}, __test_const_{name}(), "value");
                }}
            }}
        "#, ty = rust_ty, name = name));
    }

    fn assert_no_generics(&self, _i: &ast::Item, generics: &ast::Generics) {
         assert!(generics.lifetimes.len() == 0);
         assert!(generics.ty_params.len() == 0);
         assert!(generics.where_clause.predicates.len() == 0);
    }

    fn ty2name(&self, ty: &ast::Ty) -> String {
        match ty.node {
            ast::TyPath(_, ref path) => {
                path.segments.last().unwrap().identifier.to_string()
            }
            ast::TyPtr(ref t) => {
                format!("*{} {}", match t.mutbl {
                    ast::MutImmutable => "const",
                    ast::MutMutable => "mut",
                }, self.ty2name(&t.ty))
            }
            _ => panic!("unknown ty {:?}", ty),
        }
    }
}

impl<'a, 'v> Visitor<'v> for TestGenerator<'a> {
     fn visit_item(&mut self, i: &'v ast::Item) {
         match i.node {
             ast::ItemTy(_, ref generics) => {
                 self.assert_no_generics(i, generics);
                 self.test_type(&i.ident.to_string());
             }

             ast::ItemStruct(ref s, ref generics) => {
                 self.assert_no_generics(i, generics);
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

             ast::ItemConst(ref ty, _) => {
                 let ty = self.ty2name(ty);
                 self.test_const(&i.ident.to_string(), &ty);
             }

             _ => {}
         }
         visit::walk_item(self, i)
     }
}

impl<'v> Visitor<'v> for StructFinder {
     fn visit_item(&mut self, i: &'v ast::Item) {
         match i.node {
             ast::ItemStruct(..) => {
                 self.structs.insert(i.ident.to_string());
             }

             _ => {}
         }
         visit::walk_item(self, i)
     }
}
