use std::env;
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Target {
    // Info about the target
    pub triple: String,
    pub triple_split: Vec<String>,
    pub arch: String,
    pub abi: String,
    pub endian: Endian,
    pub env: String,
    pub families: Vec<String>,
    pub os: String,
    pub pointer_width: PointerWidth,
    pub target_features: Vec<String>,
    pub vendor: String,

    // General build info
    pub manifest_dir: PathBuf,
    pub out_dir: PathBuf,
    pub cargo_features: Vec<String>,
}

impl Target {
    pub fn from_env() -> Self {
        let triple = env::var("TARGET").unwrap();
        let triple_split = triple.split('-').map(ToOwned::to_owned).collect();
        let families = env::var("CARGO_CFG_TARGET_FAMILY")
            .map(|feats| feats.split(',').map(ToOwned::to_owned).collect())
            .unwrap_or_default();
        let target_features = env::var("CARGO_CFG_TARGET_FEATURE")
            .map(|feats| feats.split(',').map(ToOwned::to_owned).collect())
            .unwrap_or_default();
        let cargo_features = env::vars()
            .filter_map(|(name, _value)| name.strip_prefix("CARGO_FEATURE_").map(ToOwned::to_owned))
            .map(|s| s.to_lowercase().replace("_", "-"))
            .collect();
        let pointer_width = match env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().as_str() {
            "32" => PointerWidth::P32,
            "64" => PointerWidth::P64,
            x => panic!("unsupported pointer width {x}"),
        };
        let endian = match env::var("CARGO_CFG_TARGET_ENDIAN").unwrap().as_str() {
            "little" => Endian::Little,
            "big" => Endian::Big,
            x => panic!("unsupported endian {x}"),
        };

        Self {
            triple,
            triple_split,
            arch: env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            abi: env::var("CARGO_CFG_TARGET_ABI").unwrap(),
            endian,
            env: env::var("CARGO_CFG_TARGET_ENV").unwrap(),
            families,
            os: env::var("CARGO_CFG_TARGET_OS").unwrap(),
            pointer_width,
            target_features,
            vendor: env::var("CARGO_CFG_TARGET_VENDOR").unwrap(),

            manifest_dir: PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()),
            out_dir: PathBuf::from(env::var("OUT_DIR").unwrap()),
            cargo_features,
        }
    }
}

#[allow(dead_code)]
impl Target {
    /* arch */

    pub fn aarch64(&self) -> bool {
        self.arch == "aarch64"
    }

    pub fn arm32(&self) -> bool {
        self.arch == "arm"
    }

    pub fn hexagon(&self) -> bool {
        self.arch == "hexagon"
    }

    pub fn loongarch32(&self) -> bool {
        self.arch == "loongarch32"
    }

    pub fn loongarch64(&self) -> bool {
        self.arch == "loongarch64"
    }

    pub fn loongarch(&self) -> bool {
        self.loongarch32() || self.loongarch64()
    }

    pub fn mips32(&self) -> bool {
        self.arch == "mips32" || self.arch == "mips32r6"
    }

    pub fn mips64(&self) -> bool {
        self.arch == "mips64" || self.arch == "mips64r6"
    }

    pub fn mips(&self) -> bool {
        self.mips32() || self.mips64()
    }

    pub fn ppc32(&self) -> bool {
        self.arch == "powerpc"
    }

    pub fn ppc64(&self) -> bool {
        self.arch == "powerpc64"
    }

    pub fn ppc64be(&self) -> bool {
        self.ppc64() && self.endian == Endian::Big
    }

    pub fn ppc64le(&self) -> bool {
        self.ppc64() && self.endian == Endian::Little
    }

    pub fn ppc(&self) -> bool {
        self.ppc32() || self.ppc64()
    }

    pub fn riscv32(&self) -> bool {
        self.arch == "riscv32"
    }

    pub fn riscv64(&self) -> bool {
        self.arch == "riscv64"
    }

    pub fn riscv(&self) -> bool {
        self.riscv32() || self.riscv64()
    }

    pub fn s390x(&self) -> bool {
        self.arch == "s390x"
    }

    pub fn sparc32(&self) -> bool {
        self.arch == "sparc"
    }

    pub fn sparc64(&self) -> bool {
        self.arch == "sparc64"
    }

    pub fn sparc(&self) -> bool {
        self.sparc32() || self.sparc64()
    }

    pub fn wasm32(&self) -> bool {
        self.arch == "wasm32"
    }

    pub fn x86_32(&self) -> bool {
        self.arch == "x86"
    }

    pub fn x86_64(&self) -> bool {
        self.arch == "x86_64"
    }

    pub fn x86(&self) -> bool {
        self.x86_32() || self.x86_64()
    }

    /* abi */

    pub fn eabihf(&self) -> bool {
        self.abi == "eabihf"
    }

    pub fn pauthtest(&self) -> bool {
        self.abi == "pauthtest"
    }

    pub fn x32(&self) -> bool {
        self.abi == "x32"
    }

    /* env */

    pub fn gnu(&self) -> bool {
        self.env == "gnu"
    }

    pub fn msvc(&self) -> bool {
        self.env == "msvc"
    }

    pub fn musl(&self) -> bool {
        self.env == "musl" || self.env == "ohos"
    }

    pub fn nto_iosock(&self) -> bool {
        self.env.contains("nto") && self.env.contains("iosock")
    }

    pub fn uclibc(&self) -> bool {
        self.env == "uclibc"
    }

    /* os */

    pub fn aix(&self) -> bool {
        self.os == "aix"
    }

    pub fn android(&self) -> bool {
        self.os == "android"
    }

    pub fn cygwin(&self) -> bool {
        self.os == "cygwin"
    }

    pub fn dragonfly(&self) -> bool {
        self.os == "dragonfly"
    }

    pub fn emscripten(&self) -> bool {
        self.os == "emscripten"
    }

    pub fn freebsd(&self) -> bool {
        self.os == "freebsd"
    }

    pub fn fuchsia(&self) -> bool {
        self.os == "fuchsia"
    }

    pub fn haiku(&self) -> bool {
        self.os == "haiku"
    }

    pub fn illumos(&self) -> bool {
        self.os == "illumos"
    }

    pub fn l4re(&self) -> bool {
        self.os == "l4re"
    }

    pub fn linux(&self) -> bool {
        self.os == "linux"
    }

    pub fn netbsd(&self) -> bool {
        self.os == "netbsd"
    }

    pub fn nto(&self) -> bool {
        self.os == "nto"
    }

    pub fn openbsd(&self) -> bool {
        self.os == "openbsd"
    }

    pub fn qurt(&self) -> bool {
        self.os == "qurt"
    }

    pub fn redox(&self) -> bool {
        self.os == "redox"
    }

    pub fn solaris(&self) -> bool {
        self.os == "solaris"
    }

    pub fn vxworks(&self) -> bool {
        self.os == "vxworks"
    }

    pub fn wali(&self) -> bool {
        self.os == "linux" && self.wasm32()
    }

    pub fn wasi(&self) -> bool {
        self.os == "wasi"
    }

    pub fn wasip2(&self) -> bool {
        self.wasi() && self.env == "p2"
    }

    pub fn win(&self) -> bool {
        self.os == "windows"
    }

    /* vendor */

    pub fn apple(&self) -> bool {
        self.vendor == "apple"
    }

    /* other */

    pub fn p32(&self) -> bool {
        self.pointer_width == PointerWidth::P32
    }

    pub fn p64(&self) -> bool {
        self.pointer_width == PointerWidth::P64
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerWidth {
    P32,
    P64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endian {
    Little,
    Big,
}
