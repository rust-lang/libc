#![allow(unused_must_use)]

extern crate gcc;
extern crate syntex_syntax as syntax;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use syntax::ast;
use syntax::parse::token::InternedString;
use syntax::attr;
use syntax::parse::{self, ParseSess};
use syntax::visit::{self, Visitor};

struct TestGenerator {
    rust: Box<Write>,
    c: Box<Write>,
}

fn main() {
    let sess = ParseSess::new();

    let src = Path::new("../src/lib.rs");
    let cfg = Vec::new();
    let mut krate = parse::parse_crate_from_file(src, cfg, &sess);
    build_cfg(&mut krate.config);

    let mut gated_cfgs = Vec::new();
    let krate = syntax::config::strip_unconfigured_items(&sess.span_diagnostic,
                                                         krate,
                                                         &mut gated_cfgs);

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let rust_out = BufWriter::new(File::create(out.join("all.rs")).unwrap());
    let mut c_out = BufWriter::new(File::create(out.join("all.c")).unwrap());

    writeln!(c_out, "
#include <netinet/in.h>
#include <netinet/ip.h>
#include <signal.h>
#include <stdint.h>
#include <sys/types.h>
#include <time.h>
");

    visit::walk_crate(&mut TestGenerator {
        rust: Box::new(rust_out),
        c: Box::new(c_out),
    }, &krate);

    gcc::Config::new()
                .file(out.join("all.c"))
                .compile("liball.a");
}

fn build_cfg(cfg: &mut ast::CrateConfig) {
    let target = env::var("TARGET").unwrap();

    let (arch, target_pointer_width) = if target.starts_with("x86_64") {
        ("x86_64", "64")
    } else if target.starts_with("i686") {
        ("x86", "32")
    } else {
        panic!("unknown arch/pointer width: {}", target)
    };
    let (os, family, env) = if target.contains("unknown-linux") {
        ("linux", "unix", "gnu")
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

impl TestGenerator {
    fn typedef(&mut self, ty: &str) {
        let cty = if ty.starts_with("c_") {
            let rest = ty[2..].replace("long", " long");
            if rest.starts_with("u") {
                format!("unsigned {}", &rest[1..])
            } else {
                rest
            }
        } else {
            (match ty {
                ty => ty,
            }).to_string()
        };
        writeln!(self.c, r#"
            uint64_t ty_{ty}_size() {{
                return sizeof({cty});
            }}
        "#, ty = ty, cty = cty);
        writeln!(self.rust, r#"
            #[test]
            fn test_{ty}_size() {{
                extern {{ fn ty_{ty}_size() -> u64; }}
                assert_eq!(mem::size_of::<libc::{ty}>() as u64,
                           unsafe {{ ty_{ty}_size() }});
            }}
        "#, ty = ty);
    }
}

impl<'v> Visitor<'v> for TestGenerator {
     fn visit_item(&mut self, i: &'v ast::Item) {
         match i.node {
             ast::ItemTy(_, ref generics) => {
                 assert!(generics.lifetimes.len() == 0);
                 assert!(generics.ty_params.len() == 0);
                 assert!(generics.where_clause.predicates.len() == 0);
                 self.typedef(&i.ident.to_string());
             }

             _ => {}
         }
         visit::walk_item(self, i)
     }
}
