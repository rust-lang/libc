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
use syntax::diagnostic::SpanHandler;
use syntax::ext::base::SyntaxExtension;
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

struct TestGenerator<'a> {
    target: String,
    rust: Box<Write>,
    c: Box<Write>,
    sh: &'a SpanHandler,
    structs: HashSet<String>,
    abi: Abi,
}

struct StructFinder {
    structs: HashSet<String>,
}

impl<'a> TestGenerator<'a> {
    fn defines(&self) -> Vec<&'static str> {
        let mut ret = Vec::new();

        // Pull in extra goodies on linux
        if self.target.contains("unknown-linux-gnu") {
            ret.push("_GNU_SOURCE");
        }

        // MSVC doesn't have stdalign.h so get alignof ourselves
        if self.target.contains("msvc") {
            ret.push("alignof __alignof");
        }

        // android also doesn't have stdalign.h so get alignof ourselves
        if self.target.contains("android") {
            ret.push("alignof __alignof__");
        }

        // Pull in extra goodies on mingw
        if self.target.contains("windows") {
            ret.push("_WIN32_WINNT 0x8000");
        }
        return ret
    }

    fn headers(&self) -> Vec<&'static str> {
        let mut base = Vec::new();

        base.extend([
            "errno.h",
            "fcntl.h",
            "limits.h",
            "stddef.h",
            "stdint.h",
            "stdio.h",
            "stdlib.h",
            "sys/stat.h",
            "sys/types.h",
            "time.h",
            "wchar.h",
        ].iter().cloned());

        if self.target.contains("apple-darwin") {
            base.push("mach-o/dyld.h");
            base.push("mach/mach_time.h");
        }

        if self.target.contains("unknown-linux") ||
           self.target.contains("android") {
            base.push("linux/if_packet.h");
            base.push("net/ethernet.h");
        }

        if self.target.contains("windows") {
            base.push("winsock2.h"); // must be before windows.h

            base.push("direct.h");
            base.push("io.h");
            base.push("sys/utime.h");
            base.push("windows.h");
            base.push("process.h");
            base.push("ws2ipdef.h");

            if self.target.contains("gnu") {
                base.push("stdalign.h");
                base.push("ws2tcpip.h");
            }
        } else {
            base.push("ctype.h");
            base.push("dirent.h");
            base.push("net/if.h");
            base.push("netdb.h");
            base.push("netinet/in.h");
            base.push("netinet/ip.h");
            base.push("netinet/tcp.h");
            base.push("pthread.h");
            base.push("signal.h");
            base.push("string.h");
            base.push("sys/file.h");
            base.push("sys/ioctl.h");
            base.push("sys/mman.h");
            base.push("sys/resource.h");
            base.push("sys/socket.h");
            base.push("sys/time.h");
            base.push("sys/un.h");
            base.push("sys/wait.h");
            base.push("unistd.h");
            base.push("utime.h");

            if self.target.contains("android") {
                base.push("arpa/inet.h");
            } else {
                base.push("glob.h");
                base.push("ifaddrs.h");
                base.push("stdalign.h");
                base.push("sys/sysctl.h");
            }
        }

        return base
    }

    fn rust2c(&self, ty: &str) -> String {
        let windows = self.target.contains("windows");
        match ty {
            t if t.starts_with("c_") => {
                match &ty[2..].replace("long", " long")[..] {
                    s if s.starts_with("u") => format!("unsigned {}", &s[1..]),
                    "short" => format!("short"),
                    s if s.starts_with("s") => format!("signed {}", &s[1..]),
                    s => s.to_string(),
                }
            }

            // Just pass all these through, no need for a "struct" prefix
            "glob_t" |
            "FILE" |
            "DIR" |
            "fpos_t" => ty.to_string(),
            t if t.starts_with("pthread") => t.to_string(),

            // Windows uppercase structs don't have `struct` in front, there's a
            // few special cases for windows, and then otherwise put `struct` in
            // front of everything.
            t if self.structs.contains(t) => {
                if windows && ty.chars().next().unwrap().is_uppercase() {
                    t.to_string()
                } else if windows && t == "stat" {
                    "struct __stat64".to_string()
                } else if windows && t == "utimbuf" {
                    "struct __utimbuf64".to_string()
                } else {
                    format!("struct {}", t)
                }
            }

            // Fixup a few types on windows that don't actually exist.
            "time64_t" if windows => "__time64_t".to_string(),
            "ssize_t" if windows => "SSIZE_T".to_string(),

            t => t.to_string(),
        }
    }

    fn rust2cfield(&self, struct_: &str, field: &str) -> String {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_ == "stat" => {
                if self.target.contains("apple-darwin") {
                    s.replace("_nsec", "spec.tv_nsec")
                } else if self.target.contains("android") {
                    s.to_string()
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
        } else if self.target.starts_with("arm") {
            ("arm", "32")
        } else {
            panic!("unknown arch/pointer width: {}", self.target)
        };
        let (os, family, env) = if self.target.contains("unknown-linux-gnu") {
            ("linux", "unix", "gnu")
        } else if self.target.contains("unknown-linux-musl") {
            ("linux", "unix", "musl")
        } else if self.target.contains("apple-darwin") {
            ("macos", "unix", "")
        } else if self.target.contains("windows-msvc") {
            ("windows", "windows", "msvc")
        } else if self.target.contains("windows-gnu") {
            ("windows", "windows", "gnu")
        } else if self.target.contains("android") {
            ("android", "unix", "")
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
        abi: Abi::C,
    };

    // Parse the libc crate
    let src = Path::new("../src/lib.rs");
    let cfg = Vec::new();
    let krate = parse::parse_crate_from_file(src, cfg, &sess);

    // expand macros
    let ecfg = expand::ExpansionConfig::default("libc".to_string());
    let exts = vec![
        (intern("macro_rules"), SyntaxExtension::MacroRulesTT),
    ];
    let mut krate = expand::expand_crate(&sess, ecfg, Vec::new(),
                                         exts, &mut Vec::new(), krate);

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

    // Compile our C shim to be linked into tests
    let mut cfg = gcc::Config::new();
    cfg.file(out.join("all.c"));

    if tg.target.contains("msvc") {
        cfg.flag("/W3").flag("/Wall").flag("/WX")
           .flag("/wd4820")  // weird warning about adding padding?
           .flag("/wd4100")  // don't warn about unused parameters
           .flag("/wd4996"); // don't warn about deprecated functions
    } else {
        cfg.flag("-Wall").flag("-Wextra").flag("-Werror")
           .flag("-Wno-unused-parameter");
    }

    drop(tg);
    cfg.compile("liball.a");
}

impl<'a> TestGenerator<'a> {
    fn test_type(&mut self, ty: &str) {
        match ty {
            // sighandler_t is crazy across platforms
            "sighandler_t" => return,

            // Not actually defined on android, but it's not hurting anyone
            "in_port_t" if self.target.contains("android") => return,
            _ => {}
        }
        let c = self.rust_ty_to_c_ty(ty);
        self.test_size_align(ty, &c);
    }

    fn test_struct(&mut self, ty: &str, s: &ast::StructDef) {
        let cty = self.rust_ty_to_c_ty(ty);
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
                uint64_t __test_offset_{ty}_{rust_field}(void) {{
                    return offsetof({cty}, {c_field});
                }}
                uint64_t __test_size_{ty}_{rust_field}(void) {{
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
            uint64_t __test_size_{ty}(void) {{ return sizeof({cty}); }}
            uint64_t __test_align_{ty}(void) {{ return alignof({cty}); }}
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

    fn rust_ty_to_c_ty(&self, mut rust_ty: &str) -> String {
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
        let mingw = self.target.contains("windows-gnu");

        // Apparently these don't exist in mingw headers?
        match name {
            "MEM_RESET_UNDO" |
            "FILE_ATTRIBUTE_NO_SCRUB_DATA" |
            "FILE_ATTRIBUTE_INTEGRITY_STREAM" |
            "ERROR_NOTHING_TO_TERMINATE" if mingw => return,
            _ => {}
        }

        let cty = self.rust_ty_to_c_ty(rust_ty);

        // SIG_IGN has weird types on platforms, just worry about it as a size_t
        let cast = if name == "SIG_IGN" {"(size_t)"} else {""};

        t!(writeln!(self.c, r#"
            int __test_const_{name}({cty} *outptr) {{
                *outptr = {cast}({name});
                return 1;
            }}
        "#, name = name, cast = cast, cty = cty));
        t!(writeln!(self.rust, r#"
            #[test]
            fn const_{name}() {{
                extern {{
                    fn __test_const_{name}(out: *mut {ty}) -> c_int;
                }}
                unsafe {{
                    let mut o = mem::zeroed();
                    if __test_const_{name}(&mut o) == 0 {{
                        panic!("not defined");
                    }} else {{
                        same({name}, o, "value");
                    }}
                }}
            }}
        "#, ty = rust_ty, name = name));
    }

    fn test_extern_fn(&mut self, name: &str, cname: &str,
                      args: &[String], ret: &str,
                      variadic: bool, abi: Abi) {
        match name {
            // manually verified
            "execv" |
            "execve" |
            "execvp" |
            "execvpe" |
            "glob" |
            "getrlimit" |
            "setrlimit" |
            "signal" |
            "getopt" => return,
            _ => {}
        }
        let args = if args.len() == 0 && !variadic {
            "void".to_string()
        } else {
            args.iter().map(|a| self.rust_ty_to_c_ty(a)).collect::<Vec<_>>()
                .connect(", ") + if variadic {", ..."} else {""}
        };
        let cret = self.rust_ty_to_c_ty(ret);
        let abi = match abi {
            Abi::C => "",
            Abi::Stdcall => "__stdcall ",
            Abi::System if self.target.contains("i686-pc-windows") => {
                "__stdcall "
            }
            Abi::System => "",
            a => panic!("unknown ABI: {}", a),
        };
        t!(writeln!(self.c, r#"
            {ret} ({abi}*__test_fn_{name}(void))({args}) {{
                return {cname};
            }}
        "#, name = name, cname = cname, args = args, ret = cret, abi = abi));
        t!(writeln!(self.rust, r#"
            #[test]
            #[cfg_attr(windows, ignore)] // FIXME -- dllimport weirdness?
            fn fn_{name}() {{
                extern {{
                    fn __test_fn_{name}() -> size_t;
                }}
                unsafe {{
                    same({name} as usize,
                         __test_fn_{name}() as usize, "function pointer");
                }}
            }}
        "#, name = name));
    }

    fn assert_no_generics(&self, _i: ast::Ident, generics: &ast::Generics) {
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
            ast::TyBareFn(ref t) => {
                assert!(t.lifetimes.len() == 0);
                let (ret, mut args, variadic) = self.decl2rust(&t.decl);
                assert!(!variadic);
                if args.len() == 0 {
                    args.push("void".to_string());
                }
                format!("{}(*)({})", ret, args.connect(", "))
            }
            _ => panic!("unknown ty {:?}", ty),
        }
    }

    fn decl2rust(&self, decl: &ast::FnDecl) -> (String, Vec<String>, bool) {
        let args = decl.inputs.iter().map(|arg| {
            self.ty2name(&arg.ty)
        }).collect::<Vec<_>>();
        let ret = match decl.output {
            ast::NoReturn(..) |
            ast::DefaultReturn(..) => "void".to_string(),
            ast::Return(ref t) => self.ty2name(t),
        };
        (ret, args, decl.variadic)
    }
}

impl<'a, 'v> Visitor<'v> for TestGenerator<'a> {
    fn visit_item(&mut self, i: &'v ast::Item) {
        let prev_abi = self.abi;
        match i.node {
            ast::ItemTy(_, ref generics) => {
                self.assert_no_generics(i.ident, generics);
                self.test_type(&i.ident.to_string());
            }

            ast::ItemStruct(ref s, ref generics) => {
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

            ast::ItemConst(ref ty, _) => {
                let ty = self.ty2name(ty);
                self.test_const(&i.ident.to_string(), &ty);
            }

            ast::ItemForeignMod(ref fm) => {
                self.abi = fm.abi;
            }

            _ => {}
        }
        visit::walk_item(self, i);
        self.abi = prev_abi;
    }

    fn visit_foreign_item(&mut self, i: &'v ast::ForeignItem) {
        match i.node {
            ast::ForeignItemFn(ref decl, ref generics) => {
                self.assert_no_generics(i.ident, generics);
                let (ret, args, variadic) = self.decl2rust(decl);
                let cname = match attr::first_attr_value_str_by_name(&i.attrs,
                                                                     "link_name") {
                    Some(ref i) if !i.to_string().contains("$") => {
                        i.to_string()
                    }
                    _ => i.ident.to_string(),
                };
                let abi = self.abi;
                self.test_extern_fn(&i.ident.to_string(), &cname, &args, &ret,
                                    variadic, abi);
            }
            ast::ForeignItemStatic(_, _) => {
            }
        }
        visit::walk_foreign_item(self, i)
    }
}

impl<'v> Visitor<'v> for StructFinder {
    fn visit_item(&mut self, i: &'v ast::Item) {
        match i.node {
            ast::ItemStruct(..) => {
                self.structs.insert(i.ident.to_string());
            }
            ast::ItemEnum(..) => {
                self.structs.insert(i.ident.to_string());
            }

            _ => {}
        }
        visit::walk_item(self, i)
    }
}
