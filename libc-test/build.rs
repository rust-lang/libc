#![allow(unused_must_use)]

extern crate gcc;
extern crate syntex_syntax as syntax;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use syntax::ast;
use syntax::diagnostic::SpanHandler;
use syntax::parse::token::InternedString;
use syntax::attr::{self, ReprAttr};
use syntax::parse::{self, ParseSess};
use syntax::visit::{self, Visitor};

struct TestGenerator<'a> {
    rust: Box<Write>,
    c: Box<Write>,
    sh: &'a SpanHandler,
}

fn main() {
    let target = env::var("TARGET").unwrap();

    let sess = ParseSess::new();
    let src = Path::new("../src/lib.rs");
    let cfg = Vec::new();
    let mut krate = parse::parse_crate_from_file(src, cfg, &sess);
    build_cfg(&mut krate.config, &target);

    let mut gated_cfgs = Vec::new();
    let krate = syntax::config::strip_unconfigured_items(&sess.span_diagnostic,
                                                         krate,
                                                         &mut gated_cfgs);

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let rust_out = BufWriter::new(File::create(out.join("all.rs")).unwrap());
    let mut c_out = BufWriter::new(File::create(out.join("all.c")).unwrap());

    writeln!(c_out, "
#include <glob.h>
#include <ifaddrs.h>
#include <netdb.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <pthread.h>
#include <signal.h>
#include <stdalign.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/resource.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/un.h>
#include <time.h>
#include <utime.h>
#include <wchar.h>
");

    if target.contains("apple-darwin") {
        writeln!(c_out, "
#include <mach/mach_time.h>
");
    }

    visit::walk_crate(&mut TestGenerator {
        rust: Box::new(rust_out),
        c: Box::new(c_out),
        sh: &sess.span_diagnostic,
    }, &krate);

    gcc::Config::new()
                .file(out.join("all.c"))
                .compile("liball.a");
}

fn build_cfg(cfg: &mut ast::CrateConfig, target: &str) {
    let (arch, target_pointer_width) = if target.starts_with("x86_64") {
        ("x86_64", "64")
    } else if target.starts_with("i686") {
        ("x86", "32")
    } else {
        panic!("unknown arch/pointer width: {}", target)
    };
    let (os, family, env) = if target.contains("unknown-linux") {
        ("linux", "unix", "gnu")
    } else if target.contains("apple-darwin") {
        ("macos", "unix", "")
    } else {
        panic!("unknown os/family width: {}", target)
    };

    let mk = attr::mk_name_value_item_str;
    let s = InternedString::new;
    cfg.push(attr::mk_word_item(s(family)));
    cfg.push(mk(s("target_os"), s(os)));
    cfg.push(mk(s("target_family"), s(family)));
    cfg.push(mk(s("target_arch"), s(arch)));
    // skip endianness
    cfg.push(mk(s("target_pointer_width"), s(target_pointer_width)));
    cfg.push(mk(s("target_env"), s(env)));
}

impl<'a> TestGenerator<'a> {
    fn test_type(&mut self, ty: &str) {
        let cty = if ty.starts_with("c_") {
            match &ty[2..].replace("long", " long")[..] {
                s if s.starts_with("u") => format!("unsigned {}", &s[1..]),
                "short" => format!("short"),
                s if s.starts_with("s") => format!("signed {}", &s[1..]),
                s => s.to_string(),
            }
        } else {
            (match ty {
                "sighandler_t" => return,
                ty => ty,
            }).to_string()
        };
        self.test_size_align(ty, &cty);
    }

    fn test_struct(&mut self, ty: &str, s: &ast::StructDef) {
        let cty = match ty {
            t if ty.starts_with("pthread") => t.to_string(),
            "glob_t" => "glob_t".to_string(),
            "ip6_mreq" => "struct ipv6_mreq".to_string(),
            s => format!("struct {}", s),
        };
        self.test_size_align(ty, &cty);

        writeln!(self.rust, r#"
            #[test]
            fn field_offset_size_{ty}() {{
        "#, ty = ty);
        for field in s.fields.iter() {
            let name = match field.node.kind {
                ast::NamedField(name, ast::Public) => name,
                ast::NamedField(_, ast::Inherited) => continue,
                ast::UnnamedField(..) => panic!("no tuple structs in FFI"),
            };

            let cname = match &name.to_string()[..] {
                s if s.ends_with("_nsec") && ty == "stat" => {
                    s.replace("_nsec", "spec.tv_nsec")
                }
                s => s.to_string(),
            };

            writeln!(self.c, r#"
                uint64_t __test_offset_{ty}_{rust_field}() {{
                    return offsetof({cty}, {c_field});
                }}
                uint64_t __test_size_{ty}_{rust_field}() {{
                    {cty}* foo = NULL;
                    return sizeof(foo->{c_field});
                }}
            "#, ty = ty, cty = cty, rust_field = name, c_field = cname);
            writeln!(self.rust, r#"
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
            "#, ty = ty, field = name);
        }
        writeln!(self.rust, r#"
            }}
        "#);
    }

    fn test_size_align(&mut self, rust: &str, c: &str) {
        writeln!(self.c, r#"
            uint64_t __test_size_{ty}() {{ return sizeof({cty}); }}
            uint64_t __test_align_{ty}() {{ return alignof({cty}); }}
        "#, ty = rust, cty = c);
        writeln!(self.rust, r#"
            #[test]
            fn size_align_{ty}() {{
                extern {{
                    fn __test_size_{ty}() -> u64;
                    fn __test_align_{ty}() -> u64;
                }}
                unsafe {{
                    same(mem::size_of::<{ty}>() as u64,
                         __test_size_{ty}(), "size");
                    same(mem::align_of::<{ty}>() as u64,
                         __test_align_{ty}(), "align");
                }}
            }}
        "#, ty = rust);
    }

    fn assert_no_generics(&self, _i: &ast::Item, generics: &ast::Generics) {
         assert!(generics.lifetimes.len() == 0);
         assert!(generics.ty_params.len() == 0);
         assert!(generics.where_clause.predicates.len() == 0);
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

             _ => {}
         }
         visit::walk_item(self, i)
     }
}
