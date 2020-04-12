use core::prelude::v1::*;
use ::*;

pub type c_char = u8;

pub type c_long = i64;
pub type c_ulong = u64;

pub type rlim_t = u64;

pub const HAVE_INITFINI_ARRAY: i32 = 1;
pub const WINT_MIN: i32 = 0;
pub const LITTLE_ENDIAN: i32 = 1234;
pub const BIG_ENDIAN: i32 = 4321;
pub const PDP_ENDIAN: i32 = 3412;
pub const BYTE_ORDER: i32 = 1234;
pub const FD_SETSIZE: i32 = 64;
pub const SCHED_OTHER: i32 = 0;
pub const SCHED_FIFO: i32 = 1;
pub const SCHED_RR: i32 = 2;
pub const PTHREAD_SCOPE_PROCESS: i32 = 0;
pub const PTHREAD_SCOPE_SYSTEM: i32 = 1;
pub const PTHREAD_INHERIT_SCHED: i32 = 1;
pub const PTHREAD_EXPLICIT_SCHED: i32 = 2;
pub const PTHREAD_CREATE_DETACHED: i32 = 0;
pub const PTHREAD_CREATE_JOINABLE: i32 = 1;
pub const F_ULOCK: i32 = 0;
pub const F_LOCK: i32 = 1;
pub const F_TLOCK: i32 = 2;
pub const F_TEST: i32 = 3;
pub const F_OK: i32 = 0;
pub const R_OK: i32 = 4;
pub const W_OK: i32 = 2;
pub const X_OK: i32 = 1;
pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;
pub const MB_LEN_MAX: i32 = 16;
pub const NR_OPEN: i32 = 1024;
pub const NGROUPS_MAX: i32 = 65536;
pub const ARG_MAX: i32 = 131072;
pub const LINK_MAX: i32 = 127;
pub const MAX_CANON: i32 = 255;
pub const MAX_INPUT: i32 = 255;
pub const NAME_MAX: i32 = 255;
pub const PATH_MAX: i32 = 4096;
pub const PIPE_BUF: i32 = 4096;
pub const XATTR_NAME_MAX: i32 = 255;
pub const XATTR_SIZE_MAX: i32 = 65536;
pub const XATTR_LIST_MAX: i32 = 65536;
pub const RTSIG_MAX: i32 = 32;
pub const PTHREAD_KEYS_MAX: i32 = 1024;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: i32 = 4;
pub const AIO_PRIO_DELTA_MAX: i32 = 20;
pub const PTHREAD_STACK_MIN: usize = 16384;
pub const DELAYTIMER_MAX: i32 = 2147483647;
pub const TTY_NAME_MAX: i32 = 32;
pub const LOGIN_NAME_MAX: i32 = 256;
pub const HOST_NAME_MAX: i32 = 64;
pub const MQ_PRIO_MAX: i32 = 32768;
pub const SEM_VALUE_MAX: i32 = 2147483647;
pub const BC_BASE_MAX: i32 = 99;
pub const BC_DIM_MAX: i32 = 2048;
pub const BC_SCALE_MAX: i32 = 99;
pub const BC_STRING_MAX: i32 = 1000;
pub const COLL_WEIGHTS_MAX: i32 = 255;
pub const EXPR_NEST_MAX: i32 = 32;
pub const LINE_MAX: i32 = 2048;
pub const CHARCLASS_NAME_MAX: i32 = 2048;
pub const RE_DUP_MAX: i32 = 32767;
pub const TSS_DTOR_ITERATIONS: i32 = 4;
pub const CLOCKS_PER_SEC: i32 = 1000;
pub const CLK_TCK: i32 = 1000;
pub const CLOCK_ENABLED: i32 = 1;
pub const CLOCK_DISABLED: i32 = 0;
pub const CLOCK_ALLOWED: i32 = 1;
pub const CLOCK_DISALLOWED: i32 = 0;
pub const CLOCK_REALTIME: clockid_t = 1;
pub const CLOCK_MONOTONIC: clockid_t = 4;
pub const TIMER_ABSTIME: i32 = 4;
pub const FP_NAN: i32 = 0;
pub const FP_INFINITE: i32 = 1;
pub const FP_ZERO: i32 = 2;
pub const FP_SUBNORMAL: i32 = 3;
pub const FP_NORMAL: i32 = 4;
pub const MATH_ERRNO: i32 = 1;
pub const MATH_ERREXCEPT: i32 = 2;
pub const math_errhandling: i32 = 1;
pub const DOMAIN: i32 = 1;
pub const SING: i32 = 2;
pub const OVERFLOW: i32 = 3;
pub const UNDERFLOW: i32 = 4;
pub const TLOSS: i32 = 5;
pub const PLOSS: i32 = 6;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const M_TWOPI: f64 = 6.283185307179586;
pub const M_SQRTPI: f64 = 1.772453850905516;
pub const M_SQRT3: f64 = 1.7320508075688772;
pub const M_IVLN10: f64 = 0.4342944819032518;
pub const M_LOG2_E: f64 = 0.6931471805599453;
pub const EI_NIDENT: i32 = 16;
pub const EI_MAG0: i32 = 0;
pub const ELFMAG0: i32 = 127;
pub const EI_MAG1: i32 = 1;
pub const ELFMAG1: u8 = 69u8;
pub const EI_MAG2: i32 = 2;
pub const ELFMAG2: u8 = 76u8;
pub const EI_MAG3: i32 = 3;
pub const ELFMAG3: u8 = 70u8;
pub const ELFMAG: &'static [u8; 5usize] = b"\x7FELF\0";
pub const SELFMAG: i32 = 4;
pub const EI_CLASS: i32 = 4;
pub const ELFCLASSNONE: i32 = 0;
pub const ELFCLASS32: i32 = 1;
pub const ELFCLASS64: i32 = 2;
pub const ELFCLASSNUM: i32 = 3;
pub const EI_DATA: i32 = 5;
pub const ELFDATANONE: i32 = 0;
pub const ELFDATA2LSB: i32 = 1;
pub const ELFDATA2MSB: i32 = 2;
pub const ELFDATANUM: i32 = 3;
pub const EI_VERSION: i32 = 6;
pub const EI_OSABI: i32 = 7;
pub const ELFOSABI_NONE: i32 = 0;
pub const ELFOSABI_SYSV: i32 = 0;
pub const ELFOSABI_HPUX: i32 = 1;
pub const ELFOSABI_NETBSD: i32 = 2;
pub const ELFOSABI_LINUX: i32 = 3;
pub const ELFOSABI_GNU: i32 = 3;
pub const ELFOSABI_SOLARIS: i32 = 6;
pub const ELFOSABI_AIX: i32 = 7;
pub const ELFOSABI_IRIX: i32 = 8;
pub const ELFOSABI_FREEBSD: i32 = 9;
pub const ELFOSABI_TRU64: i32 = 10;
pub const ELFOSABI_MODESTO: i32 = 11;
pub const ELFOSABI_OPENBSD: i32 = 12;
pub const ELFOSABI_ARM: i32 = 97;
pub const ELFOSABI_STANDALONE: i32 = 255;
pub const EI_ABIVERSION: i32 = 8;
pub const EI_PAD: i32 = 9;
pub const ET_NONE: i32 = 0;
pub const ET_REL: i32 = 1;
pub const ET_EXEC: i32 = 2;
pub const ET_DYN: i32 = 3;
pub const ET_CORE: i32 = 4;
pub const ET_NUM: i32 = 5;
pub const ET_LOOS: i32 = 65024;
pub const ET_HIOS: i32 = 65279;
pub const ET_LOPROC: i32 = 65280;
pub const ET_HIPROC: i32 = 65535;
pub const EM_NONE: i32 = 0;
pub const EM_M32: i32 = 1;
pub const EM_SPARC: i32 = 2;
pub const EM_386: i32 = 3;
pub const EM_68K: i32 = 4;
pub const EM_88K: i32 = 5;
pub const EM_860: i32 = 7;
pub const EM_MIPS: i32 = 8;
pub const EM_S370: i32 = 9;
pub const EM_MIPS_RS3_LE: i32 = 10;
pub const EM_PARISC: i32 = 15;
pub const EM_VPP500: i32 = 17;
pub const EM_SPARC32PLUS: i32 = 18;
pub const EM_960: i32 = 19;
pub const EM_PPC: i32 = 20;
pub const EM_PPC64: i32 = 21;
pub const EM_S390: i32 = 22;
pub const EM_V800: i32 = 36;
pub const EM_FR20: i32 = 37;
pub const EM_RH32: i32 = 38;
pub const EM_RCE: i32 = 39;
pub const EM_ARM: i32 = 40;
pub const EM_FAKE_ALPHA: i32 = 41;
pub const EM_SH: i32 = 42;
pub const EM_SPARCV9: i32 = 43;
pub const EM_TRICORE: i32 = 44;
pub const EM_ARC: i32 = 45;
pub const EM_H8_300: i32 = 46;
pub const EM_H8_300H: i32 = 47;
pub const EM_H8S: i32 = 48;
pub const EM_H8_500: i32 = 49;
pub const EM_IA_64: i32 = 50;
pub const EM_MIPS_X: i32 = 51;
pub const EM_COLDFIRE: i32 = 52;
pub const EM_68HC12: i32 = 53;
pub const EM_MMA: i32 = 54;
pub const EM_PCP: i32 = 55;
pub const EM_NCPU: i32 = 56;
pub const EM_NDR1: i32 = 57;
pub const EM_STARCORE: i32 = 58;
pub const EM_ME16: i32 = 59;
pub const EM_ST100: i32 = 60;
pub const EM_TINYJ: i32 = 61;
pub const EM_X86_64: i32 = 62;
pub const EM_PDSP: i32 = 63;
pub const EM_FX66: i32 = 66;
pub const EM_ST9PLUS: i32 = 67;
pub const EM_ST7: i32 = 68;
pub const EM_68HC16: i32 = 69;
pub const EM_68HC11: i32 = 70;
pub const EM_68HC08: i32 = 71;
pub const EM_68HC05: i32 = 72;
pub const EM_SVX: i32 = 73;
pub const EM_ST19: i32 = 74;
pub const EM_VAX: i32 = 75;
pub const EM_CRIS: i32 = 76;
pub const EM_JAVELIN: i32 = 77;
pub const EM_FIREPATH: i32 = 78;
pub const EM_ZSP: i32 = 79;
pub const EM_MMIX: i32 = 80;
pub const EM_HUANY: i32 = 81;
pub const EM_PRISM: i32 = 82;
pub const EM_AVR: i32 = 83;
pub const EM_FR30: i32 = 84;
pub const EM_D10V: i32 = 85;
pub const EM_D30V: i32 = 86;
pub const EM_V850: i32 = 87;
pub const EM_M32R: i32 = 88;
pub const EM_MN10300: i32 = 89;
pub const EM_MN10200: i32 = 90;
pub const EM_PJ: i32 = 91;
pub const EM_OR1K: i32 = 92;
pub const EM_OPENRISC: i32 = 92;
pub const EM_ARC_A5: i32 = 93;
pub const EM_ARC_COMPACT: i32 = 93;
pub const EM_XTENSA: i32 = 94;
pub const EM_VIDEOCORE: i32 = 95;
pub const EM_TMM_GPP: i32 = 96;
pub const EM_NS32K: i32 = 97;
pub const EM_TPC: i32 = 98;
pub const EM_SNP1K: i32 = 99;
pub const EM_ST200: i32 = 100;
pub const EM_IP2K: i32 = 101;
pub const EM_MAX: i32 = 102;
pub const EM_CR: i32 = 103;
pub const EM_F2MC16: i32 = 104;
pub const EM_MSP430: i32 = 105;
pub const EM_BLACKFIN: i32 = 106;
pub const EM_SE_C33: i32 = 107;
pub const EM_SEP: i32 = 108;
pub const EM_ARCA: i32 = 109;
pub const EM_UNICORE: i32 = 110;
pub const EM_EXCESS: i32 = 111;
pub const EM_DXP: i32 = 112;
pub const EM_ALTERA_NIOS2: i32 = 113;
pub const EM_CRX: i32 = 114;
pub const EM_XGATE: i32 = 115;
pub const EM_C166: i32 = 116;
pub const EM_M16C: i32 = 117;
pub const EM_DSPIC30F: i32 = 118;
pub const EM_CE: i32 = 119;
pub const EM_M32C: i32 = 120;
pub const EM_TSK3000: i32 = 131;
pub const EM_RS08: i32 = 132;
pub const EM_SHARC: i32 = 133;
pub const EM_ECOG2: i32 = 134;
pub const EM_SCORE7: i32 = 135;
pub const EM_DSP24: i32 = 136;
pub const EM_VIDEOCORE3: i32 = 137;
pub const EM_LATTICEMICO32: i32 = 138;
pub const EM_SE_C17: i32 = 139;
pub const EM_TI_C6000: i32 = 140;
pub const EM_TI_C2000: i32 = 141;
pub const EM_TI_C5500: i32 = 142;
pub const EM_TI_ARP32: i32 = 143;
pub const EM_TI_PRU: i32 = 144;
pub const EM_MMDSP_PLUS: i32 = 160;
pub const EM_CYPRESS_M8C: i32 = 161;
pub const EM_R32C: i32 = 162;
pub const EM_TRIMEDIA: i32 = 163;
pub const EM_QDSP6: i32 = 164;
pub const EM_8051: i32 = 165;
pub const EM_STXP7X: i32 = 166;
pub const EM_NDS32: i32 = 167;
pub const EM_ECOG1X: i32 = 168;
pub const EM_MAXQ30: i32 = 169;
pub const EM_XIMO16: i32 = 170;
pub const EM_MANIK: i32 = 171;
pub const EM_CRAYNV2: i32 = 172;
pub const EM_RX: i32 = 173;
pub const EM_METAG: i32 = 174;
pub const EM_MCST_ELBRUS: i32 = 175;
pub const EM_ECOG16: i32 = 176;
pub const EM_CR16: i32 = 177;
pub const EM_ETPU: i32 = 178;
pub const EM_SLE9X: i32 = 179;
pub const EM_L10M: i32 = 180;
pub const EM_K10M: i32 = 181;
pub const EM_AARCH64: i32 = 183;
pub const EM_AVR32: i32 = 185;
pub const EM_STM8: i32 = 186;
pub const EM_TILE64: i32 = 187;
pub const EM_TILEPRO: i32 = 188;
pub const EM_MICROBLAZE: i32 = 189;
pub const EM_CUDA: i32 = 190;
pub const EM_TILEGX: i32 = 191;
pub const EM_CLOUDSHIELD: i32 = 192;
pub const EM_COREA_1ST: i32 = 193;
pub const EM_COREA_2ND: i32 = 194;
pub const EM_ARC_COMPACT2: i32 = 195;
pub const EM_OPEN8: i32 = 196;
pub const EM_RL78: i32 = 197;
pub const EM_VIDEOCORE5: i32 = 198;
pub const EM_78KOR: i32 = 199;
pub const EM_56800EX: i32 = 200;
pub const EM_BA1: i32 = 201;
pub const EM_BA2: i32 = 202;
pub const EM_XCORE: i32 = 203;
pub const EM_MCHP_PIC: i32 = 204;
pub const EM_KM32: i32 = 210;
pub const EM_KMX32: i32 = 211;
pub const EM_EMX16: i32 = 212;
pub const EM_EMX8: i32 = 213;
pub const EM_KVARC: i32 = 214;
pub const EM_CDP: i32 = 215;
pub const EM_COGE: i32 = 216;
pub const EM_COOL: i32 = 217;
pub const EM_NORC: i32 = 218;
pub const EM_CSR_KALIMBA: i32 = 219;
pub const EM_Z80: i32 = 220;
pub const EM_VISIUM: i32 = 221;
pub const EM_FT32: i32 = 222;
pub const EM_MOXIE: i32 = 223;
pub const EM_AMDGPU: i32 = 224;
pub const EM_RISCV: i32 = 243;
pub const EM_BPF: i32 = 247;
pub const EM_NUM: i32 = 248;
pub const EM_ALPHA: i32 = 36902;
pub const EV_NONE: i32 = 0;
pub const EV_CURRENT: i32 = 1;
pub const EV_NUM: i32 = 2;
pub const SHN_UNDEF: i32 = 0;
pub const SHN_LORESERVE: i32 = 65280;
pub const SHN_LOPROC: i32 = 65280;
pub const SHN_BEFORE: i32 = 65280;
pub const SHN_AFTER: i32 = 65281;
pub const SHN_HIPROC: i32 = 65311;
pub const SHN_LOOS: i32 = 65312;
pub const SHN_HIOS: i32 = 65343;
pub const SHN_ABS: i32 = 65521;
pub const SHN_COMMON: i32 = 65522;
pub const SHN_XINDEX: i32 = 65535;
pub const SHN_HIRESERVE: i32 = 65535;
pub const SHT_NULL: i32 = 0;
pub const SHT_PROGBITS: i32 = 1;
pub const SHT_SYMTAB: i32 = 2;
pub const SHT_STRTAB: i32 = 3;
pub const SHT_RELA: i32 = 4;
pub const SHT_HASH: i32 = 5;
pub const SHT_DYNAMIC: i32 = 6;
pub const SHT_NOTE: i32 = 7;
pub const SHT_NOBITS: i32 = 8;
pub const SHT_REL: i32 = 9;
pub const SHT_SHLIB: i32 = 10;
pub const SHT_DYNSYM: i32 = 11;
pub const SHT_INIT_ARRAY: i32 = 14;
pub const SHT_FINI_ARRAY: i32 = 15;
pub const SHT_PREINIT_ARRAY: i32 = 16;
pub const SHT_GROUP: i32 = 17;
pub const SHT_SYMTAB_SHNDX: i32 = 18;
pub const SHT_NUM: i32 = 19;
pub const SHT_LOOS: i32 = 1610612736;
pub const SHT_GNU_ATTRIBUTES: i32 = 1879048181;
pub const SHT_GNU_HASH: i32 = 1879048182;
pub const SHT_GNU_LIBLIST: i32 = 1879048183;
pub const SHT_CHECKSUM: i32 = 1879048184;
pub const SHT_LOSUNW: i32 = 1879048186;
pub const SHT_SUNW_move: i32 = 1879048186;
pub const SHT_SUNW_COMDAT: i32 = 1879048187;
pub const SHT_SUNW_syminfo: i32 = 1879048188;
pub const SHT_GNU_verdef: i32 = 1879048189;
pub const SHT_GNU_verneed: i32 = 1879048190;
pub const SHT_GNU_versym: i32 = 1879048191;
pub const SHT_HISUNW: i32 = 1879048191;
pub const SHT_HIOS: i32 = 1879048191;
pub const SHT_LOPROC: i32 = 1879048192;
pub const SHT_HIPROC: i32 = 2147483647;
pub const SHT_LOUSER: i32 = 2147483648;
pub const SHT_HIUSER: i32 = 2415919103;
pub const SHF_WRITE: i32 = 1;
pub const SHF_ALLOC: i32 = 2;
pub const SHF_EXECINSTR: i32 = 4;
pub const SHF_MERGE: i32 = 16;
pub const SHF_STRINGS: i32 = 32;
pub const SHF_INFO_LINK: i32 = 64;
pub const SHF_LINK_ORDER: i32 = 128;
pub const SHF_OS_NONCONFORMING: i32 = 256;
pub const SHF_GROUP: i32 = 512;
pub const SHF_TLS: i32 = 1024;
pub const SHF_COMPRESSED: i32 = 2048;
pub const SHF_MASKOS: i32 = 267386880;
pub const SHF_MASKPROC: i32 = 4026531840;
pub const SHF_ORDERED: i32 = 1073741824;
pub const SHF_EXCLUDE: i32 = 2147483648;
pub const ELFCOMPRESS_ZLIB: i32 = 1;
pub const ELFCOMPRESS_LOOS: i32 = 1610612736;
pub const ELFCOMPRESS_HIOS: i32 = 1879048191;
pub const ELFCOMPRESS_LOPROC: i32 = 1879048192;
pub const ELFCOMPRESS_HIPROC: i32 = 2147483647;
pub const GRP_COMDAT: i32 = 1;
pub const SYMINFO_BT_SELF: i32 = 65535;
pub const SYMINFO_BT_PARENT: i32 = 65534;
pub const SYMINFO_BT_LOWRESERVE: i32 = 65280;
pub const SYMINFO_FLG_DIRECT: i32 = 1;
pub const SYMINFO_FLG_PASSTHRU: i32 = 2;
pub const SYMINFO_FLG_COPY: i32 = 4;
pub const SYMINFO_FLG_LAZYLOAD: i32 = 8;
pub const SYMINFO_NONE: i32 = 0;
pub const SYMINFO_CURRENT: i32 = 1;
pub const SYMINFO_NUM: i32 = 2;
pub const STB_LOCAL: i32 = 0;
pub const STB_GLOBAL: i32 = 1;
pub const STB_WEAK: i32 = 2;
pub const STB_NUM: i32 = 3;
pub const STB_LOOS: i32 = 10;
pub const STB_GNU_UNIQUE: i32 = 10;
pub const STB_HIOS: i32 = 12;
pub const STB_LOPROC: i32 = 13;
pub const STB_HIPROC: i32 = 15;
pub const STT_NOTYPE: i32 = 0;
pub const STT_OBJECT: i32 = 1;
pub const STT_FUNC: i32 = 2;
pub const STT_SECTION: i32 = 3;
pub const STT_FILE: i32 = 4;
pub const STT_COMMON: i32 = 5;
pub const STT_TLS: i32 = 6;
pub const STT_NUM: i32 = 7;
pub const STT_LOOS: i32 = 10;
pub const STT_GNU_IFUNC: i32 = 10;
pub const STT_HIOS: i32 = 12;
pub const STT_LOPROC: i32 = 13;
pub const STT_HIPROC: i32 = 15;
pub const STN_UNDEF: i32 = 0;
pub const STV_DEFAULT: i32 = 0;
pub const STV_INTERNAL: i32 = 1;
pub const STV_HIDDEN: i32 = 2;
pub const STV_PROTECTED: i32 = 3;
pub const PT_NULL: i32 = 0;
pub const PT_LOAD: i32 = 1;
pub const PT_DYNAMIC: i32 = 2;
pub const PT_INTERP: i32 = 3;
pub const PT_NOTE: i32 = 4;
pub const PT_SHLIB: i32 = 5;
pub const PT_PHDR: i32 = 6;
pub const PT_TLS: i32 = 7;
pub const PT_NUM: i32 = 8;
pub const PT_LOOS: i32 = 1610612736;
pub const PT_GNU_EH_FRAME: i32 = 1685382480;
pub const PT_GNU_STACK: i32 = 1685382481;
pub const PT_GNU_RELRO: i32 = 1685382482;
pub const PT_LOSUNW: i32 = 1879048186;
pub const PT_SUNWBSS: i32 = 1879048186;
pub const PT_SUNWSTACK: i32 = 1879048187;
pub const PT_HISUNW: i32 = 1879048191;
pub const PT_HIOS: i32 = 1879048191;
pub const PT_LOPROC: i32 = 1879048192;
pub const PT_HIPROC: i32 = 2147483647;
pub const PN_XNUM: i32 = 65535;
pub const PF_X: i32 = 1;
pub const PF_W: i32 = 2;
pub const PF_R: i32 = 4;
pub const PF_MASKOS: i32 = 267386880;
pub const PF_MASKPROC: i32 = 4026531840;
pub const NT_PRSTATUS: i32 = 1;
pub const NT_FPREGSET: i32 = 2;
pub const NT_PRPSINFO: i32 = 3;
pub const NT_PRXREG: i32 = 4;
pub const NT_TASKSTRUCT: i32 = 4;
pub const NT_PLATFORM: i32 = 5;
pub const NT_AUXV: i32 = 6;
pub const NT_GWINDOWS: i32 = 7;
pub const NT_ASRS: i32 = 8;
pub const NT_PSTATUS: i32 = 10;
pub const NT_PSINFO: i32 = 13;
pub const NT_PRCRED: i32 = 14;
pub const NT_UTSNAME: i32 = 15;
pub const NT_LWPSTATUS: i32 = 16;
pub const NT_LWPSINFO: i32 = 17;
pub const NT_PRFPXREG: i32 = 20;
pub const NT_SIGINFO: i32 = 1397311305;
pub const NT_FILE: i32 = 1179208773;
pub const NT_PRXFPREG: i32 = 1189489535;
pub const NT_PPC_VMX: i32 = 256;
pub const NT_PPC_SPE: i32 = 257;
pub const NT_PPC_VSX: i32 = 258;
pub const NT_386_TLS: i32 = 512;
pub const NT_386_IOPERM: i32 = 513;
pub const NT_X86_XSTATE: i32 = 514;
pub const NT_S390_HIGH_GPRS: i32 = 768;
pub const NT_S390_TIMER: i32 = 769;
pub const NT_S390_TODCMP: i32 = 770;
pub const NT_S390_TODPREG: i32 = 771;
pub const NT_S390_CTRS: i32 = 772;
pub const NT_S390_PREFIX: i32 = 773;
pub const NT_S390_LAST_BREAK: i32 = 774;
pub const NT_S390_SYSTEM_CALL: i32 = 775;
pub const NT_S390_TDB: i32 = 776;
pub const NT_ARM_VFP: i32 = 1024;
pub const NT_ARM_TLS: i32 = 1025;
pub const NT_ARM_HW_BREAK: i32 = 1026;
pub const NT_ARM_HW_WATCH: i32 = 1027;
pub const NT_ARM_SYSTEM_CALL: i32 = 1028;
pub const NT_ARM_SVE: i32 = 1029;
pub const NT_METAG_CBUF: i32 = 1280;
pub const NT_METAG_RPIPE: i32 = 1281;
pub const NT_METAG_TLS: i32 = 1282;
pub const NT_VERSION: i32 = 1;
pub const DT_NULL: i32 = 0;
pub const DT_NEEDED: i32 = 1;
pub const DT_PLTRELSZ: i32 = 2;
pub const DT_PLTGOT: i32 = 3;
pub const DT_HASH: i32 = 4;
pub const DT_STRTAB: i32 = 5;
pub const DT_SYMTAB: i32 = 6;
pub const DT_RELA: i32 = 7;
pub const DT_RELASZ: i32 = 8;
pub const DT_RELAENT: i32 = 9;
pub const DT_STRSZ: i32 = 10;
pub const DT_SYMENT: i32 = 11;
pub const DT_INIT: i32 = 12;
pub const DT_FINI: i32 = 13;
pub const DT_SONAME: i32 = 14;
pub const DT_RPATH: i32 = 15;
pub const DT_SYMBOLIC: i32 = 16;
pub const DT_REL: i32 = 17;
pub const DT_RELSZ: i32 = 18;
pub const DT_RELENT: i32 = 19;
pub const DT_PLTREL: i32 = 20;
pub const DT_DEBUG: i32 = 21;
pub const DT_TEXTREL: i32 = 22;
pub const DT_JMPREL: i32 = 23;
pub const DT_BIND_NOW: i32 = 24;
pub const DT_INIT_ARRAY: i32 = 25;
pub const DT_FINI_ARRAY: i32 = 26;
pub const DT_INIT_ARRAYSZ: i32 = 27;
pub const DT_FINI_ARRAYSZ: i32 = 28;
pub const DT_RUNPATH: i32 = 29;
pub const DT_FLAGS: i32 = 30;
pub const DT_ENCODING: i32 = 32;
pub const DT_PREINIT_ARRAY: i32 = 32;
pub const DT_PREINIT_ARRAYSZ: i32 = 33;
pub const DT_NUM: i32 = 34;
pub const DT_LOOS: i32 = 1610612749;
pub const DT_HIOS: i32 = 1879044096;
pub const DT_LOPROC: i32 = 1879048192;
pub const DT_HIPROC: i32 = 2147483647;
pub const DT_VALRNGLO: i32 = 1879047424;
pub const DT_GNU_PRELINKED: i32 = 1879047669;
pub const DT_GNU_CONFLICTSZ: i32 = 1879047670;
pub const DT_GNU_LIBLISTSZ: i32 = 1879047671;
pub const DT_CHECKSUM: i32 = 1879047672;
pub const DT_PLTPADSZ: i32 = 1879047673;
pub const DT_MOVEENT: i32 = 1879047674;
pub const DT_MOVESZ: i32 = 1879047675;
pub const DT_FEATURE_1: i32 = 1879047676;
pub const DT_POSFLAG_1: i32 = 1879047677;
pub const DT_SYMINSZ: i32 = 1879047678;
pub const DT_SYMINENT: i32 = 1879047679;
pub const DT_VALRNGHI: i32 = 1879047679;
pub const DT_VALNUM: i32 = 12;
pub const DT_ADDRRNGLO: i32 = 1879047680;
pub const DT_GNU_HASH: i32 = 1879047925;
pub const DT_TLSDESC_PLT: i32 = 1879047926;
pub const DT_TLSDESC_GOT: i32 = 1879047927;
pub const DT_GNU_CONFLICT: i32 = 1879047928;
pub const DT_GNU_LIBLIST: i32 = 1879047929;
pub const DT_CONFIG: i32 = 1879047930;
pub const DT_DEPAUDIT: i32 = 1879047931;
pub const DT_AUDIT: i32 = 1879047932;
pub const DT_PLTPAD: i32 = 1879047933;
pub const DT_MOVETAB: i32 = 1879047934;
pub const DT_SYMINFO: i32 = 1879047935;
pub const DT_ADDRRNGHI: i32 = 1879047935;
pub const DT_ADDRNUM: i32 = 11;
pub const DT_VERSYM: i32 = 1879048176;
pub const DT_RELACOUNT: i32 = 1879048185;
pub const DT_RELCOUNT: i32 = 1879048186;
pub const DT_FLAGS_1: i32 = 1879048187;
pub const DT_VERDEF: i32 = 1879048188;
pub const DT_VERDEFNUM: i32 = 1879048189;
pub const DT_VERNEED: i32 = 1879048190;
pub const DT_VERNEEDNUM: i32 = 1879048191;
pub const DT_VERSIONTAGNUM: i32 = 16;
pub const DT_AUXILIARY: i32 = 2147483645;
pub const DT_FILTER: i32 = 2147483647;
pub const DT_EXTRANUM: i32 = 3;
pub const DF_ORIGIN: i32 = 1;
pub const DF_SYMBOLIC: i32 = 2;
pub const DF_TEXTREL: i32 = 4;
pub const DF_BIND_NOW: i32 = 8;
pub const DF_STATIC_TLS: i32 = 16;
pub const DF_1_NOW: i32 = 1;
pub const DF_1_GLOBAL: i32 = 2;
pub const DF_1_GROUP: i32 = 4;
pub const DF_1_NODELETE: i32 = 8;
pub const DF_1_LOADFLTR: i32 = 16;
pub const DF_1_INITFIRST: i32 = 32;
pub const DF_1_NOOPEN: i32 = 64;
pub const DF_1_ORIGIN: i32 = 128;
pub const DF_1_DIRECT: i32 = 256;
pub const DF_1_TRANS: i32 = 512;
pub const DF_1_INTERPOSE: i32 = 1024;
pub const DF_1_NODEFLIB: i32 = 2048;
pub const DF_1_NODUMP: i32 = 4096;
pub const DF_1_CONFALT: i32 = 8192;
pub const DF_1_ENDFILTEE: i32 = 16384;
pub const DF_1_DISPRELDNE: i32 = 32768;
pub const DF_1_DISPRELPND: i32 = 65536;
pub const DF_1_NODIRECT: i32 = 131072;
pub const DF_1_IGNMULDEF: i32 = 262144;
pub const DF_1_NOKSYMS: i32 = 524288;
pub const DF_1_NOHDR: i32 = 1048576;
pub const DF_1_EDITED: i32 = 2097152;
pub const DF_1_NORELOC: i32 = 4194304;
pub const DF_1_SYMINTPOSE: i32 = 8388608;
pub const DF_1_GLOBAUDIT: i32 = 16777216;
pub const DF_1_SINGLETON: i32 = 33554432;
pub const DTF_1_PARINIT: i32 = 1;
pub const DTF_1_CONFEXP: i32 = 2;
pub const DF_P1_LAZYLOAD: i32 = 1;
pub const DF_P1_GROUPPERM: i32 = 2;
pub const VER_DEF_NONE: i32 = 0;
pub const VER_DEF_CURRENT: i32 = 1;
pub const VER_DEF_NUM: i32 = 2;
pub const VER_FLG_BASE: i32 = 1;
pub const VER_FLG_WEAK: i32 = 2;
pub const VER_NDX_LOCAL: i32 = 0;
pub const VER_NDX_GLOBAL: i32 = 1;
pub const VER_NDX_LORESERVE: i32 = 65280;
pub const VER_NDX_ELIMINATE: i32 = 65281;
pub const VER_NEED_NONE: i32 = 0;
pub const VER_NEED_CURRENT: i32 = 1;
pub const VER_NEED_NUM: i32 = 2;
pub const AT_NULL: i32 = 0;
pub const AT_IGNORE: i32 = 1;
pub const AT_EXECFD: i32 = 2;
pub const AT_PHDR: i32 = 3;
pub const AT_PHENT: i32 = 4;
pub const AT_PHNUM: i32 = 5;
pub const AT_PAGESZ: i32 = 6;
pub const AT_BASE: i32 = 7;
pub const AT_FLAGS: i32 = 8;
pub const AT_ENTRY: i32 = 9;
pub const AT_NOTELF: i32 = 10;
pub const AT_UID: i32 = 11;
pub const AT_EUID: i32 = 12;
pub const AT_GID: i32 = 13;
pub const AT_EGID: i32 = 14;
pub const AT_CLKTCK: i32 = 17;
pub const AT_PLATFORM: i32 = 15;
pub const AT_HWCAP: i32 = 16;
pub const AT_FPUCW: i32 = 18;
pub const AT_DCACHEBSIZE: i32 = 19;
pub const AT_ICACHEBSIZE: i32 = 20;
pub const AT_UCACHEBSIZE: i32 = 21;
pub const AT_IGNOREPPC: i32 = 22;
pub const AT_SECURE: i32 = 23;
pub const AT_BASE_PLATFORM: i32 = 24;
pub const AT_RANDOM: i32 = 25;
pub const AT_HWCAP2: i32 = 26;
pub const AT_EXECFN: i32 = 31;
pub const AT_SYSINFO: i32 = 32;
pub const AT_SYSINFO_EHDR: i32 = 33;
pub const AT_L1I_CACHESHAPE: i32 = 34;
pub const AT_L1D_CACHESHAPE: i32 = 35;
pub const AT_L2_CACHESHAPE: i32 = 36;
pub const AT_L3_CACHESHAPE: i32 = 37;
pub const ELF_NOTE_SOLARIS: &'static [u8; 13usize] = b"SUNW Solaris\0";
pub const ELF_NOTE_GNU: &'static [u8; 4usize] = b"GNU\0";
pub const ELF_NOTE_PAGESIZE_HINT: i32 = 1;
pub const NT_GNU_ABI_TAG: i32 = 1;
pub const ELF_NOTE_ABI: i32 = 1;
pub const ELF_NOTE_OS_LINUX: i32 = 0;
pub const ELF_NOTE_OS_GNU: i32 = 1;
pub const ELF_NOTE_OS_SOLARIS2: i32 = 2;
pub const ELF_NOTE_OS_FREEBSD: i32 = 3;
pub const NT_GNU_BUILD_ID: i32 = 3;
pub const NT_GNU_GOLD_VERSION: i32 = 4;
pub const EF_CPU32: i32 = 8454144;
pub const R_68K_NONE: i32 = 0;
pub const R_68K_32: i32 = 1;
pub const R_68K_16: i32 = 2;
pub const R_68K_8: i32 = 3;
pub const R_68K_PC32: i32 = 4;
pub const R_68K_PC16: i32 = 5;
pub const R_68K_PC8: i32 = 6;
pub const R_68K_GOT32: i32 = 7;
pub const R_68K_GOT16: i32 = 8;
pub const R_68K_GOT8: i32 = 9;
pub const R_68K_GOT32O: i32 = 10;
pub const R_68K_GOT16O: i32 = 11;
pub const R_68K_GOT8O: i32 = 12;
pub const R_68K_PLT32: i32 = 13;
pub const R_68K_PLT16: i32 = 14;
pub const R_68K_PLT8: i32 = 15;
pub const R_68K_PLT32O: i32 = 16;
pub const R_68K_PLT16O: i32 = 17;
pub const R_68K_PLT8O: i32 = 18;
pub const R_68K_COPY: i32 = 19;
pub const R_68K_GLOB_DAT: i32 = 20;
pub const R_68K_JMP_SLOT: i32 = 21;
pub const R_68K_RELATIVE: i32 = 22;
pub const R_68K_NUM: i32 = 23;
pub const R_386_NONE: i32 = 0;
pub const R_386_32: i32 = 1;
pub const R_386_PC32: i32 = 2;
pub const R_386_GOT32: i32 = 3;
pub const R_386_PLT32: i32 = 4;
pub const R_386_COPY: i32 = 5;
pub const R_386_GLOB_DAT: i32 = 6;
pub const R_386_JMP_SLOT: i32 = 7;
pub const R_386_RELATIVE: i32 = 8;
pub const R_386_GOTOFF: i32 = 9;
pub const R_386_GOTPC: i32 = 10;
pub const R_386_32PLT: i32 = 11;
pub const R_386_TLS_TPOFF: i32 = 14;
pub const R_386_TLS_IE: i32 = 15;
pub const R_386_TLS_GOTIE: i32 = 16;
pub const R_386_TLS_LE: i32 = 17;
pub const R_386_TLS_GD: i32 = 18;
pub const R_386_TLS_LDM: i32 = 19;
pub const R_386_16: i32 = 20;
pub const R_386_PC16: i32 = 21;
pub const R_386_8: i32 = 22;
pub const R_386_PC8: i32 = 23;
pub const R_386_TLS_GD_32: i32 = 24;
pub const R_386_TLS_GD_PUSH: i32 = 25;
pub const R_386_TLS_GD_CALL: i32 = 26;
pub const R_386_TLS_GD_POP: i32 = 27;
pub const R_386_TLS_LDM_32: i32 = 28;
pub const R_386_TLS_LDM_PUSH: i32 = 29;
pub const R_386_TLS_LDM_CALL: i32 = 30;
pub const R_386_TLS_LDM_POP: i32 = 31;
pub const R_386_TLS_LDO_32: i32 = 32;
pub const R_386_TLS_IE_32: i32 = 33;
pub const R_386_TLS_LE_32: i32 = 34;
pub const R_386_TLS_DTPMOD32: i32 = 35;
pub const R_386_TLS_DTPOFF32: i32 = 36;
pub const R_386_TLS_TPOFF32: i32 = 37;
pub const R_386_SIZE32: i32 = 38;
pub const R_386_TLS_GOTDESC: i32 = 39;
pub const R_386_TLS_DESC_CALL: i32 = 40;
pub const R_386_TLS_DESC: i32 = 41;
pub const R_386_IRELATIVE: i32 = 42;
pub const R_386_GOT32X: i32 = 43;
pub const R_386_NUM: i32 = 44;
pub const STT_SPARC_REGISTER: i32 = 13;
pub const EF_SPARCV9_MM: i32 = 3;
pub const EF_SPARCV9_TSO: i32 = 0;
pub const EF_SPARCV9_PSO: i32 = 1;
pub const EF_SPARCV9_RMO: i32 = 2;
pub const EF_SPARC_LEDATA: i32 = 8388608;
pub const EF_SPARC_EXT_MASK: i32 = 16776960;
pub const EF_SPARC_32PLUS: i32 = 256;
pub const EF_SPARC_SUN_US1: i32 = 512;
pub const EF_SPARC_HAL_R1: i32 = 1024;
pub const EF_SPARC_SUN_US3: i32 = 2048;
pub const R_SPARC_NONE: i32 = 0;
pub const R_SPARC_8: i32 = 1;
pub const R_SPARC_16: i32 = 2;
pub const R_SPARC_32: i32 = 3;
pub const R_SPARC_DISP8: i32 = 4;
pub const R_SPARC_DISP16: i32 = 5;
pub const R_SPARC_DISP32: i32 = 6;
pub const R_SPARC_WDISP30: i32 = 7;
pub const R_SPARC_WDISP22: i32 = 8;
pub const R_SPARC_HI22: i32 = 9;
pub const R_SPARC_22: i32 = 10;
pub const R_SPARC_13: i32 = 11;
pub const R_SPARC_LO10: i32 = 12;
pub const R_SPARC_GOT10: i32 = 13;
pub const R_SPARC_GOT13: i32 = 14;
pub const R_SPARC_GOT22: i32 = 15;
pub const R_SPARC_PC10: i32 = 16;
pub const R_SPARC_PC22: i32 = 17;
pub const R_SPARC_WPLT30: i32 = 18;
pub const R_SPARC_COPY: i32 = 19;
pub const R_SPARC_GLOB_DAT: i32 = 20;
pub const R_SPARC_JMP_SLOT: i32 = 21;
pub const R_SPARC_RELATIVE: i32 = 22;
pub const R_SPARC_UA32: i32 = 23;
pub const R_SPARC_PLT32: i32 = 24;
pub const R_SPARC_HIPLT22: i32 = 25;
pub const R_SPARC_LOPLT10: i32 = 26;
pub const R_SPARC_PCPLT32: i32 = 27;
pub const R_SPARC_PCPLT22: i32 = 28;
pub const R_SPARC_PCPLT10: i32 = 29;
pub const R_SPARC_10: i32 = 30;
pub const R_SPARC_11: i32 = 31;
pub const R_SPARC_64: i32 = 32;
pub const R_SPARC_OLO10: i32 = 33;
pub const R_SPARC_HH22: i32 = 34;
pub const R_SPARC_HM10: i32 = 35;
pub const R_SPARC_LM22: i32 = 36;
pub const R_SPARC_PC_HH22: i32 = 37;
pub const R_SPARC_PC_HM10: i32 = 38;
pub const R_SPARC_PC_LM22: i32 = 39;
pub const R_SPARC_WDISP16: i32 = 40;
pub const R_SPARC_WDISP19: i32 = 41;
pub const R_SPARC_GLOB_JMP: i32 = 42;
pub const R_SPARC_7: i32 = 43;
pub const R_SPARC_5: i32 = 44;
pub const R_SPARC_6: i32 = 45;
pub const R_SPARC_DISP64: i32 = 46;
pub const R_SPARC_PLT64: i32 = 47;
pub const R_SPARC_HIX22: i32 = 48;
pub const R_SPARC_LOX10: i32 = 49;
pub const R_SPARC_H44: i32 = 50;
pub const R_SPARC_M44: i32 = 51;
pub const R_SPARC_L44: i32 = 52;
pub const R_SPARC_REGISTER: i32 = 53;
pub const R_SPARC_UA64: i32 = 54;
pub const R_SPARC_UA16: i32 = 55;
pub const R_SPARC_TLS_GD_HI22: i32 = 56;
pub const R_SPARC_TLS_GD_LO10: i32 = 57;
pub const R_SPARC_TLS_GD_ADD: i32 = 58;
pub const R_SPARC_TLS_GD_CALL: i32 = 59;
pub const R_SPARC_TLS_LDM_HI22: i32 = 60;
pub const R_SPARC_TLS_LDM_LO10: i32 = 61;
pub const R_SPARC_TLS_LDM_ADD: i32 = 62;
pub const R_SPARC_TLS_LDM_CALL: i32 = 63;
pub const R_SPARC_TLS_LDO_HIX22: i32 = 64;
pub const R_SPARC_TLS_LDO_LOX10: i32 = 65;
pub const R_SPARC_TLS_LDO_ADD: i32 = 66;
pub const R_SPARC_TLS_IE_HI22: i32 = 67;
pub const R_SPARC_TLS_IE_LO10: i32 = 68;
pub const R_SPARC_TLS_IE_LD: i32 = 69;
pub const R_SPARC_TLS_IE_LDX: i32 = 70;
pub const R_SPARC_TLS_IE_ADD: i32 = 71;
pub const R_SPARC_TLS_LE_HIX22: i32 = 72;
pub const R_SPARC_TLS_LE_LOX10: i32 = 73;
pub const R_SPARC_TLS_DTPMOD32: i32 = 74;
pub const R_SPARC_TLS_DTPMOD64: i32 = 75;
pub const R_SPARC_TLS_DTPOFF32: i32 = 76;
pub const R_SPARC_TLS_DTPOFF64: i32 = 77;
pub const R_SPARC_TLS_TPOFF32: i32 = 78;
pub const R_SPARC_TLS_TPOFF64: i32 = 79;
pub const R_SPARC_GOTDATA_HIX22: i32 = 80;
pub const R_SPARC_GOTDATA_LOX10: i32 = 81;
pub const R_SPARC_GOTDATA_OP_HIX22: i32 = 82;
pub const R_SPARC_GOTDATA_OP_LOX10: i32 = 83;
pub const R_SPARC_GOTDATA_OP: i32 = 84;
pub const R_SPARC_H34: i32 = 85;
pub const R_SPARC_SIZE32: i32 = 86;
pub const R_SPARC_SIZE64: i32 = 87;
pub const R_SPARC_GNU_VTINHERIT: i32 = 250;
pub const R_SPARC_GNU_VTENTRY: i32 = 251;
pub const R_SPARC_REV32: i32 = 252;
pub const R_SPARC_NUM: i32 = 253;
pub const DT_SPARC_REGISTER: i32 = 1879048193;
pub const DT_SPARC_NUM: i32 = 2;
pub const EF_MIPS_NOREORDER: i32 = 1;
pub const EF_MIPS_PIC: i32 = 2;
pub const EF_MIPS_CPIC: i32 = 4;
pub const EF_MIPS_XGOT: i32 = 8;
pub const EF_MIPS_64BIT_WHIRL: i32 = 16;
pub const EF_MIPS_ABI2: i32 = 32;
pub const EF_MIPS_ABI_ON32: i32 = 64;
pub const EF_MIPS_FP64: i32 = 512;
pub const EF_MIPS_NAN2008: i32 = 1024;
pub const EF_MIPS_ARCH: i32 = 4026531840;
pub const EF_MIPS_ARCH_1: i32 = 0;
pub const EF_MIPS_ARCH_2: i32 = 268435456;
pub const EF_MIPS_ARCH_3: i32 = 536870912;
pub const EF_MIPS_ARCH_4: i32 = 805306368;
pub const EF_MIPS_ARCH_5: i32 = 1073741824;
pub const EF_MIPS_ARCH_32: i32 = 1342177280;
pub const EF_MIPS_ARCH_64: i32 = 1610612736;
pub const EF_MIPS_ARCH_32R2: i32 = 1879048192;
pub const EF_MIPS_ARCH_64R2: i32 = 2147483648;
pub const E_MIPS_ARCH_1: i32 = 0;
pub const E_MIPS_ARCH_2: i32 = 268435456;
pub const E_MIPS_ARCH_3: i32 = 536870912;
pub const E_MIPS_ARCH_4: i32 = 805306368;
pub const E_MIPS_ARCH_5: i32 = 1073741824;
pub const E_MIPS_ARCH_32: i32 = 1342177280;
pub const E_MIPS_ARCH_64: i32 = 1610612736;
pub const SHN_MIPS_ACOMMON: i32 = 65280;
pub const SHN_MIPS_TEXT: i32 = 65281;
pub const SHN_MIPS_DATA: i32 = 65282;
pub const SHN_MIPS_SCOMMON: i32 = 65283;
pub const SHN_MIPS_SUNDEFINED: i32 = 65284;
pub const SHT_MIPS_LIBLIST: i32 = 1879048192;
pub const SHT_MIPS_MSYM: i32 = 1879048193;
pub const SHT_MIPS_CONFLICT: i32 = 1879048194;
pub const SHT_MIPS_GPTAB: i32 = 1879048195;
pub const SHT_MIPS_UCODE: i32 = 1879048196;
pub const SHT_MIPS_DEBUG: i32 = 1879048197;
pub const SHT_MIPS_REGINFO: i32 = 1879048198;
pub const SHT_MIPS_PACKAGE: i32 = 1879048199;
pub const SHT_MIPS_PACKSYM: i32 = 1879048200;
pub const SHT_MIPS_RELD: i32 = 1879048201;
pub const SHT_MIPS_IFACE: i32 = 1879048203;
pub const SHT_MIPS_CONTENT: i32 = 1879048204;
pub const SHT_MIPS_OPTIONS: i32 = 1879048205;
pub const SHT_MIPS_SHDR: i32 = 1879048208;
pub const SHT_MIPS_FDESC: i32 = 1879048209;
pub const SHT_MIPS_EXTSYM: i32 = 1879048210;
pub const SHT_MIPS_DENSE: i32 = 1879048211;
pub const SHT_MIPS_PDESC: i32 = 1879048212;
pub const SHT_MIPS_LOCSYM: i32 = 1879048213;
pub const SHT_MIPS_AUXSYM: i32 = 1879048214;
pub const SHT_MIPS_OPTSYM: i32 = 1879048215;
pub const SHT_MIPS_LOCSTR: i32 = 1879048216;
pub const SHT_MIPS_LINE: i32 = 1879048217;
pub const SHT_MIPS_RFDESC: i32 = 1879048218;
pub const SHT_MIPS_DELTASYM: i32 = 1879048219;
pub const SHT_MIPS_DELTAINST: i32 = 1879048220;
pub const SHT_MIPS_DELTACLASS: i32 = 1879048221;
pub const SHT_MIPS_DWARF: i32 = 1879048222;
pub const SHT_MIPS_DELTADECL: i32 = 1879048223;
pub const SHT_MIPS_SYMBOL_LIB: i32 = 1879048224;
pub const SHT_MIPS_EVENTS: i32 = 1879048225;
pub const SHT_MIPS_TRANSLATE: i32 = 1879048226;
pub const SHT_MIPS_PIXIE: i32 = 1879048227;
pub const SHT_MIPS_XLATE: i32 = 1879048228;
pub const SHT_MIPS_XLATE_DEBUG: i32 = 1879048229;
pub const SHT_MIPS_WHIRL: i32 = 1879048230;
pub const SHT_MIPS_EH_REGION: i32 = 1879048231;
pub const SHT_MIPS_XLATE_OLD: i32 = 1879048232;
pub const SHT_MIPS_PDR_EXCEPTION: i32 = 1879048233;
pub const SHF_MIPS_GPREL: i32 = 268435456;
pub const SHF_MIPS_MERGE: i32 = 536870912;
pub const SHF_MIPS_ADDR: i32 = 1073741824;
pub const SHF_MIPS_STRINGS: i32 = 2147483648;
pub const SHF_MIPS_NOSTRIP: i32 = 134217728;
pub const SHF_MIPS_LOCAL: i32 = 67108864;
pub const SHF_MIPS_NAMES: i32 = 33554432;
pub const SHF_MIPS_NODUPE: i32 = 16777216;
pub const STO_MIPS_DEFAULT: i32 = 0;
pub const STO_MIPS_INTERNAL: i32 = 1;
pub const STO_MIPS_HIDDEN: i32 = 2;
pub const STO_MIPS_PROTECTED: i32 = 3;
pub const STO_MIPS_PLT: i32 = 8;
pub const STO_MIPS_SC_ALIGN_UNUSED: i32 = 255;
pub const STB_MIPS_SPLIT_COMMON: i32 = 13;
pub const ODK_NULL: i32 = 0;
pub const ODK_REGINFO: i32 = 1;
pub const ODK_EXCEPTIONS: i32 = 2;
pub const ODK_PAD: i32 = 3;
pub const ODK_HWPATCH: i32 = 4;
pub const ODK_FILL: i32 = 5;
pub const ODK_TAGS: i32 = 6;
pub const ODK_HWAND: i32 = 7;
pub const ODK_HWOR: i32 = 8;
pub const OEX_FPU_MIN: i32 = 31;
pub const OEX_FPU_MAX: i32 = 7936;
pub const OEX_PAGE0: i32 = 65536;
pub const OEX_SMM: i32 = 131072;
pub const OEX_FPDBUG: i32 = 262144;
pub const OEX_PRECISEFP: i32 = 262144;
pub const OEX_DISMISS: i32 = 524288;
pub const OEX_FPU_INVAL: i32 = 16;
pub const OEX_FPU_DIV0: i32 = 8;
pub const OEX_FPU_OFLO: i32 = 4;
pub const OEX_FPU_UFLO: i32 = 2;
pub const OEX_FPU_INEX: i32 = 1;
pub const OHW_R4KEOP: i32 = 1;
pub const OHW_R8KPFETCH: i32 = 2;
pub const OHW_R5KEOP: i32 = 4;
pub const OHW_R5KCVTL: i32 = 8;
pub const OPAD_PREFIX: i32 = 1;
pub const OPAD_POSTFIX: i32 = 2;
pub const OPAD_SYMBOL: i32 = 4;
pub const OHWA0_R4KEOP_CHECKED: i32 = 1;
pub const OHWA1_R4KEOP_CLEAN: i32 = 2;
pub const R_MIPS_NONE: i32 = 0;
pub const R_MIPS_16: i32 = 1;
pub const R_MIPS_32: i32 = 2;
pub const R_MIPS_REL32: i32 = 3;
pub const R_MIPS_26: i32 = 4;
pub const R_MIPS_HI16: i32 = 5;
pub const R_MIPS_LO16: i32 = 6;
pub const R_MIPS_GPREL16: i32 = 7;
pub const R_MIPS_LITERAL: i32 = 8;
pub const R_MIPS_GOT16: i32 = 9;
pub const R_MIPS_PC16: i32 = 10;
pub const R_MIPS_CALL16: i32 = 11;
pub const R_MIPS_GPREL32: i32 = 12;
pub const R_MIPS_SHIFT5: i32 = 16;
pub const R_MIPS_SHIFT6: i32 = 17;
pub const R_MIPS_64: i32 = 18;
pub const R_MIPS_GOT_DISP: i32 = 19;
pub const R_MIPS_GOT_PAGE: i32 = 20;
pub const R_MIPS_GOT_OFST: i32 = 21;
pub const R_MIPS_GOT_HI16: i32 = 22;
pub const R_MIPS_GOT_LO16: i32 = 23;
pub const R_MIPS_SUB: i32 = 24;
pub const R_MIPS_INSERT_A: i32 = 25;
pub const R_MIPS_INSERT_B: i32 = 26;
pub const R_MIPS_DELETE: i32 = 27;
pub const R_MIPS_HIGHER: i32 = 28;
pub const R_MIPS_HIGHEST: i32 = 29;
pub const R_MIPS_CALL_HI16: i32 = 30;
pub const R_MIPS_CALL_LO16: i32 = 31;
pub const R_MIPS_SCN_DISP: i32 = 32;
pub const R_MIPS_REL16: i32 = 33;
pub const R_MIPS_ADD_IMMEDIATE: i32 = 34;
pub const R_MIPS_PJUMP: i32 = 35;
pub const R_MIPS_RELGOT: i32 = 36;
pub const R_MIPS_JALR: i32 = 37;
pub const R_MIPS_TLS_DTPMOD32: i32 = 38;
pub const R_MIPS_TLS_DTPREL32: i32 = 39;
pub const R_MIPS_TLS_DTPMOD64: i32 = 40;
pub const R_MIPS_TLS_DTPREL64: i32 = 41;
pub const R_MIPS_TLS_GD: i32 = 42;
pub const R_MIPS_TLS_LDM: i32 = 43;
pub const R_MIPS_TLS_DTPREL_HI16: i32 = 44;
pub const R_MIPS_TLS_DTPREL_LO16: i32 = 45;
pub const R_MIPS_TLS_GOTTPREL: i32 = 46;
pub const R_MIPS_TLS_TPREL32: i32 = 47;
pub const R_MIPS_TLS_TPREL64: i32 = 48;
pub const R_MIPS_TLS_TPREL_HI16: i32 = 49;
pub const R_MIPS_TLS_TPREL_LO16: i32 = 50;
pub const R_MIPS_GLOB_DAT: i32 = 51;
pub const R_MIPS_COPY: i32 = 126;
pub const R_MIPS_JUMP_SLOT: i32 = 127;
pub const R_MIPS_NUM: i32 = 128;
pub const PT_MIPS_REGINFO: i32 = 1879048192;
pub const PT_MIPS_RTPROC: i32 = 1879048193;
pub const PT_MIPS_OPTIONS: i32 = 1879048194;
pub const PT_MIPS_ABIFLAGS: i32 = 1879048195;
pub const PF_MIPS_LOCAL: i32 = 268435456;
pub const DT_MIPS_RLD_VERSION: i32 = 1879048193;
pub const DT_MIPS_TIME_STAMP: i32 = 1879048194;
pub const DT_MIPS_ICHECKSUM: i32 = 1879048195;
pub const DT_MIPS_IVERSION: i32 = 1879048196;
pub const DT_MIPS_FLAGS: i32 = 1879048197;
pub const DT_MIPS_BASE_ADDRESS: i32 = 1879048198;
pub const DT_MIPS_MSYM: i32 = 1879048199;
pub const DT_MIPS_CONFLICT: i32 = 1879048200;
pub const DT_MIPS_LIBLIST: i32 = 1879048201;
pub const DT_MIPS_LOCAL_GOTNO: i32 = 1879048202;
pub const DT_MIPS_CONFLICTNO: i32 = 1879048203;
pub const DT_MIPS_LIBLISTNO: i32 = 1879048208;
pub const DT_MIPS_SYMTABNO: i32 = 1879048209;
pub const DT_MIPS_UNREFEXTNO: i32 = 1879048210;
pub const DT_MIPS_GOTSYM: i32 = 1879048211;
pub const DT_MIPS_HIPAGENO: i32 = 1879048212;
pub const DT_MIPS_RLD_MAP: i32 = 1879048214;
pub const DT_MIPS_DELTA_CLASS: i32 = 1879048215;
pub const DT_MIPS_DELTA_CLASS_NO: i32 = 1879048216;
pub const DT_MIPS_DELTA_INSTANCE: i32 = 1879048217;
pub const DT_MIPS_DELTA_INSTANCE_NO: i32 = 1879048218;
pub const DT_MIPS_DELTA_RELOC: i32 = 1879048219;
pub const DT_MIPS_DELTA_RELOC_NO: i32 = 1879048220;
pub const DT_MIPS_DELTA_SYM: i32 = 1879048221;
pub const DT_MIPS_DELTA_SYM_NO: i32 = 1879048222;
pub const DT_MIPS_DELTA_CLASSSYM: i32 = 1879048224;
pub const DT_MIPS_DELTA_CLASSSYM_NO: i32 = 1879048225;
pub const DT_MIPS_CXX_FLAGS: i32 = 1879048226;
pub const DT_MIPS_PIXIE_INIT: i32 = 1879048227;
pub const DT_MIPS_SYMBOL_LIB: i32 = 1879048228;
pub const DT_MIPS_LOCALPAGE_GOTIDX: i32 = 1879048229;
pub const DT_MIPS_LOCAL_GOTIDX: i32 = 1879048230;
pub const DT_MIPS_HIDDEN_GOTIDX: i32 = 1879048231;
pub const DT_MIPS_PROTECTED_GOTIDX: i32 = 1879048232;
pub const DT_MIPS_OPTIONS: i32 = 1879048233;
pub const DT_MIPS_INTERFACE: i32 = 1879048234;
pub const DT_MIPS_DYNSTR_ALIGN: i32 = 1879048235;
pub const DT_MIPS_INTERFACE_SIZE: i32 = 1879048236;
pub const DT_MIPS_RLD_TEXT_RESOLVE_ADDR: i32 = 1879048237;
pub const DT_MIPS_PERF_SUFFIX: i32 = 1879048238;
pub const DT_MIPS_COMPACT_SIZE: i32 = 1879048239;
pub const DT_MIPS_GP_VALUE: i32 = 1879048240;
pub const DT_MIPS_AUX_DYNAMIC: i32 = 1879048241;
pub const DT_MIPS_PLTGOT: i32 = 1879048242;
pub const DT_MIPS_RWPLT: i32 = 1879048244;
pub const DT_MIPS_RLD_MAP_REL: i32 = 1879048245;
pub const DT_MIPS_NUM: i32 = 54;
pub const RHF_NONE: i32 = 0;
pub const RHF_QUICKSTART: i32 = 1;
pub const RHF_NOTPOT: i32 = 2;
pub const RHF_NO_LIBRARY_REPLACEMENT: i32 = 4;
pub const RHF_NO_MOVE: i32 = 8;
pub const RHF_SGI_ONLY: i32 = 16;
pub const RHF_GUARANTEE_INIT: i32 = 32;
pub const RHF_DELTA_C_PLUS_PLUS: i32 = 64;
pub const RHF_GUARANTEE_START_INIT: i32 = 128;
pub const RHF_PIXIE: i32 = 256;
pub const RHF_DEFAULT_DELAY_LOAD: i32 = 512;
pub const RHF_REQUICKSTART: i32 = 1024;
pub const RHF_REQUICKSTARTED: i32 = 2048;
pub const RHF_CORD: i32 = 4096;
pub const RHF_NO_UNRES_UNDEF: i32 = 8192;
pub const RHF_RLD_ORDER_SAFE: i32 = 16384;
pub const LL_NONE: i32 = 0;
pub const LL_EXACT_MATCH: i32 = 1;
pub const LL_IGNORE_INT_VER: i32 = 2;
pub const LL_REQUIRE_MINOR: i32 = 4;
pub const LL_EXPORTS: i32 = 8;
pub const LL_DELAY_LOAD: i32 = 16;
pub const LL_DELTA: i32 = 32;
pub const MIPS_AFL_REG_NONE: i32 = 0;
pub const MIPS_AFL_REG_32: i32 = 1;
pub const MIPS_AFL_REG_64: i32 = 2;
pub const MIPS_AFL_REG_128: i32 = 3;
pub const MIPS_AFL_ASE_DSP: i32 = 1;
pub const MIPS_AFL_ASE_DSPR2: i32 = 2;
pub const MIPS_AFL_ASE_EVA: i32 = 4;
pub const MIPS_AFL_ASE_MCU: i32 = 8;
pub const MIPS_AFL_ASE_MDMX: i32 = 16;
pub const MIPS_AFL_ASE_MIPS3D: i32 = 32;
pub const MIPS_AFL_ASE_MT: i32 = 64;
pub const MIPS_AFL_ASE_SMARTMIPS: i32 = 128;
pub const MIPS_AFL_ASE_VIRT: i32 = 256;
pub const MIPS_AFL_ASE_MSA: i32 = 512;
pub const MIPS_AFL_ASE_MIPS16: i32 = 1024;
pub const MIPS_AFL_ASE_MICROMIPS: i32 = 2048;
pub const MIPS_AFL_ASE_XPA: i32 = 4096;
pub const MIPS_AFL_ASE_MASK: i32 = 8191;
pub const MIPS_AFL_EXT_XLR: i32 = 1;
pub const MIPS_AFL_EXT_OCTEON2: i32 = 2;
pub const MIPS_AFL_EXT_OCTEONP: i32 = 3;
pub const MIPS_AFL_EXT_LOONGSON_3A: i32 = 4;
pub const MIPS_AFL_EXT_OCTEON: i32 = 5;
pub const MIPS_AFL_EXT_5900: i32 = 6;
pub const MIPS_AFL_EXT_4650: i32 = 7;
pub const MIPS_AFL_EXT_4010: i32 = 8;
pub const MIPS_AFL_EXT_4100: i32 = 9;
pub const MIPS_AFL_EXT_3900: i32 = 10;
pub const MIPS_AFL_EXT_10000: i32 = 11;
pub const MIPS_AFL_EXT_SB1: i32 = 12;
pub const MIPS_AFL_EXT_4111: i32 = 13;
pub const MIPS_AFL_EXT_4120: i32 = 14;
pub const MIPS_AFL_EXT_5400: i32 = 15;
pub const MIPS_AFL_EXT_5500: i32 = 16;
pub const MIPS_AFL_EXT_LOONGSON_2E: i32 = 17;
pub const MIPS_AFL_EXT_LOONGSON_2F: i32 = 18;
pub const MIPS_AFL_FLAGS1_ODDSPREG: i32 = 1;
pub const EF_PARISC_TRAPNIL: i32 = 65536;
pub const EF_PARISC_EXT: i32 = 131072;
pub const EF_PARISC_LSB: i32 = 262144;
pub const EF_PARISC_WIDE: i32 = 524288;
pub const EF_PARISC_NO_KABP: i32 = 1048576;
pub const EF_PARISC_LAZYSWAP: i32 = 4194304;
pub const EF_PARISC_ARCH: i32 = 65535;
pub const EFA_PARISC_1_0: i32 = 523;
pub const EFA_PARISC_1_1: i32 = 528;
pub const EFA_PARISC_2_0: i32 = 532;
pub const SHN_PARISC_ANSI_COMMON: i32 = 65280;
pub const SHN_PARISC_HUGE_COMMON: i32 = 65281;
pub const SHT_PARISC_EXT: i32 = 1879048192;
pub const SHT_PARISC_UNWIND: i32 = 1879048193;
pub const SHT_PARISC_DOC: i32 = 1879048194;
pub const SHF_PARISC_SHORT: i32 = 536870912;
pub const SHF_PARISC_HUGE: i32 = 1073741824;
pub const SHF_PARISC_SBP: i32 = 2147483648;
pub const STT_PARISC_MILLICODE: i32 = 13;
pub const STT_HP_OPAQUE: i32 = 11;
pub const STT_HP_STUB: i32 = 12;
pub const R_PARISC_NONE: i32 = 0;
pub const R_PARISC_DIR32: i32 = 1;
pub const R_PARISC_DIR21L: i32 = 2;
pub const R_PARISC_DIR17R: i32 = 3;
pub const R_PARISC_DIR17F: i32 = 4;
pub const R_PARISC_DIR14R: i32 = 6;
pub const R_PARISC_PCREL32: i32 = 9;
pub const R_PARISC_PCREL21L: i32 = 10;
pub const R_PARISC_PCREL17R: i32 = 11;
pub const R_PARISC_PCREL17F: i32 = 12;
pub const R_PARISC_PCREL14R: i32 = 14;
pub const R_PARISC_DPREL21L: i32 = 18;
pub const R_PARISC_DPREL14R: i32 = 22;
pub const R_PARISC_GPREL21L: i32 = 26;
pub const R_PARISC_GPREL14R: i32 = 30;
pub const R_PARISC_LTOFF21L: i32 = 34;
pub const R_PARISC_LTOFF14R: i32 = 38;
pub const R_PARISC_SECREL32: i32 = 41;
pub const R_PARISC_SEGBASE: i32 = 48;
pub const R_PARISC_SEGREL32: i32 = 49;
pub const R_PARISC_PLTOFF21L: i32 = 50;
pub const R_PARISC_PLTOFF14R: i32 = 54;
pub const R_PARISC_LTOFF_FPTR32: i32 = 57;
pub const R_PARISC_LTOFF_FPTR21L: i32 = 58;
pub const R_PARISC_LTOFF_FPTR14R: i32 = 62;
pub const R_PARISC_FPTR64: i32 = 64;
pub const R_PARISC_PLABEL32: i32 = 65;
pub const R_PARISC_PLABEL21L: i32 = 66;
pub const R_PARISC_PLABEL14R: i32 = 70;
pub const R_PARISC_PCREL64: i32 = 72;
pub const R_PARISC_PCREL22F: i32 = 74;
pub const R_PARISC_PCREL14WR: i32 = 75;
pub const R_PARISC_PCREL14DR: i32 = 76;
pub const R_PARISC_PCREL16F: i32 = 77;
pub const R_PARISC_PCREL16WF: i32 = 78;
pub const R_PARISC_PCREL16DF: i32 = 79;
pub const R_PARISC_DIR64: i32 = 80;
pub const R_PARISC_DIR14WR: i32 = 83;
pub const R_PARISC_DIR14DR: i32 = 84;
pub const R_PARISC_DIR16F: i32 = 85;
pub const R_PARISC_DIR16WF: i32 = 86;
pub const R_PARISC_DIR16DF: i32 = 87;
pub const R_PARISC_GPREL64: i32 = 88;
pub const R_PARISC_GPREL14WR: i32 = 91;
pub const R_PARISC_GPREL14DR: i32 = 92;
pub const R_PARISC_GPREL16F: i32 = 93;
pub const R_PARISC_GPREL16WF: i32 = 94;
pub const R_PARISC_GPREL16DF: i32 = 95;
pub const R_PARISC_LTOFF64: i32 = 96;
pub const R_PARISC_LTOFF14WR: i32 = 99;
pub const R_PARISC_LTOFF14DR: i32 = 100;
pub const R_PARISC_LTOFF16F: i32 = 101;
pub const R_PARISC_LTOFF16WF: i32 = 102;
pub const R_PARISC_LTOFF16DF: i32 = 103;
pub const R_PARISC_SECREL64: i32 = 104;
pub const R_PARISC_SEGREL64: i32 = 112;
pub const R_PARISC_PLTOFF14WR: i32 = 115;
pub const R_PARISC_PLTOFF14DR: i32 = 116;
pub const R_PARISC_PLTOFF16F: i32 = 117;
pub const R_PARISC_PLTOFF16WF: i32 = 118;
pub const R_PARISC_PLTOFF16DF: i32 = 119;
pub const R_PARISC_LTOFF_FPTR64: i32 = 120;
pub const R_PARISC_LTOFF_FPTR14WR: i32 = 123;
pub const R_PARISC_LTOFF_FPTR14DR: i32 = 124;
pub const R_PARISC_LTOFF_FPTR16F: i32 = 125;
pub const R_PARISC_LTOFF_FPTR16WF: i32 = 126;
pub const R_PARISC_LTOFF_FPTR16DF: i32 = 127;
pub const R_PARISC_LORESERVE: i32 = 128;
pub const R_PARISC_COPY: i32 = 128;
pub const R_PARISC_IPLT: i32 = 129;
pub const R_PARISC_EPLT: i32 = 130;
pub const R_PARISC_TPREL32: i32 = 153;
pub const R_PARISC_TPREL21L: i32 = 154;
pub const R_PARISC_TPREL14R: i32 = 158;
pub const R_PARISC_LTOFF_TP21L: i32 = 162;
pub const R_PARISC_LTOFF_TP14R: i32 = 166;
pub const R_PARISC_LTOFF_TP14F: i32 = 167;
pub const R_PARISC_TPREL64: i32 = 216;
pub const R_PARISC_TPREL14WR: i32 = 219;
pub const R_PARISC_TPREL14DR: i32 = 220;
pub const R_PARISC_TPREL16F: i32 = 221;
pub const R_PARISC_TPREL16WF: i32 = 222;
pub const R_PARISC_TPREL16DF: i32 = 223;
pub const R_PARISC_LTOFF_TP64: i32 = 224;
pub const R_PARISC_LTOFF_TP14WR: i32 = 227;
pub const R_PARISC_LTOFF_TP14DR: i32 = 228;
pub const R_PARISC_LTOFF_TP16F: i32 = 229;
pub const R_PARISC_LTOFF_TP16WF: i32 = 230;
pub const R_PARISC_LTOFF_TP16DF: i32 = 231;
pub const R_PARISC_GNU_VTENTRY: i32 = 232;
pub const R_PARISC_GNU_VTINHERIT: i32 = 233;
pub const R_PARISC_TLS_GD21L: i32 = 234;
pub const R_PARISC_TLS_GD14R: i32 = 235;
pub const R_PARISC_TLS_GDCALL: i32 = 236;
pub const R_PARISC_TLS_LDM21L: i32 = 237;
pub const R_PARISC_TLS_LDM14R: i32 = 238;
pub const R_PARISC_TLS_LDMCALL: i32 = 239;
pub const R_PARISC_TLS_LDO21L: i32 = 240;
pub const R_PARISC_TLS_LDO14R: i32 = 241;
pub const R_PARISC_TLS_DTPMOD32: i32 = 242;
pub const R_PARISC_TLS_DTPMOD64: i32 = 243;
pub const R_PARISC_TLS_DTPOFF32: i32 = 244;
pub const R_PARISC_TLS_DTPOFF64: i32 = 245;
pub const R_PARISC_TLS_LE21L: i32 = 154;
pub const R_PARISC_TLS_LE14R: i32 = 158;
pub const R_PARISC_TLS_IE21L: i32 = 162;
pub const R_PARISC_TLS_IE14R: i32 = 166;
pub const R_PARISC_TLS_TPREL32: i32 = 153;
pub const R_PARISC_TLS_TPREL64: i32 = 216;
pub const R_PARISC_HIRESERVE: i32 = 255;
pub const PT_HP_TLS: i32 = 1610612736;
pub const PT_HP_CORE_NONE: i32 = 1610612737;
pub const PT_HP_CORE_VERSION: i32 = 1610612738;
pub const PT_HP_CORE_KERNEL: i32 = 1610612739;
pub const PT_HP_CORE_COMM: i32 = 1610612740;
pub const PT_HP_CORE_PROC: i32 = 1610612741;
pub const PT_HP_CORE_LOADABLE: i32 = 1610612742;
pub const PT_HP_CORE_STACK: i32 = 1610612743;
pub const PT_HP_CORE_SHM: i32 = 1610612744;
pub const PT_HP_CORE_MMF: i32 = 1610612745;
pub const PT_HP_PARALLEL: i32 = 1610612752;
pub const PT_HP_FASTBIND: i32 = 1610612753;
pub const PT_HP_OPT_ANNOT: i32 = 1610612754;
pub const PT_HP_HSL_ANNOT: i32 = 1610612755;
pub const PT_HP_STACK: i32 = 1610612756;
pub const PT_PARISC_ARCHEXT: i32 = 1879048192;
pub const PT_PARISC_UNWIND: i32 = 1879048193;
pub const PF_PARISC_SBP: i32 = 134217728;
pub const PF_HP_PAGE_SIZE: i32 = 1048576;
pub const PF_HP_FAR_SHARED: i32 = 2097152;
pub const PF_HP_NEAR_SHARED: i32 = 4194304;
pub const PF_HP_CODE: i32 = 16777216;
pub const PF_HP_MODIFY: i32 = 33554432;
pub const PF_HP_LAZYSWAP: i32 = 67108864;
pub const PF_HP_SBP: i32 = 134217728;
pub const EF_ALPHA_32BIT: i32 = 1;
pub const EF_ALPHA_CANRELAX: i32 = 2;
pub const SHT_ALPHA_DEBUG: i32 = 1879048193;
pub const SHT_ALPHA_REGINFO: i32 = 1879048194;
pub const SHF_ALPHA_GPREL: i32 = 268435456;
pub const STO_ALPHA_NOPV: i32 = 128;
pub const STO_ALPHA_STD_GPLOAD: i32 = 136;
pub const R_ALPHA_NONE: i32 = 0;
pub const R_ALPHA_REFLONG: i32 = 1;
pub const R_ALPHA_REFQUAD: i32 = 2;
pub const R_ALPHA_GPREL32: i32 = 3;
pub const R_ALPHA_LITERAL: i32 = 4;
pub const R_ALPHA_LITUSE: i32 = 5;
pub const R_ALPHA_GPDISP: i32 = 6;
pub const R_ALPHA_BRADDR: i32 = 7;
pub const R_ALPHA_HINT: i32 = 8;
pub const R_ALPHA_SREL16: i32 = 9;
pub const R_ALPHA_SREL32: i32 = 10;
pub const R_ALPHA_SREL64: i32 = 11;
pub const R_ALPHA_GPRELHIGH: i32 = 17;
pub const R_ALPHA_GPRELLOW: i32 = 18;
pub const R_ALPHA_GPREL16: i32 = 19;
pub const R_ALPHA_COPY: i32 = 24;
pub const R_ALPHA_GLOB_DAT: i32 = 25;
pub const R_ALPHA_JMP_SLOT: i32 = 26;
pub const R_ALPHA_RELATIVE: i32 = 27;
pub const R_ALPHA_TLS_GD_HI: i32 = 28;
pub const R_ALPHA_TLSGD: i32 = 29;
pub const R_ALPHA_TLS_LDM: i32 = 30;
pub const R_ALPHA_DTPMOD64: i32 = 31;
pub const R_ALPHA_GOTDTPREL: i32 = 32;
pub const R_ALPHA_DTPREL64: i32 = 33;
pub const R_ALPHA_DTPRELHI: i32 = 34;
pub const R_ALPHA_DTPRELLO: i32 = 35;
pub const R_ALPHA_DTPREL16: i32 = 36;
pub const R_ALPHA_GOTTPREL: i32 = 37;
pub const R_ALPHA_TPREL64: i32 = 38;
pub const R_ALPHA_TPRELHI: i32 = 39;
pub const R_ALPHA_TPRELLO: i32 = 40;
pub const R_ALPHA_TPREL16: i32 = 41;
pub const R_ALPHA_NUM: i32 = 46;
pub const LITUSE_ALPHA_ADDR: i32 = 0;
pub const LITUSE_ALPHA_BASE: i32 = 1;
pub const LITUSE_ALPHA_BYTOFF: i32 = 2;
pub const LITUSE_ALPHA_JSR: i32 = 3;
pub const LITUSE_ALPHA_TLS_GD: i32 = 4;
pub const LITUSE_ALPHA_TLS_LDM: i32 = 5;
pub const DT_ALPHA_PLTRO: i32 = 1879048192;
pub const DT_ALPHA_NUM: i32 = 1;
pub const EF_PPC_EMB: i32 = 2147483648;
pub const EF_PPC_RELOCATABLE: i32 = 65536;
pub const EF_PPC_RELOCATABLE_LIB: i32 = 32768;
pub const R_PPC_NONE: i32 = 0;
pub const R_PPC_ADDR32: i32 = 1;
pub const R_PPC_ADDR24: i32 = 2;
pub const R_PPC_ADDR16: i32 = 3;
pub const R_PPC_ADDR16_LO: i32 = 4;
pub const R_PPC_ADDR16_HI: i32 = 5;
pub const R_PPC_ADDR16_HA: i32 = 6;
pub const R_PPC_ADDR14: i32 = 7;
pub const R_PPC_ADDR14_BRTAKEN: i32 = 8;
pub const R_PPC_ADDR14_BRNTAKEN: i32 = 9;
pub const R_PPC_REL24: i32 = 10;
pub const R_PPC_REL14: i32 = 11;
pub const R_PPC_REL14_BRTAKEN: i32 = 12;
pub const R_PPC_REL14_BRNTAKEN: i32 = 13;
pub const R_PPC_GOT16: i32 = 14;
pub const R_PPC_GOT16_LO: i32 = 15;
pub const R_PPC_GOT16_HI: i32 = 16;
pub const R_PPC_GOT16_HA: i32 = 17;
pub const R_PPC_PLTREL24: i32 = 18;
pub const R_PPC_COPY: i32 = 19;
pub const R_PPC_GLOB_DAT: i32 = 20;
pub const R_PPC_JMP_SLOT: i32 = 21;
pub const R_PPC_RELATIVE: i32 = 22;
pub const R_PPC_LOCAL24PC: i32 = 23;
pub const R_PPC_UADDR32: i32 = 24;
pub const R_PPC_UADDR16: i32 = 25;
pub const R_PPC_REL32: i32 = 26;
pub const R_PPC_PLT32: i32 = 27;
pub const R_PPC_PLTREL32: i32 = 28;
pub const R_PPC_PLT16_LO: i32 = 29;
pub const R_PPC_PLT16_HI: i32 = 30;
pub const R_PPC_PLT16_HA: i32 = 31;
pub const R_PPC_SDAREL16: i32 = 32;
pub const R_PPC_SECTOFF: i32 = 33;
pub const R_PPC_SECTOFF_LO: i32 = 34;
pub const R_PPC_SECTOFF_HI: i32 = 35;
pub const R_PPC_SECTOFF_HA: i32 = 36;
pub const R_PPC_TLS: i32 = 67;
pub const R_PPC_DTPMOD32: i32 = 68;
pub const R_PPC_TPREL16: i32 = 69;
pub const R_PPC_TPREL16_LO: i32 = 70;
pub const R_PPC_TPREL16_HI: i32 = 71;
pub const R_PPC_TPREL16_HA: i32 = 72;
pub const R_PPC_TPREL32: i32 = 73;
pub const R_PPC_DTPREL16: i32 = 74;
pub const R_PPC_DTPREL16_LO: i32 = 75;
pub const R_PPC_DTPREL16_HI: i32 = 76;
pub const R_PPC_DTPREL16_HA: i32 = 77;
pub const R_PPC_DTPREL32: i32 = 78;
pub const R_PPC_GOT_TLSGD16: i32 = 79;
pub const R_PPC_GOT_TLSGD16_LO: i32 = 80;
pub const R_PPC_GOT_TLSGD16_HI: i32 = 81;
pub const R_PPC_GOT_TLSGD16_HA: i32 = 82;
pub const R_PPC_GOT_TLSLD16: i32 = 83;
pub const R_PPC_GOT_TLSLD16_LO: i32 = 84;
pub const R_PPC_GOT_TLSLD16_HI: i32 = 85;
pub const R_PPC_GOT_TLSLD16_HA: i32 = 86;
pub const R_PPC_GOT_TPREL16: i32 = 87;
pub const R_PPC_GOT_TPREL16_LO: i32 = 88;
pub const R_PPC_GOT_TPREL16_HI: i32 = 89;
pub const R_PPC_GOT_TPREL16_HA: i32 = 90;
pub const R_PPC_GOT_DTPREL16: i32 = 91;
pub const R_PPC_GOT_DTPREL16_LO: i32 = 92;
pub const R_PPC_GOT_DTPREL16_HI: i32 = 93;
pub const R_PPC_GOT_DTPREL16_HA: i32 = 94;
pub const R_PPC_TLSGD: i32 = 95;
pub const R_PPC_TLSLD: i32 = 96;
pub const R_PPC_EMB_NADDR32: i32 = 101;
pub const R_PPC_EMB_NADDR16: i32 = 102;
pub const R_PPC_EMB_NADDR16_LO: i32 = 103;
pub const R_PPC_EMB_NADDR16_HI: i32 = 104;
pub const R_PPC_EMB_NADDR16_HA: i32 = 105;
pub const R_PPC_EMB_SDAI16: i32 = 106;
pub const R_PPC_EMB_SDA2I16: i32 = 107;
pub const R_PPC_EMB_SDA2REL: i32 = 108;
pub const R_PPC_EMB_SDA21: i32 = 109;
pub const R_PPC_EMB_MRKREF: i32 = 110;
pub const R_PPC_EMB_RELSEC16: i32 = 111;
pub const R_PPC_EMB_RELST_LO: i32 = 112;
pub const R_PPC_EMB_RELST_HI: i32 = 113;
pub const R_PPC_EMB_RELST_HA: i32 = 114;
pub const R_PPC_EMB_BIT_FLD: i32 = 115;
pub const R_PPC_EMB_RELSDA: i32 = 116;
pub const R_PPC_DIAB_SDA21_LO: i32 = 180;
pub const R_PPC_DIAB_SDA21_HI: i32 = 181;
pub const R_PPC_DIAB_SDA21_HA: i32 = 182;
pub const R_PPC_DIAB_RELSDA_LO: i32 = 183;
pub const R_PPC_DIAB_RELSDA_HI: i32 = 184;
pub const R_PPC_DIAB_RELSDA_HA: i32 = 185;
pub const R_PPC_IRELATIVE: i32 = 248;
pub const R_PPC_REL16: i32 = 249;
pub const R_PPC_REL16_LO: i32 = 250;
pub const R_PPC_REL16_HI: i32 = 251;
pub const R_PPC_REL16_HA: i32 = 252;
pub const R_PPC_TOC16: i32 = 255;
pub const DT_PPC_GOT: i32 = 1879048192;
pub const DT_PPC_OPT: i32 = 1879048193;
pub const DT_PPC_NUM: i32 = 2;
pub const PPC_OPT_TLS: i32 = 1;
pub const R_PPC64_NONE: i32 = 0;
pub const R_PPC64_ADDR32: i32 = 1;
pub const R_PPC64_ADDR24: i32 = 2;
pub const R_PPC64_ADDR16: i32 = 3;
pub const R_PPC64_ADDR16_LO: i32 = 4;
pub const R_PPC64_ADDR16_HI: i32 = 5;
pub const R_PPC64_ADDR16_HA: i32 = 6;
pub const R_PPC64_ADDR14: i32 = 7;
pub const R_PPC64_ADDR14_BRTAKEN: i32 = 8;
pub const R_PPC64_ADDR14_BRNTAKEN: i32 = 9;
pub const R_PPC64_REL24: i32 = 10;
pub const R_PPC64_REL14: i32 = 11;
pub const R_PPC64_REL14_BRTAKEN: i32 = 12;
pub const R_PPC64_REL14_BRNTAKEN: i32 = 13;
pub const R_PPC64_GOT16: i32 = 14;
pub const R_PPC64_GOT16_LO: i32 = 15;
pub const R_PPC64_GOT16_HI: i32 = 16;
pub const R_PPC64_GOT16_HA: i32 = 17;
pub const R_PPC64_COPY: i32 = 19;
pub const R_PPC64_GLOB_DAT: i32 = 20;
pub const R_PPC64_JMP_SLOT: i32 = 21;
pub const R_PPC64_RELATIVE: i32 = 22;
pub const R_PPC64_UADDR32: i32 = 24;
pub const R_PPC64_UADDR16: i32 = 25;
pub const R_PPC64_REL32: i32 = 26;
pub const R_PPC64_PLT32: i32 = 27;
pub const R_PPC64_PLTREL32: i32 = 28;
pub const R_PPC64_PLT16_LO: i32 = 29;
pub const R_PPC64_PLT16_HI: i32 = 30;
pub const R_PPC64_PLT16_HA: i32 = 31;
pub const R_PPC64_SECTOFF: i32 = 33;
pub const R_PPC64_SECTOFF_LO: i32 = 34;
pub const R_PPC64_SECTOFF_HI: i32 = 35;
pub const R_PPC64_SECTOFF_HA: i32 = 36;
pub const R_PPC64_ADDR30: i32 = 37;
pub const R_PPC64_ADDR64: i32 = 38;
pub const R_PPC64_ADDR16_HIGHER: i32 = 39;
pub const R_PPC64_ADDR16_HIGHERA: i32 = 40;
pub const R_PPC64_ADDR16_HIGHEST: i32 = 41;
pub const R_PPC64_ADDR16_HIGHESTA: i32 = 42;
pub const R_PPC64_UADDR64: i32 = 43;
pub const R_PPC64_REL64: i32 = 44;
pub const R_PPC64_PLT64: i32 = 45;
pub const R_PPC64_PLTREL64: i32 = 46;
pub const R_PPC64_TOC16: i32 = 47;
pub const R_PPC64_TOC16_LO: i32 = 48;
pub const R_PPC64_TOC16_HI: i32 = 49;
pub const R_PPC64_TOC16_HA: i32 = 50;
pub const R_PPC64_TOC: i32 = 51;
pub const R_PPC64_PLTGOT16: i32 = 52;
pub const R_PPC64_PLTGOT16_LO: i32 = 53;
pub const R_PPC64_PLTGOT16_HI: i32 = 54;
pub const R_PPC64_PLTGOT16_HA: i32 = 55;
pub const R_PPC64_ADDR16_DS: i32 = 56;
pub const R_PPC64_ADDR16_LO_DS: i32 = 57;
pub const R_PPC64_GOT16_DS: i32 = 58;
pub const R_PPC64_GOT16_LO_DS: i32 = 59;
pub const R_PPC64_PLT16_LO_DS: i32 = 60;
pub const R_PPC64_SECTOFF_DS: i32 = 61;
pub const R_PPC64_SECTOFF_LO_DS: i32 = 62;
pub const R_PPC64_TOC16_DS: i32 = 63;
pub const R_PPC64_TOC16_LO_DS: i32 = 64;
pub const R_PPC64_PLTGOT16_DS: i32 = 65;
pub const R_PPC64_PLTGOT16_LO_DS: i32 = 66;
pub const R_PPC64_TLS: i32 = 67;
pub const R_PPC64_DTPMOD64: i32 = 68;
pub const R_PPC64_TPREL16: i32 = 69;
pub const R_PPC64_TPREL16_LO: i32 = 70;
pub const R_PPC64_TPREL16_HI: i32 = 71;
pub const R_PPC64_TPREL16_HA: i32 = 72;
pub const R_PPC64_TPREL64: i32 = 73;
pub const R_PPC64_DTPREL16: i32 = 74;
pub const R_PPC64_DTPREL16_LO: i32 = 75;
pub const R_PPC64_DTPREL16_HI: i32 = 76;
pub const R_PPC64_DTPREL16_HA: i32 = 77;
pub const R_PPC64_DTPREL64: i32 = 78;
pub const R_PPC64_GOT_TLSGD16: i32 = 79;
pub const R_PPC64_GOT_TLSGD16_LO: i32 = 80;
pub const R_PPC64_GOT_TLSGD16_HI: i32 = 81;
pub const R_PPC64_GOT_TLSGD16_HA: i32 = 82;
pub const R_PPC64_GOT_TLSLD16: i32 = 83;
pub const R_PPC64_GOT_TLSLD16_LO: i32 = 84;
pub const R_PPC64_GOT_TLSLD16_HI: i32 = 85;
pub const R_PPC64_GOT_TLSLD16_HA: i32 = 86;
pub const R_PPC64_GOT_TPREL16_DS: i32 = 87;
pub const R_PPC64_GOT_TPREL16_LO_DS: i32 = 88;
pub const R_PPC64_GOT_TPREL16_HI: i32 = 89;
pub const R_PPC64_GOT_TPREL16_HA: i32 = 90;
pub const R_PPC64_GOT_DTPREL16_DS: i32 = 91;
pub const R_PPC64_GOT_DTPREL16_LO_DS: i32 = 92;
pub const R_PPC64_GOT_DTPREL16_HI: i32 = 93;
pub const R_PPC64_GOT_DTPREL16_HA: i32 = 94;
pub const R_PPC64_TPREL16_DS: i32 = 95;
pub const R_PPC64_TPREL16_LO_DS: i32 = 96;
pub const R_PPC64_TPREL16_HIGHER: i32 = 97;
pub const R_PPC64_TPREL16_HIGHERA: i32 = 98;
pub const R_PPC64_TPREL16_HIGHEST: i32 = 99;
pub const R_PPC64_TPREL16_HIGHESTA: i32 = 100;
pub const R_PPC64_DTPREL16_DS: i32 = 101;
pub const R_PPC64_DTPREL16_LO_DS: i32 = 102;
pub const R_PPC64_DTPREL16_HIGHER: i32 = 103;
pub const R_PPC64_DTPREL16_HIGHERA: i32 = 104;
pub const R_PPC64_DTPREL16_HIGHEST: i32 = 105;
pub const R_PPC64_DTPREL16_HIGHESTA: i32 = 106;
pub const R_PPC64_TLSGD: i32 = 107;
pub const R_PPC64_TLSLD: i32 = 108;
pub const R_PPC64_TOCSAVE: i32 = 109;
pub const R_PPC64_ADDR16_HIGH: i32 = 110;
pub const R_PPC64_ADDR16_HIGHA: i32 = 111;
pub const R_PPC64_TPREL16_HIGH: i32 = 112;
pub const R_PPC64_TPREL16_HIGHA: i32 = 113;
pub const R_PPC64_DTPREL16_HIGH: i32 = 114;
pub const R_PPC64_DTPREL16_HIGHA: i32 = 115;
pub const R_PPC64_JMP_IREL: i32 = 247;
pub const R_PPC64_IRELATIVE: i32 = 248;
pub const R_PPC64_REL16: i32 = 249;
pub const R_PPC64_REL16_LO: i32 = 250;
pub const R_PPC64_REL16_HI: i32 = 251;
pub const R_PPC64_REL16_HA: i32 = 252;
pub const EF_PPC64_ABI: i32 = 3;
pub const DT_PPC64_GLINK: i32 = 1879048192;
pub const DT_PPC64_OPD: i32 = 1879048193;
pub const DT_PPC64_OPDSZ: i32 = 1879048194;
pub const DT_PPC64_OPT: i32 = 1879048195;
pub const DT_PPC64_NUM: i32 = 4;
pub const PPC64_OPT_TLS: i32 = 1;
pub const PPC64_OPT_MULTI_TOC: i32 = 2;
pub const STO_PPC64_LOCAL_BIT: i32 = 5;
pub const STO_PPC64_LOCAL_MASK: i32 = 224;
pub const EF_ARM_RELEXEC: i32 = 1;
pub const EF_ARM_HASENTRY: i32 = 2;
pub const EF_ARM_INTERWORK: i32 = 4;
pub const EF_ARM_APCS_26: i32 = 8;
pub const EF_ARM_APCS_FLOAT: i32 = 16;
pub const EF_ARM_PIC: i32 = 32;
pub const EF_ARM_ALIGN8: i32 = 64;
pub const EF_ARM_NEW_ABI: i32 = 128;
pub const EF_ARM_OLD_ABI: i32 = 256;
pub const EF_ARM_SOFT_FLOAT: i32 = 512;
pub const EF_ARM_VFP_FLOAT: i32 = 1024;
pub const EF_ARM_MAVERICK_FLOAT: i32 = 2048;
pub const EF_ARM_ABI_FLOAT_SOFT: i32 = 512;
pub const EF_ARM_ABI_FLOAT_HARD: i32 = 1024;
pub const EF_ARM_SYMSARESORTED: i32 = 4;
pub const EF_ARM_DYNSYMSUSESEGIDX: i32 = 8;
pub const EF_ARM_MAPSYMSFIRST: i32 = 16;
pub const EF_ARM_EABIMASK: i32 = 4278190080;
pub const EF_ARM_BE8: i32 = 8388608;
pub const EF_ARM_LE8: i32 = 4194304;
pub const EF_ARM_EABI_UNKNOWN: i32 = 0;
pub const EF_ARM_EABI_VER1: i32 = 16777216;
pub const EF_ARM_EABI_VER2: i32 = 33554432;
pub const EF_ARM_EABI_VER3: i32 = 50331648;
pub const EF_ARM_EABI_VER4: i32 = 67108864;
pub const EF_ARM_EABI_VER5: i32 = 83886080;
pub const STT_ARM_TFUNC: i32 = 13;
pub const STT_ARM_16BIT: i32 = 15;
pub const SHF_ARM_ENTRYSECT: i32 = 268435456;
pub const SHF_ARM_COMDEF: i32 = 2147483648;
pub const PF_ARM_SB: i32 = 268435456;
pub const PF_ARM_PI: i32 = 536870912;
pub const PF_ARM_ABS: i32 = 1073741824;
pub const PT_ARM_EXIDX: i32 = 1879048193;
pub const SHT_ARM_EXIDX: i32 = 1879048193;
pub const SHT_ARM_PREEMPTMAP: i32 = 1879048194;
pub const SHT_ARM_ATTRIBUTES: i32 = 1879048195;
pub const R_AARCH64_NONE: i32 = 0;
pub const R_AARCH64_P32_ABS32: i32 = 1;
pub const R_AARCH64_P32_COPY: i32 = 180;
pub const R_AARCH64_P32_GLOB_DAT: i32 = 181;
pub const R_AARCH64_P32_JUMP_SLOT: i32 = 182;
pub const R_AARCH64_P32_RELATIVE: i32 = 183;
pub const R_AARCH64_P32_TLS_DTPMOD: i32 = 184;
pub const R_AARCH64_P32_TLS_DTPREL: i32 = 185;
pub const R_AARCH64_P32_TLS_TPREL: i32 = 186;
pub const R_AARCH64_P32_TLSDESC: i32 = 187;
pub const R_AARCH64_P32_IRELATIVE: i32 = 188;
pub const R_AARCH64_ABS64: i32 = 257;
pub const R_AARCH64_ABS32: i32 = 258;
pub const R_AARCH64_ABS16: i32 = 259;
pub const R_AARCH64_PREL64: i32 = 260;
pub const R_AARCH64_PREL32: i32 = 261;
pub const R_AARCH64_PREL16: i32 = 262;
pub const R_AARCH64_MOVW_UABS_G0: i32 = 263;
pub const R_AARCH64_MOVW_UABS_G0_NC: i32 = 264;
pub const R_AARCH64_MOVW_UABS_G1: i32 = 265;
pub const R_AARCH64_MOVW_UABS_G1_NC: i32 = 266;
pub const R_AARCH64_MOVW_UABS_G2: i32 = 267;
pub const R_AARCH64_MOVW_UABS_G2_NC: i32 = 268;
pub const R_AARCH64_MOVW_UABS_G3: i32 = 269;
pub const R_AARCH64_MOVW_SABS_G0: i32 = 270;
pub const R_AARCH64_MOVW_SABS_G1: i32 = 271;
pub const R_AARCH64_MOVW_SABS_G2: i32 = 272;
pub const R_AARCH64_LD_PREL_LO19: i32 = 273;
pub const R_AARCH64_ADR_PREL_LO21: i32 = 274;
pub const R_AARCH64_ADR_PREL_PG_HI21: i32 = 275;
pub const R_AARCH64_ADR_PREL_PG_HI21_NC: i32 = 276;
pub const R_AARCH64_ADD_ABS_LO12_NC: i32 = 277;
pub const R_AARCH64_LDST8_ABS_LO12_NC: i32 = 278;
pub const R_AARCH64_TSTBR14: i32 = 279;
pub const R_AARCH64_CONDBR19: i32 = 280;
pub const R_AARCH64_JUMP26: i32 = 282;
pub const R_AARCH64_CALL26: i32 = 283;
pub const R_AARCH64_LDST16_ABS_LO12_NC: i32 = 284;
pub const R_AARCH64_LDST32_ABS_LO12_NC: i32 = 285;
pub const R_AARCH64_LDST64_ABS_LO12_NC: i32 = 286;
pub const R_AARCH64_MOVW_PREL_G0: i32 = 287;
pub const R_AARCH64_MOVW_PREL_G0_NC: i32 = 288;
pub const R_AARCH64_MOVW_PREL_G1: i32 = 289;
pub const R_AARCH64_MOVW_PREL_G1_NC: i32 = 290;
pub const R_AARCH64_MOVW_PREL_G2: i32 = 291;
pub const R_AARCH64_MOVW_PREL_G2_NC: i32 = 292;
pub const R_AARCH64_MOVW_PREL_G3: i32 = 293;
pub const R_AARCH64_LDST128_ABS_LO12_NC: i32 = 299;
pub const R_AARCH64_MOVW_GOTOFF_G0: i32 = 300;
pub const R_AARCH64_MOVW_GOTOFF_G0_NC: i32 = 301;
pub const R_AARCH64_MOVW_GOTOFF_G1: i32 = 302;
pub const R_AARCH64_MOVW_GOTOFF_G1_NC: i32 = 303;
pub const R_AARCH64_MOVW_GOTOFF_G2: i32 = 304;
pub const R_AARCH64_MOVW_GOTOFF_G2_NC: i32 = 305;
pub const R_AARCH64_MOVW_GOTOFF_G3: i32 = 306;
pub const R_AARCH64_GOTREL64: i32 = 307;
pub const R_AARCH64_GOTREL32: i32 = 308;
pub const R_AARCH64_GOT_LD_PREL19: i32 = 309;
pub const R_AARCH64_LD64_GOTOFF_LO15: i32 = 310;
pub const R_AARCH64_ADR_GOT_PAGE: i32 = 311;
pub const R_AARCH64_LD64_GOT_LO12_NC: i32 = 312;
pub const R_AARCH64_LD64_GOTPAGE_LO15: i32 = 313;
pub const R_AARCH64_TLSGD_ADR_PREL21: i32 = 512;
pub const R_AARCH64_TLSGD_ADR_PAGE21: i32 = 513;
pub const R_AARCH64_TLSGD_ADD_LO12_NC: i32 = 514;
pub const R_AARCH64_TLSGD_MOVW_G1: i32 = 515;
pub const R_AARCH64_TLSGD_MOVW_G0_NC: i32 = 516;
pub const R_AARCH64_TLSLD_ADR_PREL21: i32 = 517;
pub const R_AARCH64_TLSLD_ADR_PAGE21: i32 = 518;
pub const R_AARCH64_TLSLD_ADD_LO12_NC: i32 = 519;
pub const R_AARCH64_TLSLD_MOVW_G1: i32 = 520;
pub const R_AARCH64_TLSLD_MOVW_G0_NC: i32 = 521;
pub const R_AARCH64_TLSLD_LD_PREL19: i32 = 522;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G2: i32 = 523;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G1: i32 = 524;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: i32 = 525;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G0: i32 = 526;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: i32 = 527;
pub const R_AARCH64_TLSLD_ADD_DTPREL_HI12: i32 = 528;
pub const R_AARCH64_TLSLD_ADD_DTPREL_LO12: i32 = 529;
pub const R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: i32 = 530;
pub const R_AARCH64_TLSLD_LDST8_DTPREL_LO12: i32 = 531;
pub const R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: i32 = 532;
pub const R_AARCH64_TLSLD_LDST16_DTPREL_LO12: i32 = 533;
pub const R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: i32 = 534;
pub const R_AARCH64_TLSLD_LDST32_DTPREL_LO12: i32 = 535;
pub const R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: i32 = 536;
pub const R_AARCH64_TLSLD_LDST64_DTPREL_LO12: i32 = 537;
pub const R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: i32 = 538;
pub const R_AARCH64_TLSIE_MOVW_GOTTPREL_G1: i32 = 539;
pub const R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: i32 = 540;
pub const R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: i32 = 541;
pub const R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: i32 = 542;
pub const R_AARCH64_TLSIE_LD_GOTTPREL_PREL19: i32 = 543;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G2: i32 = 544;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G1: i32 = 545;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G1_NC: i32 = 546;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G0: i32 = 547;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G0_NC: i32 = 548;
pub const R_AARCH64_TLSLE_ADD_TPREL_HI12: i32 = 549;
pub const R_AARCH64_TLSLE_ADD_TPREL_LO12: i32 = 550;
pub const R_AARCH64_TLSLE_ADD_TPREL_LO12_NC: i32 = 551;
pub const R_AARCH64_TLSLE_LDST8_TPREL_LO12: i32 = 552;
pub const R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: i32 = 553;
pub const R_AARCH64_TLSLE_LDST16_TPREL_LO12: i32 = 554;
pub const R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: i32 = 555;
pub const R_AARCH64_TLSLE_LDST32_TPREL_LO12: i32 = 556;
pub const R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: i32 = 557;
pub const R_AARCH64_TLSLE_LDST64_TPREL_LO12: i32 = 558;
pub const R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: i32 = 559;
pub const R_AARCH64_TLSDESC_LD_PREL19: i32 = 560;
pub const R_AARCH64_TLSDESC_ADR_PREL21: i32 = 561;
pub const R_AARCH64_TLSDESC_ADR_PAGE21: i32 = 562;
pub const R_AARCH64_TLSDESC_LD64_LO12: i32 = 563;
pub const R_AARCH64_TLSDESC_ADD_LO12: i32 = 564;
pub const R_AARCH64_TLSDESC_OFF_G1: i32 = 565;
pub const R_AARCH64_TLSDESC_OFF_G0_NC: i32 = 566;
pub const R_AARCH64_TLSDESC_LDR: i32 = 567;
pub const R_AARCH64_TLSDESC_ADD: i32 = 568;
pub const R_AARCH64_TLSDESC_CALL: i32 = 569;
pub const R_AARCH64_TLSLE_LDST128_TPREL_LO12: i32 = 570;
pub const R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC: i32 = 571;
pub const R_AARCH64_TLSLD_LDST128_DTPREL_LO12: i32 = 572;
pub const R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC: i32 = 573;
pub const R_AARCH64_COPY: i32 = 1024;
pub const R_AARCH64_GLOB_DAT: i32 = 1025;
pub const R_AARCH64_JUMP_SLOT: i32 = 1026;
pub const R_AARCH64_RELATIVE: i32 = 1027;
pub const R_AARCH64_TLS_DTPMOD: i32 = 1028;
pub const R_AARCH64_TLS_DTPMOD64: i32 = 1028;
pub const R_AARCH64_TLS_DTPREL: i32 = 1029;
pub const R_AARCH64_TLS_DTPREL64: i32 = 1029;
pub const R_AARCH64_TLS_TPREL: i32 = 1030;
pub const R_AARCH64_TLS_TPREL64: i32 = 1030;
pub const R_AARCH64_TLSDESC: i32 = 1031;
pub const R_ARM_NONE: i32 = 0;
pub const R_ARM_PC24: i32 = 1;
pub const R_ARM_ABS32: i32 = 2;
pub const R_ARM_REL32: i32 = 3;
pub const R_ARM_PC13: i32 = 4;
pub const R_ARM_ABS16: i32 = 5;
pub const R_ARM_ABS12: i32 = 6;
pub const R_ARM_THM_ABS5: i32 = 7;
pub const R_ARM_ABS8: i32 = 8;
pub const R_ARM_SBREL32: i32 = 9;
pub const R_ARM_THM_PC22: i32 = 10;
pub const R_ARM_THM_PC8: i32 = 11;
pub const R_ARM_AMP_VCALL9: i32 = 12;
pub const R_ARM_TLS_DESC: i32 = 13;
pub const R_ARM_THM_SWI8: i32 = 14;
pub const R_ARM_XPC25: i32 = 15;
pub const R_ARM_THM_XPC22: i32 = 16;
pub const R_ARM_TLS_DTPMOD32: i32 = 17;
pub const R_ARM_TLS_DTPOFF32: i32 = 18;
pub const R_ARM_TLS_TPOFF32: i32 = 19;
pub const R_ARM_COPY: i32 = 20;
pub const R_ARM_GLOB_DAT: i32 = 21;
pub const R_ARM_JUMP_SLOT: i32 = 22;
pub const R_ARM_RELATIVE: i32 = 23;
pub const R_ARM_GOTOFF: i32 = 24;
pub const R_ARM_GOTPC: i32 = 25;
pub const R_ARM_GOT32: i32 = 26;
pub const R_ARM_PLT32: i32 = 27;
pub const R_ARM_CALL: i32 = 28;
pub const R_ARM_JUMP24: i32 = 29;
pub const R_ARM_THM_JUMP24: i32 = 30;
pub const R_ARM_BASE_ABS: i32 = 31;
pub const R_ARM_ALU_PCREL_7_0: i32 = 32;
pub const R_ARM_ALU_PCREL_15_8: i32 = 33;
pub const R_ARM_ALU_PCREL_23_15: i32 = 34;
pub const R_ARM_LDR_SBREL_11_0: i32 = 35;
pub const R_ARM_ALU_SBREL_19_12: i32 = 36;
pub const R_ARM_ALU_SBREL_27_20: i32 = 37;
pub const R_ARM_TARGET1: i32 = 38;
pub const R_ARM_SBREL31: i32 = 39;
pub const R_ARM_V4BX: i32 = 40;
pub const R_ARM_TARGET2: i32 = 41;
pub const R_ARM_PREL31: i32 = 42;
pub const R_ARM_MOVW_ABS_NC: i32 = 43;
pub const R_ARM_MOVT_ABS: i32 = 44;
pub const R_ARM_MOVW_PREL_NC: i32 = 45;
pub const R_ARM_MOVT_PREL: i32 = 46;
pub const R_ARM_THM_MOVW_ABS_NC: i32 = 47;
pub const R_ARM_THM_MOVT_ABS: i32 = 48;
pub const R_ARM_THM_MOVW_PREL_NC: i32 = 49;
pub const R_ARM_THM_MOVT_PREL: i32 = 50;
pub const R_ARM_THM_JUMP19: i32 = 51;
pub const R_ARM_THM_JUMP6: i32 = 52;
pub const R_ARM_THM_ALU_PREL_11_0: i32 = 53;
pub const R_ARM_THM_PC12: i32 = 54;
pub const R_ARM_ABS32_NOI: i32 = 55;
pub const R_ARM_REL32_NOI: i32 = 56;
pub const R_ARM_ALU_PC_G0_NC: i32 = 57;
pub const R_ARM_ALU_PC_G0: i32 = 58;
pub const R_ARM_ALU_PC_G1_NC: i32 = 59;
pub const R_ARM_ALU_PC_G1: i32 = 60;
pub const R_ARM_ALU_PC_G2: i32 = 61;
pub const R_ARM_LDR_PC_G1: i32 = 62;
pub const R_ARM_LDR_PC_G2: i32 = 63;
pub const R_ARM_LDRS_PC_G0: i32 = 64;
pub const R_ARM_LDRS_PC_G1: i32 = 65;
pub const R_ARM_LDRS_PC_G2: i32 = 66;
pub const R_ARM_LDC_PC_G0: i32 = 67;
pub const R_ARM_LDC_PC_G1: i32 = 68;
pub const R_ARM_LDC_PC_G2: i32 = 69;
pub const R_ARM_ALU_SB_G0_NC: i32 = 70;
pub const R_ARM_ALU_SB_G0: i32 = 71;
pub const R_ARM_ALU_SB_G1_NC: i32 = 72;
pub const R_ARM_ALU_SB_G1: i32 = 73;
pub const R_ARM_ALU_SB_G2: i32 = 74;
pub const R_ARM_LDR_SB_G0: i32 = 75;
pub const R_ARM_LDR_SB_G1: i32 = 76;
pub const R_ARM_LDR_SB_G2: i32 = 77;
pub const R_ARM_LDRS_SB_G0: i32 = 78;
pub const R_ARM_LDRS_SB_G1: i32 = 79;
pub const R_ARM_LDRS_SB_G2: i32 = 80;
pub const R_ARM_LDC_SB_G0: i32 = 81;
pub const R_ARM_LDC_SB_G1: i32 = 82;
pub const R_ARM_LDC_SB_G2: i32 = 83;
pub const R_ARM_MOVW_BREL_NC: i32 = 84;
pub const R_ARM_MOVT_BREL: i32 = 85;
pub const R_ARM_MOVW_BREL: i32 = 86;
pub const R_ARM_THM_MOVW_BREL_NC: i32 = 87;
pub const R_ARM_THM_MOVT_BREL: i32 = 88;
pub const R_ARM_THM_MOVW_BREL: i32 = 89;
pub const R_ARM_TLS_GOTDESC: i32 = 90;
pub const R_ARM_TLS_CALL: i32 = 91;
pub const R_ARM_TLS_DESCSEQ: i32 = 92;
pub const R_ARM_THM_TLS_CALL: i32 = 93;
pub const R_ARM_PLT32_ABS: i32 = 94;
pub const R_ARM_GOT_ABS: i32 = 95;
pub const R_ARM_GOT_PREL: i32 = 96;
pub const R_ARM_GOT_BREL12: i32 = 97;
pub const R_ARM_GOTOFF12: i32 = 98;
pub const R_ARM_GOTRELAX: i32 = 99;
pub const R_ARM_GNU_VTENTRY: i32 = 100;
pub const R_ARM_GNU_VTINHERIT: i32 = 101;
pub const R_ARM_THM_PC11: i32 = 102;
pub const R_ARM_THM_PC9: i32 = 103;
pub const R_ARM_TLS_GD32: i32 = 104;
pub const R_ARM_TLS_LDM32: i32 = 105;
pub const R_ARM_TLS_LDO32: i32 = 106;
pub const R_ARM_TLS_IE32: i32 = 107;
pub const R_ARM_TLS_LE32: i32 = 108;
pub const R_ARM_TLS_LDO12: i32 = 109;
pub const R_ARM_TLS_LE12: i32 = 110;
pub const R_ARM_TLS_IE12GP: i32 = 111;
pub const R_ARM_ME_TOO: i32 = 128;
pub const R_ARM_THM_TLS_DESCSEQ: i32 = 129;
pub const R_ARM_THM_TLS_DESCSEQ16: i32 = 129;
pub const R_ARM_THM_TLS_DESCSEQ32: i32 = 130;
pub const R_ARM_THM_GOT_BREL12: i32 = 131;
pub const R_ARM_IRELATIVE: i32 = 160;
pub const R_ARM_RXPC25: i32 = 249;
pub const R_ARM_RSBREL32: i32 = 250;
pub const R_ARM_THM_RPC22: i32 = 251;
pub const R_ARM_RREL32: i32 = 252;
pub const R_ARM_RABS22: i32 = 253;
pub const R_ARM_RPC24: i32 = 254;
pub const R_ARM_RBASE: i32 = 255;
pub const R_ARM_NUM: i32 = 256;
pub const EF_IA_64_MASKOS: i32 = 15;
pub const EF_IA_64_ABI64: i32 = 16;
pub const EF_IA_64_ARCH: i32 = 4278190080;
pub const PT_IA_64_ARCHEXT: i32 = 1879048192;
pub const PT_IA_64_UNWIND: i32 = 1879048193;
pub const PT_IA_64_HP_OPT_ANOT: i32 = 1610612754;
pub const PT_IA_64_HP_HSL_ANOT: i32 = 1610612755;
pub const PT_IA_64_HP_STACK: i32 = 1610612756;
pub const PF_IA_64_NORECOV: i32 = 2147483648;
pub const SHT_IA_64_EXT: i32 = 1879048192;
pub const SHT_IA_64_UNWIND: i32 = 1879048193;
pub const SHF_IA_64_SHORT: i32 = 268435456;
pub const SHF_IA_64_NORECOV: i32 = 536870912;
pub const DT_IA_64_PLT_RESERVE: i32 = 1879048192;
pub const DT_IA_64_NUM: i32 = 1;
pub const R_IA64_NONE: i32 = 0;
pub const R_IA64_IMM14: i32 = 33;
pub const R_IA64_IMM22: i32 = 34;
pub const R_IA64_IMM64: i32 = 35;
pub const R_IA64_DIR32MSB: i32 = 36;
pub const R_IA64_DIR32LSB: i32 = 37;
pub const R_IA64_DIR64MSB: i32 = 38;
pub const R_IA64_DIR64LSB: i32 = 39;
pub const R_IA64_GPREL22: i32 = 42;
pub const R_IA64_GPREL64I: i32 = 43;
pub const R_IA64_GPREL32MSB: i32 = 44;
pub const R_IA64_GPREL32LSB: i32 = 45;
pub const R_IA64_GPREL64MSB: i32 = 46;
pub const R_IA64_GPREL64LSB: i32 = 47;
pub const R_IA64_LTOFF22: i32 = 50;
pub const R_IA64_LTOFF64I: i32 = 51;
pub const R_IA64_PLTOFF22: i32 = 58;
pub const R_IA64_PLTOFF64I: i32 = 59;
pub const R_IA64_PLTOFF64MSB: i32 = 62;
pub const R_IA64_PLTOFF64LSB: i32 = 63;
pub const R_IA64_FPTR64I: i32 = 67;
pub const R_IA64_FPTR32MSB: i32 = 68;
pub const R_IA64_FPTR32LSB: i32 = 69;
pub const R_IA64_FPTR64MSB: i32 = 70;
pub const R_IA64_FPTR64LSB: i32 = 71;
pub const R_IA64_PCREL60B: i32 = 72;
pub const R_IA64_PCREL21B: i32 = 73;
pub const R_IA64_PCREL21M: i32 = 74;
pub const R_IA64_PCREL21F: i32 = 75;
pub const R_IA64_PCREL32MSB: i32 = 76;
pub const R_IA64_PCREL32LSB: i32 = 77;
pub const R_IA64_PCREL64MSB: i32 = 78;
pub const R_IA64_PCREL64LSB: i32 = 79;
pub const R_IA64_LTOFF_FPTR22: i32 = 82;
pub const R_IA64_LTOFF_FPTR64I: i32 = 83;
pub const R_IA64_LTOFF_FPTR32MSB: i32 = 84;
pub const R_IA64_LTOFF_FPTR32LSB: i32 = 85;
pub const R_IA64_LTOFF_FPTR64MSB: i32 = 86;
pub const R_IA64_LTOFF_FPTR64LSB: i32 = 87;
pub const R_IA64_SEGREL32MSB: i32 = 92;
pub const R_IA64_SEGREL32LSB: i32 = 93;
pub const R_IA64_SEGREL64MSB: i32 = 94;
pub const R_IA64_SEGREL64LSB: i32 = 95;
pub const R_IA64_SECREL32MSB: i32 = 100;
pub const R_IA64_SECREL32LSB: i32 = 101;
pub const R_IA64_SECREL64MSB: i32 = 102;
pub const R_IA64_SECREL64LSB: i32 = 103;
pub const R_IA64_REL32MSB: i32 = 108;
pub const R_IA64_REL32LSB: i32 = 109;
pub const R_IA64_REL64MSB: i32 = 110;
pub const R_IA64_REL64LSB: i32 = 111;
pub const R_IA64_LTV32MSB: i32 = 116;
pub const R_IA64_LTV32LSB: i32 = 117;
pub const R_IA64_LTV64MSB: i32 = 118;
pub const R_IA64_LTV64LSB: i32 = 119;
pub const R_IA64_PCREL21BI: i32 = 121;
pub const R_IA64_PCREL22: i32 = 122;
pub const R_IA64_PCREL64I: i32 = 123;
pub const R_IA64_IPLTMSB: i32 = 128;
pub const R_IA64_IPLTLSB: i32 = 129;
pub const R_IA64_COPY: i32 = 132;
pub const R_IA64_SUB: i32 = 133;
pub const R_IA64_LTOFF22X: i32 = 134;
pub const R_IA64_LDXMOV: i32 = 135;
pub const R_IA64_TPREL14: i32 = 145;
pub const R_IA64_TPREL22: i32 = 146;
pub const R_IA64_TPREL64I: i32 = 147;
pub const R_IA64_TPREL64MSB: i32 = 150;
pub const R_IA64_TPREL64LSB: i32 = 151;
pub const R_IA64_LTOFF_TPREL22: i32 = 154;
pub const R_IA64_DTPMOD64MSB: i32 = 166;
pub const R_IA64_DTPMOD64LSB: i32 = 167;
pub const R_IA64_LTOFF_DTPMOD22: i32 = 170;
pub const R_IA64_DTPREL14: i32 = 177;
pub const R_IA64_DTPREL22: i32 = 178;
pub const R_IA64_DTPREL64I: i32 = 179;
pub const R_IA64_DTPREL32MSB: i32 = 180;
pub const R_IA64_DTPREL32LSB: i32 = 181;
pub const R_IA64_DTPREL64MSB: i32 = 182;
pub const R_IA64_DTPREL64LSB: i32 = 183;
pub const R_IA64_LTOFF_DTPREL22: i32 = 186;
pub const EF_SH_MACH_MASK: i32 = 31;
pub const EF_SH_UNKNOWN: i32 = 0;
pub const EF_SH1: i32 = 1;
pub const EF_SH2: i32 = 2;
pub const EF_SH3: i32 = 3;
pub const EF_SH_DSP: i32 = 4;
pub const EF_SH3_DSP: i32 = 5;
pub const EF_SH4AL_DSP: i32 = 6;
pub const EF_SH3E: i32 = 8;
pub const EF_SH4: i32 = 9;
pub const EF_SH2E: i32 = 11;
pub const EF_SH4A: i32 = 12;
pub const EF_SH2A: i32 = 13;
pub const EF_SH4_NOFPU: i32 = 16;
pub const EF_SH4A_NOFPU: i32 = 17;
pub const EF_SH4_NOMMU_NOFPU: i32 = 18;
pub const EF_SH2A_NOFPU: i32 = 19;
pub const EF_SH3_NOMMU: i32 = 20;
pub const EF_SH2A_SH4_NOFPU: i32 = 21;
pub const EF_SH2A_SH3_NOFPU: i32 = 22;
pub const EF_SH2A_SH4: i32 = 23;
pub const EF_SH2A_SH3E: i32 = 24;
pub const R_SH_NONE: i32 = 0;
pub const R_SH_DIR32: i32 = 1;
pub const R_SH_REL32: i32 = 2;
pub const R_SH_DIR8WPN: i32 = 3;
pub const R_SH_IND12W: i32 = 4;
pub const R_SH_DIR8WPL: i32 = 5;
pub const R_SH_DIR8WPZ: i32 = 6;
pub const R_SH_DIR8BP: i32 = 7;
pub const R_SH_DIR8W: i32 = 8;
pub const R_SH_DIR8L: i32 = 9;
pub const R_SH_SWITCH16: i32 = 25;
pub const R_SH_SWITCH32: i32 = 26;
pub const R_SH_USES: i32 = 27;
pub const R_SH_COUNT: i32 = 28;
pub const R_SH_ALIGN: i32 = 29;
pub const R_SH_CODE: i32 = 30;
pub const R_SH_DATA: i32 = 31;
pub const R_SH_LABEL: i32 = 32;
pub const R_SH_SWITCH8: i32 = 33;
pub const R_SH_GNU_VTINHERIT: i32 = 34;
pub const R_SH_GNU_VTENTRY: i32 = 35;
pub const R_SH_TLS_GD_32: i32 = 144;
pub const R_SH_TLS_LD_32: i32 = 145;
pub const R_SH_TLS_LDO_32: i32 = 146;
pub const R_SH_TLS_IE_32: i32 = 147;
pub const R_SH_TLS_LE_32: i32 = 148;
pub const R_SH_TLS_DTPMOD32: i32 = 149;
pub const R_SH_TLS_DTPOFF32: i32 = 150;
pub const R_SH_TLS_TPOFF32: i32 = 151;
pub const R_SH_GOT32: i32 = 160;
pub const R_SH_PLT32: i32 = 161;
pub const R_SH_COPY: i32 = 162;
pub const R_SH_GLOB_DAT: i32 = 163;
pub const R_SH_JMP_SLOT: i32 = 164;
pub const R_SH_RELATIVE: i32 = 165;
pub const R_SH_GOTOFF: i32 = 166;
pub const R_SH_GOTPC: i32 = 167;
pub const R_SH_GOT20: i32 = 201;
pub const R_SH_GOTOFF20: i32 = 202;
pub const R_SH_GOTFUNCDESC: i32 = 203;
pub const R_SH_GOTFUNCDEST20: i32 = 204;
pub const R_SH_GOTOFFFUNCDESC: i32 = 205;
pub const R_SH_GOTOFFFUNCDEST20: i32 = 206;
pub const R_SH_FUNCDESC: i32 = 207;
pub const R_SH_FUNCDESC_VALUE: i32 = 208;
pub const R_SH_NUM: i32 = 256;
pub const R_390_NONE: i32 = 0;
pub const R_390_8: i32 = 1;
pub const R_390_12: i32 = 2;
pub const R_390_16: i32 = 3;
pub const R_390_32: i32 = 4;
pub const R_390_PC32: i32 = 5;
pub const R_390_GOT12: i32 = 6;
pub const R_390_GOT32: i32 = 7;
pub const R_390_PLT32: i32 = 8;
pub const R_390_COPY: i32 = 9;
pub const R_390_GLOB_DAT: i32 = 10;
pub const R_390_JMP_SLOT: i32 = 11;
pub const R_390_RELATIVE: i32 = 12;
pub const R_390_GOTOFF32: i32 = 13;
pub const R_390_GOTPC: i32 = 14;
pub const R_390_GOT16: i32 = 15;
pub const R_390_PC16: i32 = 16;
pub const R_390_PC16DBL: i32 = 17;
pub const R_390_PLT16DBL: i32 = 18;
pub const R_390_PC32DBL: i32 = 19;
pub const R_390_PLT32DBL: i32 = 20;
pub const R_390_GOTPCDBL: i32 = 21;
pub const R_390_64: i32 = 22;
pub const R_390_PC64: i32 = 23;
pub const R_390_GOT64: i32 = 24;
pub const R_390_PLT64: i32 = 25;
pub const R_390_GOTENT: i32 = 26;
pub const R_390_GOTOFF16: i32 = 27;
pub const R_390_GOTOFF64: i32 = 28;
pub const R_390_GOTPLT12: i32 = 29;
pub const R_390_GOTPLT16: i32 = 30;
pub const R_390_GOTPLT32: i32 = 31;
pub const R_390_GOTPLT64: i32 = 32;
pub const R_390_GOTPLTENT: i32 = 33;
pub const R_390_PLTOFF16: i32 = 34;
pub const R_390_PLTOFF32: i32 = 35;
pub const R_390_PLTOFF64: i32 = 36;
pub const R_390_TLS_LOAD: i32 = 37;
pub const R_390_TLS_GDCALL: i32 = 38;
pub const R_390_TLS_LDCALL: i32 = 39;
pub const R_390_TLS_GD32: i32 = 40;
pub const R_390_TLS_GD64: i32 = 41;
pub const R_390_TLS_GOTIE12: i32 = 42;
pub const R_390_TLS_GOTIE32: i32 = 43;
pub const R_390_TLS_GOTIE64: i32 = 44;
pub const R_390_TLS_LDM32: i32 = 45;
pub const R_390_TLS_LDM64: i32 = 46;
pub const R_390_TLS_IE32: i32 = 47;
pub const R_390_TLS_IE64: i32 = 48;
pub const R_390_TLS_IEENT: i32 = 49;
pub const R_390_TLS_LE32: i32 = 50;
pub const R_390_TLS_LE64: i32 = 51;
pub const R_390_TLS_LDO32: i32 = 52;
pub const R_390_TLS_LDO64: i32 = 53;
pub const R_390_TLS_DTPMOD: i32 = 54;
pub const R_390_TLS_DTPOFF: i32 = 55;
pub const R_390_TLS_TPOFF: i32 = 56;
pub const R_390_20: i32 = 57;
pub const R_390_GOT20: i32 = 58;
pub const R_390_GOTPLT20: i32 = 59;
pub const R_390_TLS_GOTIE20: i32 = 60;
pub const R_390_NUM: i32 = 61;
pub const R_CRIS_NONE: i32 = 0;
pub const R_CRIS_8: i32 = 1;
pub const R_CRIS_16: i32 = 2;
pub const R_CRIS_32: i32 = 3;
pub const R_CRIS_8_PCREL: i32 = 4;
pub const R_CRIS_16_PCREL: i32 = 5;
pub const R_CRIS_32_PCREL: i32 = 6;
pub const R_CRIS_GNU_VTINHERIT: i32 = 7;
pub const R_CRIS_GNU_VTENTRY: i32 = 8;
pub const R_CRIS_COPY: i32 = 9;
pub const R_CRIS_GLOB_DAT: i32 = 10;
pub const R_CRIS_JUMP_SLOT: i32 = 11;
pub const R_CRIS_RELATIVE: i32 = 12;
pub const R_CRIS_16_GOT: i32 = 13;
pub const R_CRIS_32_GOT: i32 = 14;
pub const R_CRIS_16_GOTPLT: i32 = 15;
pub const R_CRIS_32_GOTPLT: i32 = 16;
pub const R_CRIS_32_GOTREL: i32 = 17;
pub const R_CRIS_32_PLT_GOTREL: i32 = 18;
pub const R_CRIS_32_PLT_PCREL: i32 = 19;
pub const R_CRIS_NUM: i32 = 20;
pub const R_X86_64_NONE: i32 = 0;
pub const R_X86_64_64: i32 = 1;
pub const R_X86_64_PC32: i32 = 2;
pub const R_X86_64_GOT32: i32 = 3;
pub const R_X86_64_PLT32: i32 = 4;
pub const R_X86_64_COPY: i32 = 5;
pub const R_X86_64_GLOB_DAT: i32 = 6;
pub const R_X86_64_JUMP_SLOT: i32 = 7;
pub const R_X86_64_RELATIVE: i32 = 8;
pub const R_X86_64_GOTPCREL: i32 = 9;
pub const R_X86_64_32: i32 = 10;
pub const R_X86_64_32S: i32 = 11;
pub const R_X86_64_16: i32 = 12;
pub const R_X86_64_PC16: i32 = 13;
pub const R_X86_64_8: i32 = 14;
pub const R_X86_64_PC8: i32 = 15;
pub const R_X86_64_DTPMOD64: i32 = 16;
pub const R_X86_64_DTPOFF64: i32 = 17;
pub const R_X86_64_TPOFF64: i32 = 18;
pub const R_X86_64_TLSGD: i32 = 19;
pub const R_X86_64_TLSLD: i32 = 20;
pub const R_X86_64_DTPOFF32: i32 = 21;
pub const R_X86_64_GOTTPOFF: i32 = 22;
pub const R_X86_64_TPOFF32: i32 = 23;
pub const R_X86_64_PC64: i32 = 24;
pub const R_X86_64_GOTOFF64: i32 = 25;
pub const R_X86_64_GOTPC32: i32 = 26;
pub const R_X86_64_GOT64: i32 = 27;
pub const R_X86_64_GOTPCREL64: i32 = 28;
pub const R_X86_64_GOTPC64: i32 = 29;
pub const R_X86_64_GOTPLT64: i32 = 30;
pub const R_X86_64_PLTOFF64: i32 = 31;
pub const R_X86_64_SIZE32: i32 = 32;
pub const R_X86_64_SIZE64: i32 = 33;
pub const R_X86_64_GOTPC32_TLSDESC: i32 = 34;
pub const R_X86_64_TLSDESC_CALL: i32 = 35;
pub const R_X86_64_TLSDESC: i32 = 36;
pub const R_X86_64_IRELATIVE: i32 = 37;
pub const R_X86_64_RELATIVE64: i32 = 38;
pub const R_X86_64_GOTPCRELX: i32 = 41;
pub const R_X86_64_REX_GOTPCRELX: i32 = 42;
pub const R_X86_64_NUM: i32 = 43;
pub const R_MN10300_NONE: i32 = 0;
pub const R_MN10300_32: i32 = 1;
pub const R_MN10300_16: i32 = 2;
pub const R_MN10300_8: i32 = 3;
pub const R_MN10300_PCREL32: i32 = 4;
pub const R_MN10300_PCREL16: i32 = 5;
pub const R_MN10300_PCREL8: i32 = 6;
pub const R_MN10300_GNU_VTINHERIT: i32 = 7;
pub const R_MN10300_GNU_VTENTRY: i32 = 8;
pub const R_MN10300_24: i32 = 9;
pub const R_MN10300_GOTPC32: i32 = 10;
pub const R_MN10300_GOTPC16: i32 = 11;
pub const R_MN10300_GOTOFF32: i32 = 12;
pub const R_MN10300_GOTOFF24: i32 = 13;
pub const R_MN10300_GOTOFF16: i32 = 14;
pub const R_MN10300_PLT32: i32 = 15;
pub const R_MN10300_PLT16: i32 = 16;
pub const R_MN10300_GOT32: i32 = 17;
pub const R_MN10300_GOT24: i32 = 18;
pub const R_MN10300_GOT16: i32 = 19;
pub const R_MN10300_COPY: i32 = 20;
pub const R_MN10300_GLOB_DAT: i32 = 21;
pub const R_MN10300_JMP_SLOT: i32 = 22;
pub const R_MN10300_RELATIVE: i32 = 23;
pub const R_MN10300_NUM: i32 = 24;
pub const R_M32R_NONE: i32 = 0;
pub const R_M32R_16: i32 = 1;
pub const R_M32R_32: i32 = 2;
pub const R_M32R_24: i32 = 3;
pub const R_M32R_10_PCREL: i32 = 4;
pub const R_M32R_18_PCREL: i32 = 5;
pub const R_M32R_26_PCREL: i32 = 6;
pub const R_M32R_HI16_ULO: i32 = 7;
pub const R_M32R_HI16_SLO: i32 = 8;
pub const R_M32R_LO16: i32 = 9;
pub const R_M32R_SDA16: i32 = 10;
pub const R_M32R_GNU_VTINHERIT: i32 = 11;
pub const R_M32R_GNU_VTENTRY: i32 = 12;
pub const R_M32R_16_RELA: i32 = 33;
pub const R_M32R_32_RELA: i32 = 34;
pub const R_M32R_24_RELA: i32 = 35;
pub const R_M32R_10_PCREL_RELA: i32 = 36;
pub const R_M32R_18_PCREL_RELA: i32 = 37;
pub const R_M32R_26_PCREL_RELA: i32 = 38;
pub const R_M32R_HI16_ULO_RELA: i32 = 39;
pub const R_M32R_HI16_SLO_RELA: i32 = 40;
pub const R_M32R_LO16_RELA: i32 = 41;
pub const R_M32R_SDA16_RELA: i32 = 42;
pub const R_M32R_RELA_GNU_VTINHERIT: i32 = 43;
pub const R_M32R_RELA_GNU_VTENTRY: i32 = 44;
pub const R_M32R_REL32: i32 = 45;
pub const R_M32R_GOT24: i32 = 48;
pub const R_M32R_26_PLTREL: i32 = 49;
pub const R_M32R_COPY: i32 = 50;
pub const R_M32R_GLOB_DAT: i32 = 51;
pub const R_M32R_JMP_SLOT: i32 = 52;
pub const R_M32R_RELATIVE: i32 = 53;
pub const R_M32R_GOTOFF: i32 = 54;
pub const R_M32R_GOTPC24: i32 = 55;
pub const R_M32R_GOT16_HI_ULO: i32 = 56;
pub const R_M32R_GOT16_HI_SLO: i32 = 57;
pub const R_M32R_GOT16_LO: i32 = 58;
pub const R_M32R_GOTPC_HI_ULO: i32 = 59;
pub const R_M32R_GOTPC_HI_SLO: i32 = 60;
pub const R_M32R_GOTPC_LO: i32 = 61;
pub const R_M32R_GOTOFF_HI_ULO: i32 = 62;
pub const R_M32R_GOTOFF_HI_SLO: i32 = 63;
pub const R_M32R_GOTOFF_LO: i32 = 64;
pub const R_M32R_NUM: i32 = 256;
pub const R_MICROBLAZE_NONE: i32 = 0;
pub const R_MICROBLAZE_32: i32 = 1;
pub const R_MICROBLAZE_32_PCREL: i32 = 2;
pub const R_MICROBLAZE_64_PCREL: i32 = 3;
pub const R_MICROBLAZE_32_PCREL_LO: i32 = 4;
pub const R_MICROBLAZE_64: i32 = 5;
pub const R_MICROBLAZE_32_LO: i32 = 6;
pub const R_MICROBLAZE_SRO32: i32 = 7;
pub const R_MICROBLAZE_SRW32: i32 = 8;
pub const R_MICROBLAZE_64_NONE: i32 = 9;
pub const R_MICROBLAZE_32_SYM_OP_SYM: i32 = 10;
pub const R_MICROBLAZE_GNU_VTINHERIT: i32 = 11;
pub const R_MICROBLAZE_GNU_VTENTRY: i32 = 12;
pub const R_MICROBLAZE_GOTPC_64: i32 = 13;
pub const R_MICROBLAZE_GOT_64: i32 = 14;
pub const R_MICROBLAZE_PLT_64: i32 = 15;
pub const R_MICROBLAZE_REL: i32 = 16;
pub const R_MICROBLAZE_JUMP_SLOT: i32 = 17;
pub const R_MICROBLAZE_GLOB_DAT: i32 = 18;
pub const R_MICROBLAZE_GOTOFF_64: i32 = 19;
pub const R_MICROBLAZE_GOTOFF_32: i32 = 20;
pub const R_MICROBLAZE_COPY: i32 = 21;
pub const R_MICROBLAZE_TLS: i32 = 22;
pub const R_MICROBLAZE_TLSGD: i32 = 23;
pub const R_MICROBLAZE_TLSLD: i32 = 24;
pub const R_MICROBLAZE_TLSDTPMOD32: i32 = 25;
pub const R_MICROBLAZE_TLSDTPREL32: i32 = 26;
pub const R_MICROBLAZE_TLSDTPREL64: i32 = 27;
pub const R_MICROBLAZE_TLSGOTTPREL32: i32 = 28;
pub const R_MICROBLAZE_TLSTPREL32: i32 = 29;
pub const DT_NIOS2_GP: i32 = 1879048194;
pub const R_NIOS2_NONE: i32 = 0;
pub const R_NIOS2_S16: i32 = 1;
pub const R_NIOS2_U16: i32 = 2;
pub const R_NIOS2_PCREL16: i32 = 3;
pub const R_NIOS2_CALL26: i32 = 4;
pub const R_NIOS2_IMM5: i32 = 5;
pub const R_NIOS2_CACHE_OPX: i32 = 6;
pub const R_NIOS2_IMM6: i32 = 7;
pub const R_NIOS2_IMM8: i32 = 8;
pub const R_NIOS2_HI16: i32 = 9;
pub const R_NIOS2_LO16: i32 = 10;
pub const R_NIOS2_HIADJ16: i32 = 11;
pub const R_NIOS2_BFD_RELOC_32: i32 = 12;
pub const R_NIOS2_BFD_RELOC_16: i32 = 13;
pub const R_NIOS2_BFD_RELOC_8: i32 = 14;
pub const R_NIOS2_GPREL: i32 = 15;
pub const R_NIOS2_GNU_VTINHERIT: i32 = 16;
pub const R_NIOS2_GNU_VTENTRY: i32 = 17;
pub const R_NIOS2_UJMP: i32 = 18;
pub const R_NIOS2_CJMP: i32 = 19;
pub const R_NIOS2_CALLR: i32 = 20;
pub const R_NIOS2_ALIGN: i32 = 21;
pub const R_NIOS2_GOT16: i32 = 22;
pub const R_NIOS2_CALL16: i32 = 23;
pub const R_NIOS2_GOTOFF_LO: i32 = 24;
pub const R_NIOS2_GOTOFF_HA: i32 = 25;
pub const R_NIOS2_PCREL_LO: i32 = 26;
pub const R_NIOS2_PCREL_HA: i32 = 27;
pub const R_NIOS2_TLS_GD16: i32 = 28;
pub const R_NIOS2_TLS_LDM16: i32 = 29;
pub const R_NIOS2_TLS_LDO16: i32 = 30;
pub const R_NIOS2_TLS_IE16: i32 = 31;
pub const R_NIOS2_TLS_LE16: i32 = 32;
pub const R_NIOS2_TLS_DTPMOD: i32 = 33;
pub const R_NIOS2_TLS_DTPREL: i32 = 34;
pub const R_NIOS2_TLS_TPREL: i32 = 35;
pub const R_NIOS2_COPY: i32 = 36;
pub const R_NIOS2_GLOB_DAT: i32 = 37;
pub const R_NIOS2_JUMP_SLOT: i32 = 38;
pub const R_NIOS2_RELATIVE: i32 = 39;
pub const R_NIOS2_GOTOFF: i32 = 40;
pub const R_NIOS2_CALL26_NOAT: i32 = 41;
pub const R_NIOS2_GOT_LO: i32 = 42;
pub const R_NIOS2_GOT_HA: i32 = 43;
pub const R_NIOS2_CALL_LO: i32 = 44;
pub const R_NIOS2_CALL_HA: i32 = 45;
pub const R_OR1K_NONE: i32 = 0;
pub const R_OR1K_32: i32 = 1;
pub const R_OR1K_16: i32 = 2;
pub const R_OR1K_8: i32 = 3;
pub const R_OR1K_LO_16_IN_INSN: i32 = 4;
pub const R_OR1K_HI_16_IN_INSN: i32 = 5;
pub const R_OR1K_INSN_REL_26: i32 = 6;
pub const R_OR1K_GNU_VTENTRY: i32 = 7;
pub const R_OR1K_GNU_VTINHERIT: i32 = 8;
pub const R_OR1K_32_PCREL: i32 = 9;
pub const R_OR1K_16_PCREL: i32 = 10;
pub const R_OR1K_8_PCREL: i32 = 11;
pub const R_OR1K_GOTPC_HI16: i32 = 12;
pub const R_OR1K_GOTPC_LO16: i32 = 13;
pub const R_OR1K_GOT16: i32 = 14;
pub const R_OR1K_PLT26: i32 = 15;
pub const R_OR1K_GOTOFF_HI16: i32 = 16;
pub const R_OR1K_GOTOFF_LO16: i32 = 17;
pub const R_OR1K_COPY: i32 = 18;
pub const R_OR1K_GLOB_DAT: i32 = 19;
pub const R_OR1K_JMP_SLOT: i32 = 20;
pub const R_OR1K_RELATIVE: i32 = 21;
pub const R_OR1K_TLS_GD_HI16: i32 = 22;
pub const R_OR1K_TLS_GD_LO16: i32 = 23;
pub const R_OR1K_TLS_LDM_HI16: i32 = 24;
pub const R_OR1K_TLS_LDM_LO16: i32 = 25;
pub const R_OR1K_TLS_LDO_HI16: i32 = 26;
pub const R_OR1K_TLS_LDO_LO16: i32 = 27;
pub const R_OR1K_TLS_IE_HI16: i32 = 28;
pub const R_OR1K_TLS_IE_LO16: i32 = 29;
pub const R_OR1K_TLS_LE_HI16: i32 = 30;
pub const R_OR1K_TLS_LE_LO16: i32 = 31;
pub const R_OR1K_TLS_TPOFF: i32 = 32;
pub const R_OR1K_TLS_DTPOFF: i32 = 33;
pub const R_OR1K_TLS_DTPMOD: i32 = 34;
pub const R_BPF_NONE: i32 = 0;
pub const R_BPF_MAP_FD: i32 = 1;
pub const WRDE_DOOFFS: i32 = 1;
pub const WRDE_APPEND: i32 = 2;
pub const WRDE_NOCMD: i32 = 4;
pub const WRDE_REUSE: i32 = 8;
pub const WRDE_SHOWERR: i32 = 16;
pub const WRDE_UNDEF: i32 = 32;
pub const FNM_NOMATCH: i32 = 1;
pub const FNM_NOESCAPE: i32 = 1;
pub const FNM_PATHNAME: i32 = 2;
pub const FNM_PERIOD: i32 = 4;
pub const S_BLKSIZE: i32 = 1024;
pub const S_ISUID: u32 = 2048;
pub const S_ISGID: u32 = 1024;
pub const S_ISVTX: u32 = 512;
pub const S_IREAD: u32 = 256;
pub const S_IWRITE: u32 = 128;
pub const S_IEXEC: u32 = 64;
pub const S_ENFMT: i32 = 1024;
pub const S_IFMT: u32 = 61440;
pub const S_IFDIR: u32 = 16384;
pub const S_IFCHR: u32 = 8192;
pub const S_IFBLK: u32 = 24576;
pub const S_IFREG: u32 = 32768;
pub const S_IFLNK: u32 = 40960;
pub const S_IFSOCK: u32 = 49152;
pub const S_IFIFO: u32 = 4096;
pub const S_IRUSR: u32 = 256;
pub const S_IWUSR: u32 = 128;
pub const S_IXUSR: u32 = 64;
pub const S_IRGRP: u32 = 32;
pub const S_IWGRP: u32 = 16;
pub const S_IXGRP: u32 = 8;
pub const S_IROTH: u32 = 4;
pub const S_IWOTH: u32 = 2;
pub const S_IXOTH: u32 = 1;
pub const DEFFILEMODE: i32 = 438;
pub const ST_RDONLY: i32 = 1;
pub const ST_NOSUID: i32 = 2;
pub const DST_NONE: i32 = 0;
pub const DST_USA: i32 = 1;
pub const DST_AUST: i32 = 2;
pub const DST_WET: i32 = 3;
pub const DST_MET: i32 = 4;
pub const DST_EET: i32 = 5;
pub const DST_CAN: i32 = 6;
pub const SBT_MAX: u64 = 9223372036854775807;
pub const ITIMER_REAL: i32 = 0;
pub const ITIMER_VIRTUAL: i32 = 1;
pub const ITIMER_PROF: i32 = 2;
pub const CHILD_MAX: i32 = 40;
pub const OPEN_MAX: i32 = 64;
pub const IOV_MAX: i32 = 1024;
pub const DT_UNKNOWN: i32 = 0;
pub const DT_FIFO: i32 = 1;
pub const DT_CHR: i32 = 2;
pub const DT_DIR: i32 = 4;
pub const DT_BLK: i32 = 6;
pub const DT_REG: i32 = 8;
pub const DT_LNK: i32 = 10;
pub const DT_SOCK: i32 = 12;
pub const DT_WHT: i32 = 14;
pub const MAXNAMLEN: i32 = 1024;
pub const LC_ALL: i32 = 0;
pub const LC_COLLATE: i32 = 1;
pub const LC_CTYPE: i32 = 2;
pub const LC_MONETARY: i32 = 3;
pub const LC_NUMERIC: i32 = 4;
pub const LC_TIME: i32 = 5;
pub const LC_MESSAGES: i32 = 6;
pub const LC_ALL_MASK: i32 = 1;
pub const LC_COLLATE_MASK: i32 = 2;
pub const LC_CTYPE_MASK: i32 = 4;
pub const LC_MONETARY_MASK: i32 = 8;
pub const LC_NUMERIC_MASK: i32 = 16;
pub const LC_TIME_MASK: i32 = 32;
pub const LC_MESSAGES_MASK: i32 = 64;
pub const REG_BASIC: i32 = 0;
pub const REG_EXTENDED: i32 = 1;
pub const REG_ICASE: i32 = 2;
pub const REG_NOSUB: i32 = 4;
pub const REG_NEWLINE: i32 = 8;
pub const REG_NOSPEC: i32 = 16;
pub const REG_PEND: i32 = 32;
pub const REG_DUMP: i32 = 128;
pub const REG_NOMATCH: i32 = 1;
pub const REG_BADPAT: i32 = 2;
pub const REG_ECOLLATE: i32 = 3;
pub const REG_ECTYPE: i32 = 4;
pub const REG_EESCAPE: i32 = 5;
pub const REG_ESUBREG: i32 = 6;
pub const REG_EBRACK: i32 = 7;
pub const REG_EPAREN: i32 = 8;
pub const REG_EBRACE: i32 = 9;
pub const REG_BADBR: i32 = 10;
pub const REG_ERANGE: i32 = 11;
pub const REG_ESPACE: i32 = 12;
pub const REG_BADRPT: i32 = 13;
pub const REG_EMPTY: i32 = 14;
pub const REG_ASSERT: i32 = 15;
pub const REG_INVARG: i32 = 16;
pub const REG_ATOI: i32 = 255;
pub const REG_ITOA: i32 = 256;
pub const REG_NOTBOL: i32 = 1;
pub const REG_NOTEOL: i32 = 2;
pub const REG_STARTEND: i32 = 4;
pub const REG_TRACE: i32 = 256;
pub const REG_LARGE: i32 = 512;
pub const REG_BACKR: i32 = 1024;
pub const TMAGIC: &'static [u8; 6usize] = b"ustar\0";
pub const TMAGLEN: i32 = 6;
pub const TVERSION: &'static [u8; 3usize] = b"00\0";
pub const TVERSLEN: i32 = 2;
pub const REGTYPE: u8 = 48u8;
pub const AREGTYPE: u8 = 0u8;
pub const LNKTYPE: u8 = 49u8;
pub const SYMTYPE: u8 = 50u8;
pub const CHRTYPE: u8 = 51u8;
pub const BLKTYPE: u8 = 52u8;
pub const DIRTYPE: u8 = 53u8;
pub const FIFOTYPE: u8 = 54u8;
pub const CONTTYPE: u8 = 55u8;
pub const TSUID: i32 = 2048;
pub const TSGID: i32 = 1024;
pub const TUREAD: i32 = 256;
pub const TUWRITE: i32 = 128;
pub const TUEXEC: i32 = 64;
pub const TGREAD: i32 = 32;
pub const TGWRITE: i32 = 16;
pub const TGEXEC: i32 = 8;
pub const TOREAD: i32 = 4;
pub const TOWRITE: i32 = 2;
pub const TOEXEC: i32 = 1;
pub const EXIT_FAILURE: i32 = 1;
pub const EXIT_SUCCESS: i32 = 0;
pub const RAND_MAX: i32 = 2147483647;
pub const OARMAG1: i32 = 65389;
pub const OARMAG2: i32 = 65381;
pub const ARMAG: &'static [u8; 9usize] = b"!<arch>\n\0";
pub const SARMAG: i32 = 8;
pub const AR_EFMT1: &'static [u8; 4usize] = b"#1/\0";
pub const ARFMAG: &'static [u8; 3usize] = b"`\n\0";
pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const ENOMSG: i32 = 35;
pub const EIDRM: i32 = 36;
pub const EDEADLK: i32 = 45;
pub const ENOLCK: i32 = 46;
pub const ENOSTR: i32 = 60;
pub const ENODATA: i32 = 61;
pub const ETIME: i32 = 62;
pub const ENOSR: i32 = 63;
pub const ENOLINK: i32 = 67;
pub const EPROTO: i32 = 71;
pub const EMULTIHOP: i32 = 74;
pub const EBADMSG: i32 = 77;
pub const EFTYPE: i32 = 79;
pub const ENOSYS: i32 = 88;
pub const ENOTEMPTY: i32 = 90;
pub const ENAMETOOLONG: i32 = 91;
pub const ELOOP: i32 = 92;
pub const EOPNOTSUPP: i32 = 95;
pub const EPFNOSUPPORT: i32 = 96;
pub const ECONNRESET: i32 = 104;
pub const ENOBUFS: i32 = 105;
pub const EAFNOSUPPORT: i32 = 106;
pub const EPROTOTYPE: i32 = 107;
pub const ENOTSOCK: i32 = 108;
pub const ENOPROTOOPT: i32 = 109;
pub const ECONNREFUSED: i32 = 111;
pub const EADDRINUSE: i32 = 112;
pub const ECONNABORTED: i32 = 113;
pub const ENETUNREACH: i32 = 114;
pub const ENETDOWN: i32 = 115;
pub const ETIMEDOUT: i32 = 116;
pub const EHOSTDOWN: i32 = 117;
pub const EHOSTUNREACH: i32 = 118;
pub const EINPROGRESS: i32 = 119;
pub const EALREADY: i32 = 120;
pub const EDESTADDRREQ: i32 = 121;
pub const EMSGSIZE: i32 = 122;
pub const EPROTONOSUPPORT: i32 = 123;
pub const EADDRNOTAVAIL: i32 = 125;
pub const ENETRESET: i32 = 126;
pub const EISCONN: i32 = 127;
pub const ENOTCONN: i32 = 128;
pub const ETOOMANYREFS: i32 = 129;
pub const EDQUOT: i32 = 132;
pub const ESTALE: i32 = 133;
pub const ENOTSUP: i32 = 134;
pub const EILSEQ: i32 = 138;
pub const EOVERFLOW: i32 = 139;
pub const ECANCELED: i32 = 140;
pub const ENOTRECOVERABLE: i32 = 141;
pub const EOWNERDEAD: i32 = 142;
pub const EWOULDBLOCK: i32 = 11;
pub const GLOB_APPEND: i32 = 1;
pub const GLOB_DOOFFS: i32 = 2;
pub const GLOB_ERR: i32 = 4;
pub const GLOB_MARK: i32 = 8;
pub const GLOB_NOCHECK: i32 = 16;
pub const GLOB_NOSORT: i32 = 32;
pub const GLOB_ALTDIRFUNC: i32 = 64;
pub const GLOB_BRACE: i32 = 128;
pub const GLOB_MAGCHAR: i32 = 256;
pub const GLOB_NOMAGIC: i32 = 512;
pub const GLOB_QUOTE: i32 = 1024;
pub const GLOB_TILDE: i32 = 2048;
pub const GLOB_LIMIT: i32 = 4096;
pub const GLOB_MAXPATH: i32 = 4096;
pub const GLOB_NOSPACE: i32 = -1;
pub const GLOB_ABEND: i32 = -2;
pub const no_argument: i32 = 0;
pub const required_argument: i32 = 1;
pub const optional_argument: i32 = 2;
pub const EOF: i32 = -1;
pub const BUFSIZ: i32 = 1024;
pub const FOPEN_MAX: i32 = 20;
pub const FILENAME_MAX: i32 = 1024;
pub const L_tmpnam: i32 = 1024;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const TMP_MAX: i32 = 26;
pub const L_ctermid: i32 = 16;
pub const FSETLOCKING_QUERY: i32 = 0;
pub const FSETLOCKING_INTERNAL: i32 = 1;
pub const FSETLOCKING_BYCALLER: i32 = 2;
pub const CTL_MAXNAME: i32 = 24;
pub const CTLTYPE: i32 = 15;
pub const CTLTYPE_NODE: i32 = 1;
pub const CTLTYPE_INT: i32 = 2;
pub const CTLTYPE_STRING: i32 = 3;
pub const CTLTYPE_S64: i32 = 4;
pub const CTLTYPE_OPAQUE: i32 = 5;
pub const CTLTYPE_STRUCT: i32 = 5;
pub const CTLTYPE_UINT: i32 = 6;
pub const CTLTYPE_LONG: i32 = 7;
pub const CTLTYPE_ULONG: i32 = 8;
pub const CTLTYPE_U64: i32 = 9;
pub const CTLTYPE_U8: i32 = 10;
pub const CTLTYPE_U16: i32 = 11;
pub const CTLTYPE_S8: i32 = 12;
pub const CTLTYPE_S16: i32 = 13;
pub const CTLTYPE_S32: i32 = 14;
pub const CTLTYPE_U32: i32 = 15;
pub const CTLFLAG_RD: i32 = 2147483648;
pub const CTLFLAG_WR: i32 = 1073741824;
pub const CTLFLAG_RW: i32 = 3221225472;
pub const CTLFLAG_DORMANT: i32 = 536870912;
pub const CTLFLAG_ANYBODY: i32 = 268435456;
pub const CTLFLAG_SECURE: i32 = 134217728;
pub const CTLFLAG_PRISON: i32 = 67108864;
pub const CTLFLAG_DYN: i32 = 33554432;
pub const CTLFLAG_SKIP: i32 = 16777216;
pub const CTLMASK_SECURE: i32 = 15728640;
pub const CTLFLAG_TUN: i32 = 524288;
pub const CTLFLAG_RDTUN: i32 = 2148007936;
pub const CTLFLAG_RWTUN: i32 = 3221749760;
pub const CTLFLAG_MPSAFE: i32 = 262144;
pub const CTLFLAG_VNET: i32 = 131072;
pub const CTLFLAG_DYING: i32 = 65536;
pub const CTLFLAG_CAPRD: i32 = 32768;
pub const CTLFLAG_CAPWR: i32 = 16384;
pub const CTLFLAG_STATS: i32 = 8192;
pub const CTLFLAG_NOFETCH: i32 = 4096;
pub const CTLFLAG_CAPRW: i32 = 49152;
pub const CTLSHIFT_SECURE: i32 = 20;
pub const CTLFLAG_SECURE1: i32 = 134217728;
pub const CTLFLAG_SECURE2: i32 = 135266304;
pub const CTLFLAG_SECURE3: i32 = 136314880;
pub const OID_AUTO: i32 = -1;
pub const CTL_AUTO_START: i32 = 256;
pub const CTL_UNSPEC: i32 = 0;
pub const CTL_KERN: i32 = 1;
pub const CTL_VM: i32 = 2;
pub const CTL_VFS: i32 = 3;
pub const CTL_NET: i32 = 4;
pub const CTL_DEBUG: i32 = 5;
pub const CTL_HW: i32 = 6;
pub const CTL_MACHDEP: i32 = 7;
pub const CTL_USER: i32 = 8;
pub const CTL_P1003_1B: i32 = 9;
pub const KERN_OSTYPE: i32 = 1;
pub const KERN_OSRELEASE: i32 = 2;
pub const KERN_OSREV: i32 = 3;
pub const KERN_VERSION: i32 = 4;
pub const KERN_MAXVNODES: i32 = 5;
pub const KERN_MAXPROC: i32 = 6;
pub const KERN_MAXFILES: i32 = 7;
pub const KERN_ARGMAX: i32 = 8;
pub const KERN_SECURELVL: i32 = 9;
pub const KERN_HOSTNAME: i32 = 10;
pub const KERN_HOSTID: i32 = 11;
pub const KERN_CLOCKRATE: i32 = 12;
pub const KERN_VNODE: i32 = 13;
pub const KERN_PROC: i32 = 14;
pub const KERN_FILE: i32 = 15;
pub const KERN_PROF: i32 = 16;
pub const KERN_POSIX1: i32 = 17;
pub const KERN_NGROUPS: i32 = 18;
pub const KERN_JOB_CONTROL: i32 = 19;
pub const KERN_SAVED_IDS: i32 = 20;
pub const KERN_BOOTTIME: i32 = 21;
pub const KERN_NISDOMAINNAME: i32 = 22;
pub const KERN_UPDATEINTERVAL: i32 = 23;
pub const KERN_OSRELDATE: i32 = 24;
pub const KERN_NTP_PLL: i32 = 25;
pub const KERN_BOOTFILE: i32 = 26;
pub const KERN_MAXFILESPERPROC: i32 = 27;
pub const KERN_MAXPROCPERUID: i32 = 28;
pub const KERN_DUMPDEV: i32 = 29;
pub const KERN_IPC: i32 = 30;
pub const KERN_DUMMY: i32 = 31;
pub const KERN_PS_STRINGS: i32 = 32;
pub const KERN_USRSTACK: i32 = 33;
pub const KERN_LOGSIGEXIT: i32 = 34;
pub const KERN_IOV_MAX: i32 = 35;
pub const KERN_HOSTUUID: i32 = 36;
pub const KERN_ARND: i32 = 37;
pub const KERN_MAXPHYS: i32 = 38;
pub const KERN_PROC_ALL: i32 = 0;
pub const KERN_PROC_PID: i32 = 1;
pub const KERN_PROC_PGRP: i32 = 2;
pub const KERN_PROC_SESSION: i32 = 3;
pub const KERN_PROC_TTY: i32 = 4;
pub const KERN_PROC_UID: i32 = 5;
pub const KERN_PROC_RUID: i32 = 6;
pub const KERN_PROC_ARGS: i32 = 7;
pub const KERN_PROC_PROC: i32 = 8;
pub const KERN_PROC_SV_NAME: i32 = 9;
pub const KERN_PROC_RGID: i32 = 10;
pub const KERN_PROC_GID: i32 = 11;
pub const KERN_PROC_PATHNAME: i32 = 12;
pub const KERN_PROC_OVMMAP: i32 = 13;
pub const KERN_PROC_OFILEDESC: i32 = 14;
pub const KERN_PROC_KSTACK: i32 = 15;
pub const KERN_PROC_INC_THREAD: i32 = 16;
pub const KERN_PROC_VMMAP: i32 = 32;
pub const KERN_PROC_FILEDESC: i32 = 33;
pub const KERN_PROC_GROUPS: i32 = 34;
pub const KERN_PROC_ENV: i32 = 35;
pub const KERN_PROC_AUXV: i32 = 36;
pub const KERN_PROC_RLIMIT: i32 = 37;
pub const KERN_PROC_PS_STRINGS: i32 = 38;
pub const KERN_PROC_UMASK: i32 = 39;
pub const KERN_PROC_OSREL: i32 = 40;
pub const KERN_PROC_SIGTRAMP: i32 = 41;
pub const KERN_PROC_CWD: i32 = 42;
pub const KERN_PROC_NFDS: i32 = 43;
pub const KIPC_MAXSOCKBUF: i32 = 1;
pub const KIPC_SOCKBUF_WASTE: i32 = 2;
pub const KIPC_SOMAXCONN: i32 = 3;
pub const KIPC_MAX_LINKHDR: i32 = 4;
pub const KIPC_MAX_PROTOHDR: i32 = 5;
pub const KIPC_MAX_HDR: i32 = 6;
pub const KIPC_MAX_DATALEN: i32 = 7;
pub const HW_MACHINE: i32 = 1;
pub const HW_MODEL: i32 = 2;
pub const HW_NCPU: i32 = 3;
pub const HW_BYTEORDER: i32 = 4;
pub const HW_PHYSMEM: i32 = 5;
pub const HW_USERMEM: i32 = 6;
pub const HW_PAGESIZE: i32 = 7;
pub const HW_DISKNAMES: i32 = 8;
pub const HW_DISKSTATS: i32 = 9;
pub const HW_FLOATINGPT: i32 = 10;
pub const HW_MACHINE_ARCH: i32 = 11;
pub const HW_REALMEM: i32 = 12;
pub const USER_CS_PATH: i32 = 1;
pub const USER_BC_BASE_MAX: i32 = 2;
pub const USER_BC_DIM_MAX: i32 = 3;
pub const USER_BC_SCALE_MAX: i32 = 4;
pub const USER_BC_STRING_MAX: i32 = 5;
pub const USER_COLL_WEIGHTS_MAX: i32 = 6;
pub const USER_EXPR_NEST_MAX: i32 = 7;
pub const USER_LINE_MAX: i32 = 8;
pub const USER_RE_DUP_MAX: i32 = 9;
pub const USER_POSIX2_VERSION: i32 = 10;
pub const USER_POSIX2_C_BIND: i32 = 11;
pub const USER_POSIX2_C_DEV: i32 = 12;
pub const USER_POSIX2_CHAR_TERM: i32 = 13;
pub const USER_POSIX2_FORT_DEV: i32 = 14;
pub const USER_POSIX2_FORT_RUN: i32 = 15;
pub const USER_POSIX2_LOCALEDEF: i32 = 16;
pub const USER_POSIX2_SW_DEV: i32 = 17;
pub const USER_POSIX2_UPE: i32 = 18;
pub const USER_STREAM_MAX: i32 = 19;
pub const USER_TZNAME_MAX: i32 = 20;
pub const CTL_P1003_1B_ASYNCHRONOUS_IO: i32 = 1;
pub const CTL_P1003_1B_MAPPED_FILES: i32 = 2;
pub const CTL_P1003_1B_MEMLOCK: i32 = 3;
pub const CTL_P1003_1B_MEMLOCK_RANGE: i32 = 4;
pub const CTL_P1003_1B_MEMORY_PROTECTION: i32 = 5;
pub const CTL_P1003_1B_MESSAGE_PASSING: i32 = 6;
pub const CTL_P1003_1B_PRIORITIZED_IO: i32 = 7;
pub const CTL_P1003_1B_PRIORITY_SCHEDULING: i32 = 8;
pub const CTL_P1003_1B_REALTIME_SIGNALS: i32 = 9;
pub const CTL_P1003_1B_SEMAPHORES: i32 = 10;
pub const CTL_P1003_1B_FSYNC: i32 = 11;
pub const CTL_P1003_1B_SHARED_MEMORY_OBJECTS: i32 = 12;
pub const CTL_P1003_1B_SYNCHRONIZED_IO: i32 = 13;
pub const CTL_P1003_1B_TIMERS: i32 = 14;
pub const CTL_P1003_1B_AIO_LISTIO_MAX: i32 = 15;
pub const CTL_P1003_1B_AIO_MAX: i32 = 16;
pub const CTL_P1003_1B_AIO_PRIO_DELTA_MAX: i32 = 17;
pub const CTL_P1003_1B_DELAYTIMER_MAX: i32 = 18;
pub const CTL_P1003_1B_MQ_OPEN_MAX: i32 = 19;
pub const CTL_P1003_1B_PAGESIZE: i32 = 20;
pub const CTL_P1003_1B_RTSIG_MAX: i32 = 21;
pub const CTL_P1003_1B_SEM_NSEMS_MAX: i32 = 22;
pub const CTL_P1003_1B_SEM_VALUE_MAX: i32 = 23;
pub const CTL_P1003_1B_SIGQUEUE_MAX: i32 = 24;
pub const CTL_P1003_1B_TIMER_MAX: i32 = 25;
pub const CTL_P1003_1B_MAXID: i32 = 26;
pub const WNOHANG: i32 = 1;
pub const WUNTRACED: i32 = 2;
pub const SPLAY_NEGINF: i32 = -1;
pub const SPLAY_INF: i32 = 1;
pub const RB_BLACK: i32 = 0;
pub const RB_RED: i32 = 1;
pub const RB_NEGINF: i32 = -1;
pub const RB_INF: i32 = 1;
pub const IOCPARM_SHIFT: i32 = 13;
pub const IOCPARM_MASK: i32 = 8191;
pub const IOCPARM_MAX: i32 = 8192;
pub const IOC_VOID: i32 = 536870912;
pub const IOC_OUT: i32 = 1073741824;
pub const IOC_IN: i32 = 2147483648;
pub const IOC_INOUT: i32 = 3221225472;
pub const IOC_DIRMASK: i32 = 3758096384;
pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_RDWR: i32 = 2;
pub const O_ACCMODE: i32 = 3;
pub const O_APPEND: i32 = 8;
pub const O_CREAT: i32 = 512;
pub const O_TRUNC: i32 = 1024;
pub const O_EXCL: i32 = 2048;
pub const O_SYNC: i32 = 8192;
pub const O_NONBLOCK: i32 = 16384;
pub const O_NOCTTY: i32 = 32768;
pub const O_CLOEXEC: i32 = 0; // not supported (but exec isn't either)
pub const O_NOFOLLOW: i32 = 1048576;
pub const O_DIRECTORY: i32 = 2097152;
pub const O_EXEC: i32 = 4194304;
pub const O_SEARCH: i32 = 4194304;
pub const O_DIRECT: i32 = 524288;
pub const FAPPEND: i32 = 8;
pub const FSYNC: i32 = 8192;
pub const FASYNC: i32 = 64;
pub const FNBIO: i32 = 4096;
pub const FNONBIO: i32 = 16384;
pub const FNDELAY: i32 = 16384;
pub const FREAD: i32 = 1;
pub const FWRITE: i32 = 2;
pub const FMARK: i32 = 16;
pub const FDEFER: i32 = 32;
pub const FSHLOCK: i32 = 128;
pub const FEXLOCK: i32 = 256;
pub const FOPEN: i32 = -1;
pub const FCREAT: i32 = 512;
pub const FTRUNC: i32 = 1024;
pub const FEXCL: i32 = 2048;
pub const FNOCTTY: i32 = 32768;
pub const FNONBLOCK: i32 = 16384;
pub const FD_CLOEXEC: i32 = 1;
pub const F_DUPFD: i32 = 0;
pub const F_GETFD: i32 = 1;
pub const F_SETFD: i32 = 2;
pub const F_GETFL: i32 = 3;
pub const F_SETFL: i32 = 4;
pub const F_GETOWN: i32 = 5;
pub const F_SETOWN: i32 = 6;
pub const F_GETLK: i32 = 7;
pub const F_SETLK: i32 = 8;
pub const F_SETLKW: i32 = 9;
pub const F_RGETLK: i32 = 10;
pub const F_RSETLK: i32 = 11;
pub const F_CNVT: i32 = 12;
pub const F_RSETLKW: i32 = 13;
pub const F_DUPFD_CLOEXEC: i32 = 14;
pub const F_RDLCK: i32 = 1;
pub const F_WRLCK: i32 = 2;
pub const F_UNLCK: i32 = 3;
pub const F_UNLKSYS: i32 = 4;
pub const AT_FDCWD: i32 = -2;
pub const AT_EACCESS: i32 = 1;
pub const AT_SYMLINK_NOFOLLOW: i32 = 2;
pub const AT_SYMLINK_FOLLOW: i32 = 4;
pub const AT_REMOVEDIR: i32 = 8;
pub const LOCK_SH: i32 = 1;
pub const LOCK_EX: i32 = 2;
pub const LOCK_NB: i32 = 4;
pub const LOCK_UN: i32 = 8;
pub const NLS_ENVVAR_NAME: &'static [u8; 8usize] = b"NLSPATH\0";
pub const ICONV_DEFAULT_NLSPATH: &'static [u8; 12usize] = b"/usr/locale\0";
pub const ICONV_NLS_FROM: i32 = 0;
pub const ICONV_NLS_TO: i32 = 1;
pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_RDM: i32 = 4;
pub const SOCK_SEQPACKET: i32 = 5;
pub const SOCK_CLOEXEC: i32 = 268435456;
pub const SOCK_NONBLOCK: i32 = 536870912;
pub const SO_DEBUG: i32 = 1;
pub const SO_ACCEPTCONN: i32 = 2;
pub const SO_REUSEADDR: i32 = 4;
pub const SO_KEEPALIVE: i32 = 8;
pub const SO_DONTROUTE: i32 = 16;
pub const SO_BROADCAST: i32 = 32;
pub const SO_USELOOPBACK: i32 = 64;
pub const SO_LINGER: i32 = 128;
pub const SO_OOBINLINE: i32 = 256;
pub const SO_REUSEPORT: i32 = 512;
pub const SO_TIMESTAMP: i32 = 1024;
pub const SO_NOSIGPIPE: i32 = 2048;
pub const SO_ACCEPTFILTER: i32 = 4096;
pub const SO_BINTIME: i32 = 8192;
pub const SO_NO_OFFLOAD: i32 = 16384;
pub const SO_NO_DDP: i32 = 32768;
pub const SO_SNDBUF: i32 = 4097;
pub const SO_RCVBUF: i32 = 4098;
pub const SO_SNDLOWAT: i32 = 4099;
pub const SO_RCVLOWAT: i32 = 4100;
pub const SO_SNDTIMEO: i32 = 4101;
pub const SO_RCVTIMEO: i32 = 4102;
pub const SO_ERROR: i32 = 4103;
pub const SO_TYPE: i32 = 4104;
pub const SO_LABEL: i32 = 4105;
pub const SO_PEERLABEL: i32 = 4112;
pub const SO_LISTENQLIMIT: i32 = 4113;
pub const SO_LISTENQLEN: i32 = 4114;
pub const SO_LISTENINCQLEN: i32 = 4115;
pub const SO_SETFIB: i32 = 4116;
pub const SO_USER_COOKIE: i32 = 4117;
pub const SO_PROTOCOL: i32 = 4118;
pub const SO_PROTOTYPE: i32 = 4118;
pub const SO_TS_CLOCK: i32 = 4119;
pub const SO_MAX_PACING_RATE: i32 = 4120;
pub const SO_TS_REALTIME_MICRO: i32 = 0;
pub const SO_TS_BINTIME: i32 = 1;
pub const SO_TS_REALTIME: i32 = 2;
pub const SO_TS_MONOTONIC: i32 = 3;
pub const SO_TS_DEFAULT: i32 = 0;
pub const SO_TS_CLOCK_MAX: i32 = 3;
pub const SO_VENDOR: i32 = 2147483648;
pub const SOL_SOCKET: i32 = 65535;
pub const AF_UNSPEC: i32 = 0;
pub const AF_UNIX: i32 = 1;
pub const AF_INET: i32 = 2;
pub const AF_IMPLINK: i32 = 3;
pub const AF_PUP: i32 = 4;
pub const AF_CHAOS: i32 = 5;
pub const AF_NETBIOS: i32 = 6;
pub const AF_ISO: i32 = 7;
pub const AF_OSI: i32 = 7;
pub const AF_ECMA: i32 = 8;
pub const AF_DATAKIT: i32 = 9;
pub const AF_CCITT: i32 = 10;
pub const AF_SNA: i32 = 11;
pub const AF_DECnet: i32 = 12;
pub const AF_DLI: i32 = 13;
pub const AF_LAT: i32 = 14;
pub const AF_HYLINK: i32 = 15;
pub const AF_APPLETALK: i32 = 16;
pub const AF_ROUTE: i32 = 17;
pub const AF_LINK: i32 = 18;
pub const pseudo_AF_XTP: i32 = 19;
pub const AF_COIP: i32 = 20;
pub const AF_CNT: i32 = 21;
pub const pseudo_AF_RTIP: i32 = 22;
pub const AF_IPX: i32 = 23;
pub const AF_SIP: i32 = 24;
pub const pseudo_AF_PIP: i32 = 25;
pub const AF_ISDN: i32 = 26;
pub const AF_E164: i32 = 26;
pub const pseudo_AF_KEY: i32 = 27;
pub const AF_INET6: i32 = 28;
pub const AF_NATM: i32 = 29;
pub const AF_ATM: i32 = 30;
pub const pseudo_AF_HDRCMPLT: i32 = 31;
pub const AF_NETGRAPH: i32 = 32;
pub const AF_SLOW: i32 = 33;
pub const AF_SCLUSTER: i32 = 34;
pub const AF_ARP: i32 = 35;
pub const AF_BLUETOOTH: i32 = 36;
pub const AF_IEEE80211: i32 = 37;
pub const AF_INET_SDP: i32 = 40;
pub const AF_INET6_SDP: i32 = 42;
pub const AF_MAX: i32 = 42;
pub const AF_VENDOR00: i32 = 39;
pub const AF_VENDOR01: i32 = 41;
pub const AF_VENDOR02: i32 = 43;
pub const AF_VENDOR03: i32 = 45;
pub const AF_VENDOR04: i32 = 47;
pub const AF_VENDOR05: i32 = 49;
pub const AF_VENDOR06: i32 = 51;
pub const AF_VENDOR07: i32 = 53;
pub const AF_VENDOR08: i32 = 55;
pub const AF_VENDOR09: i32 = 57;
pub const AF_VENDOR10: i32 = 59;
pub const AF_VENDOR11: i32 = 61;
pub const AF_VENDOR12: i32 = 63;
pub const AF_VENDOR13: i32 = 65;
pub const AF_VENDOR14: i32 = 67;
pub const AF_VENDOR15: i32 = 69;
pub const AF_VENDOR16: i32 = 71;
pub const AF_VENDOR17: i32 = 73;
pub const AF_VENDOR18: i32 = 75;
pub const AF_VENDOR19: i32 = 77;
pub const AF_VENDOR20: i32 = 79;
pub const AF_VENDOR21: i32 = 81;
pub const AF_VENDOR22: i32 = 83;
pub const AF_VENDOR23: i32 = 85;
pub const AF_VENDOR24: i32 = 87;
pub const AF_VENDOR25: i32 = 89;
pub const AF_VENDOR26: i32 = 91;
pub const AF_VENDOR27: i32 = 93;
pub const AF_VENDOR28: i32 = 95;
pub const AF_VENDOR29: i32 = 97;
pub const AF_VENDOR30: i32 = 99;
pub const AF_VENDOR31: i32 = 101;
pub const AF_VENDOR32: i32 = 103;
pub const AF_VENDOR33: i32 = 105;
pub const AF_VENDOR34: i32 = 107;
pub const AF_VENDOR35: i32 = 109;
pub const AF_VENDOR36: i32 = 111;
pub const AF_VENDOR37: i32 = 113;
pub const AF_VENDOR38: i32 = 115;
pub const AF_VENDOR39: i32 = 117;
pub const AF_VENDOR40: i32 = 119;
pub const AF_VENDOR41: i32 = 121;
pub const AF_VENDOR42: i32 = 123;
pub const AF_VENDOR43: i32 = 125;
pub const AF_VENDOR44: i32 = 127;
pub const AF_VENDOR45: i32 = 129;
pub const AF_VENDOR46: i32 = 131;
pub const AF_VENDOR47: i32 = 133;
pub const SOCK_MAXADDRLEN: i32 = 255;
pub const PF_UNSPEC: i32 = 0;
pub const PF_INET: i32 = 2;
pub const PF_IMPLINK: i32 = 3;
pub const PF_PUP: i32 = 4;
pub const PF_CHAOS: i32 = 5;
pub const PF_NETBIOS: i32 = 6;
pub const PF_ISO: i32 = 7;
pub const PF_OSI: i32 = 7;
pub const PF_ECMA: i32 = 8;
pub const PF_DATAKIT: i32 = 9;
pub const PF_CCITT: i32 = 10;
pub const PF_SNA: i32 = 11;
pub const PF_DECnet: i32 = 12;
pub const PF_DLI: i32 = 13;
pub const PF_LAT: i32 = 14;
pub const PF_HYLINK: i32 = 15;
pub const PF_APPLETALK: i32 = 16;
pub const PF_ROUTE: i32 = 17;
pub const PF_LINK: i32 = 18;
pub const PF_XTP: i32 = 19;
pub const PF_COIP: i32 = 20;
pub const PF_CNT: i32 = 21;
pub const PF_SIP: i32 = 24;
pub const PF_IPX: i32 = 23;
pub const PF_RTIP: i32 = 22;
pub const PF_PIP: i32 = 25;
pub const PF_ISDN: i32 = 26;
pub const PF_KEY: i32 = 27;
pub const PF_INET6: i32 = 28;
pub const PF_NATM: i32 = 29;
pub const PF_ATM: i32 = 30;
pub const PF_NETGRAPH: i32 = 32;
pub const PF_SLOW: i32 = 33;
pub const PF_SCLUSTER: i32 = 34;
pub const PF_ARP: i32 = 35;
pub const PF_BLUETOOTH: i32 = 36;
pub const PF_IEEE80211: i32 = 37;
pub const PF_INET_SDP: i32 = 40;
pub const PF_INET6_SDP: i32 = 42;
pub const PF_MAX: i32 = 42;
pub const NET_RT_DUMP: i32 = 1;
pub const NET_RT_FLAGS: i32 = 2;
pub const NET_RT_IFLIST: i32 = 3;
pub const NET_RT_IFMALIST: i32 = 4;
pub const NET_RT_IFLISTL: i32 = 5;
pub const SOMAXCONN: i32 = 128;
pub const MSG_OOB: i32 = 1;
pub const MSG_PEEK: i32 = 2;
pub const MSG_DONTROUTE: i32 = 4;
pub const MSG_EOR: i32 = 8;
pub const MSG_TRUNC: i32 = 16;
pub const MSG_CTRUNC: i32 = 32;
pub const MSG_WAITALL: i32 = 64;
pub const MSG_DONTWAIT: i32 = 128;
pub const MSG_EOF: i32 = 256;
pub const MSG_NOTIFICATION: i32 = 8192;
pub const MSG_NBIO: i32 = 16384;
pub const MSG_COMPAT: i32 = 32768;
pub const MSG_NOSIGNAL: i32 = 131072;
pub const MSG_CMSG_CLOEXEC: i32 = 262144;
pub const MSG_WAITFORONE: i32 = 524288;
pub const SCM_RIGHTS: i32 = 1;
pub const SCM_TIMESTAMP: i32 = 2;
pub const SCM_CREDS: i32 = 3;
pub const SCM_BINTIME: i32 = 4;
pub const SCM_REALTIME: i32 = 5;
pub const SCM_MONOTONIC: i32 = 6;
pub const SCM_TIME_INFO: i32 = 7;
pub const ST_INFO_HW: i32 = 1;
pub const ST_INFO_HW_HPREC: i32 = 2;
pub const SHUT_RD: i32 = 0;
pub const SHUT_WR: i32 = 1;
pub const SHUT_RDWR: i32 = 2;
pub const PRU_FLUSH_RD: i32 = 0;
pub const PRU_FLUSH_WR: i32 = 1;
pub const PRU_FLUSH_RDWR: i32 = 2;
pub const SF_NODISKIO: i32 = 1;
pub const SF_MNOWAIT: i32 = 2;
pub const SF_SYNC: i32 = 4;
pub const SF_USER_READAHEAD: i32 = 8;
pub const SF_NOCACHE: i32 = 16;
pub const RUSAGE_SELF: i32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const SIGEV_NONE: i32 = 1;
pub const SIGEV_SIGNAL: i32 = 2;
pub const SIGEV_THREAD: i32 = 3;
pub const SI_USER: i32 = 1;
pub const SI_QUEUE: i32 = 2;
pub const SI_TIMER: i32 = 3;
pub const SI_ASYNCIO: i32 = 4;
pub const SI_MESGQ: i32 = 5;
pub const SA_NOCLDSTOP: i32 = 1;
pub const MINSIGSTKSZ: i32 = 2048;
pub const SIGSTKSZ: i32 = 8192;
pub const SS_ONSTACK: i32 = 1;
pub const SS_DISABLE: i32 = 2;
pub const SIG_SETMASK: i32 = 0;
pub const SIG_BLOCK: i32 = 1;
pub const SIG_UNBLOCK: i32 = 2;
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGIOT: i32 = 6;
pub const SIGABRT: i32 = 6;
pub const SIGEMT: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGBUS: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGSYS: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGURG: i32 = 16;
pub const SIGSTOP: i32 = 17;
pub const SIGTSTP: i32 = 18;
pub const SIGCONT: i32 = 19;
pub const SIGCHLD: i32 = 20;
pub const SIGCLD: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGIO: i32 = 23;
pub const SIGPOLL: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGLOST: i32 = 29;
pub const SIGUSR1: i32 = 30;
pub const SIGUSR2: i32 = 31;
pub const NSIG: i32 = 32;
pub const POLLIN: i16 = 1;
pub const POLLPRI: i16 = 2;
pub const POLLOUT: i16 = 4;
pub const POLLRDNORM: i16 = 64;
pub const POLLWRNORM: i16 = 4;
pub const POLLRDBAND: i16 = 128;
pub const POLLWRBAND: i16 = 256;
pub const POLLINIGNEOF: i16 = 8192;
pub const POLLERR: i16 = 8;
pub const POLLHUP: i16 = 16;
pub const POLLNVAL: i16 = 32;
pub const POLLSTANDARD: i16 = 511;
pub const INFTIM: i32 = -1;
pub const NBBY: i32 = 8;
pub const HZ: i32 = 60;
pub const NOFILE: i32 = 60;
pub const PATHSIZE: i32 = 1024;
pub const MAXPATHLEN: i32 = 1024;
pub const POSIX_SPAWN_RESETIDS: i32 = 1;
pub const POSIX_SPAWN_SETPGROUP: i32 = 2;
pub const POSIX_SPAWN_SETSCHEDPARAM: i32 = 4;
pub const POSIX_SPAWN_SETSCHEDULER: i32 = 8;
pub const POSIX_SPAWN_SETSIGDEF: i32 = 16;
pub const POSIX_SPAWN_SETSIGMASK: i32 = 32;
pub const C_IRUSR: i32 = 256;
pub const C_IWUSR: i32 = 128;
pub const C_IXUSR: i32 = 64;
pub const C_IRGRP: i32 = 32;
pub const C_IWGRP: i32 = 16;
pub const C_IXGRP: i32 = 8;
pub const C_IROTH: i32 = 4;
pub const C_IWOTH: i32 = 2;
pub const C_IXOTH: i32 = 1;
pub const C_ISUID: i32 = 2048;
pub const C_ISGID: i32 = 1024;
pub const C_ISVTX: i32 = 512;
pub const C_ISDIR: i32 = 16384;
pub const C_ISFIFO: i32 = 4096;
pub const C_ISREG: i32 = 32768;
pub const C_ISBLK: i32 = 24576;
pub const C_ISCHR: i32 = 8192;
pub const C_ISCTG: i32 = 36864;
pub const C_ISLNK: i32 = 40960;
pub const C_ISSOCK: i32 = 49152;
pub const MAGIC: &'static [u8; 7usize] = b"070707\0";
pub const M_MXFAST: i32 = 1;
pub const M_NLBLKS: i32 = 2;
pub const M_GRAIN: i32 = 3;
pub const M_KEEP: i32 = 4;
pub const M_TRIM_THRESHOLD: i32 = -1;
pub const M_TOP_PAD: i32 = -2;
pub const M_MMAP_THRESHOLD: i32 = -3;
pub const M_MMAP_MAX: i32 = -4;
pub const IPPORT_RESERVED: i32 = 1024;
pub const NETDB_INTERNAL: i32 = -1;
pub const NETDB_SUCCESS: i32 = 0;
pub const HOST_NOT_FOUND: i32 = 1;
pub const TRY_AGAIN: i32 = 2;
pub const NO_RECOVERY: i32 = 3;
pub const NO_DATA: i32 = 4;
pub const NO_ADDRESS: i32 = 4;
pub const EAI_ADDRFAMILY: i32 = 1;
pub const EAI_AGAIN: i32 = 2;
pub const EAI_BADFLAGS: i32 = 3;
pub const EAI_FAIL: i32 = 4;
pub const EAI_FAMILY: i32 = 5;
pub const EAI_MEMORY: i32 = 6;
pub const EAI_NODATA: i32 = 7;
pub const EAI_NONAME: i32 = 8;
pub const EAI_SERVICE: i32 = 9;
pub const EAI_SOCKTYPE: i32 = 10;
pub const EAI_SYSTEM: i32 = 11;
pub const EAI_BADHINTS: i32 = 12;
pub const EAI_PROTOCOL: i32 = 13;
pub const EAI_OVERFLOW: i32 = 14;
pub const EAI_MAX: i32 = 15;
pub const AI_PASSIVE: i32 = 1;
pub const AI_CANONNAME: i32 = 2;
pub const AI_NUMERICHOST: i32 = 4;
pub const AI_NUMERICSERV: i32 = 8;
pub const AI_ALL: i32 = 256;
pub const AI_V4MAPPED_CFG: i32 = 512;
pub const AI_ADDRCONFIG: i32 = 1024;
pub const AI_V4MAPPED: i32 = 2048;
pub const AI_DEFAULT: i32 = 1536;
pub const NI_MAXHOST: i32 = 1025;
pub const NI_MAXSERV: i32 = 32;
pub const NI_NOFQDN: i32 = 1;
pub const NI_NUMERICHOST: i32 = 2;
pub const NI_NAMEREQD: i32 = 4;
pub const NI_NUMERICSERV: i32 = 8;
pub const NI_DGRAM: i32 = 16;
pub const NI_NUMERICSCOPE: i32 = 32;
pub const SCOPE_DELIMITER: u8 = 37u8;
pub const __LOCK_INITIALIZER: _LOCK_T = 0;
pub const __COND_INITIALIZER: _COND_T = 0;
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    lock: __LOCK_INITIALIZER,
    cond_r: __COND_INITIALIZER,
    cond_w: __COND_INITIALIZER,
    data: [0; 4],
};
pub const _SC_ARG_MAX: c_int = 0;
pub const _SC_CHILD_MAX: c_int = 1;
pub const _SC_CLK_TCK: c_int = 2;
pub const _SC_NGROUPS_MAX: c_int = 3;
pub const _SC_OPEN_MAX: c_int = 4;
pub const _SC_JOB_CONTROL: c_int = 5;
pub const _SC_SAVED_IDS: c_int = 6;
pub const _SC_VERSION: c_int = 7;
pub const _SC_PAGESIZE: c_int = 8;
pub const _SC_PAGE_SIZE: c_int = 8;
pub const _SC_NPROCESSORS_CONF: c_int = 9;
pub const _SC_NPROCESSORS_ONLN: c_int = 10;
pub const _SC_PHYS_PAGES: c_int = 11;
pub const _SC_AVPHYS_PAGES: c_int = 12;
pub const _SC_MQ_OPEN_MAX: c_int = 13;
pub const _SC_MQ_PRIO_MAX: c_int = 14;
pub const _SC_RTSIG_MAX: c_int = 15;
pub const _SC_SEM_NSEMS_MAX: c_int = 16;
pub const _SC_SEM_VALUE_MAX: c_int = 17;
pub const _SC_SIGQUEUE_MAX: c_int = 18;
pub const _SC_TIMER_MAX: c_int = 19;
pub const _SC_TZNAME_MAX: c_int = 20;
pub const _SC_ASYNCHRONOUS_IO: c_int = 21;
pub const _SC_FSYNC: c_int = 22;
pub const _SC_MAPPED_FILES: c_int = 23;
pub const _SC_MEMLOCK: c_int = 24;
pub const _SC_MEMLOCK_RANGE: c_int = 25;
pub const _SC_MEMORY_PROTECTION: c_int = 26;
pub const _SC_MESSAGE_PASSING: c_int = 27;
pub const _SC_PRIORITIZED_IO: c_int = 28;
pub const _SC_REALTIME_SIGNALS: c_int = 29;
pub const _SC_SEMAPHORES: c_int = 30;
pub const _SC_SHARED_MEMORY_OBJECTS: c_int = 31;
pub const _SC_SYNCHRONIZED_IO: c_int = 32;
pub const _SC_TIMERS: c_int = 33;
pub const _SC_AIO_LISTIO_MAX: c_int = 34;
pub const _SC_AIO_MAX: c_int = 35;
pub const _SC_AIO_PRIO_DELTA_MAX: c_int = 36;
pub const _SC_DELAYTIMER_MAX: c_int = 37;
pub const _SC_THREAD_KEYS_MAX: c_int = 38;
pub const _SC_THREAD_STACK_MIN: c_int = 39;
pub const _SC_THREAD_THREADS_MAX: c_int = 40;
pub const _SC_TTY_NAME_MAX: c_int = 41;
pub const _SC_THREADS: c_int = 42;
pub const _SC_THREAD_ATTR_STACKADDR: c_int = 43;
pub const _SC_THREAD_ATTR_STACKSIZE: c_int = 44;
pub const _SC_THREAD_PRIORITY_SCHEDULING: c_int = 45;
pub const _SC_THREAD_PRIO_INHERIT: c_int = 46;
pub const _SC_THREAD_PRIO_PROTECT: c_int = 47;
pub const _SC_THREAD_PRIO_CEILING: c_int = 47;
pub const _SC_THREAD_PROCESS_SHARED: c_int = 48;
pub const _SC_THREAD_SAFE_FUNCTIONS: c_int = 49;
pub const _SC_GETGR_R_SIZE_MAX: c_int = 50;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 51;
pub const _SC_LOGIN_NAME_MAX: c_int = 52;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: c_int = 53;
pub const _SC_ADVISORY_INFO: c_int = 54;
pub const _SC_ATEXIT_MAX: c_int = 55;
pub const _SC_BARRIERS: c_int = 56;
pub const _SC_BC_BASE_MAX: c_int = 57;
pub const _SC_BC_DIM_MAX: c_int = 58;
pub const _SC_BC_SCALE_MAX: c_int = 59;
pub const _SC_BC_STRING_MAX: c_int = 60;
pub const _SC_CLOCK_SELECTION: c_int = 61;
pub const _SC_COLL_WEIGHTS_MAX: c_int = 62;
pub const _SC_CPUTIME: c_int = 63;
pub const _SC_EXPR_NEST_MAX: c_int = 64;
pub const _SC_HOST_NAME_MAX: c_int = 65;
pub const _SC_IOV_MAX: c_int = 66;
pub const _SC_IPV6: c_int = 67;
pub const _SC_LINE_MAX: c_int = 68;
pub const _SC_MONOTONIC_CLOCK: c_int = 69;
pub const _SC_RAW_SOCKETS: c_int = 70;
pub const _SC_READER_WRITER_LOCKS: c_int = 71;
pub const _SC_REGEXP: c_int = 72;
pub const _SC_RE_DUP_MAX: c_int = 73;
pub const _SC_SHELL: c_int = 74;
pub const _SC_SPAWN: c_int = 75;
pub const _SC_SPIN_LOCKS: c_int = 76;
pub const _SC_SPORADIC_SERVER: c_int = 77;
pub const _SC_SS_REPL_MAX: c_int = 78;
pub const _SC_SYMLOOP_MAX: c_int = 79;
pub const _SC_THREAD_CPUTIME: c_int = 80;
pub const _SC_THREAD_SPORADIC_SERVER: c_int = 81;
pub const _SC_TIMEOUTS: c_int = 82;
pub const _SC_TRACE: c_int = 83;
pub const _SC_TRACE_EVENT_FILTER: c_int = 84;
pub const _SC_TRACE_EVENT_NAME_MAX: c_int = 85;
pub const _SC_TRACE_INHERIT: c_int = 86;
pub const _SC_TRACE_LOG: c_int = 87;
pub const _SC_TRACE_NAME_MAX: c_int = 88;
pub const _SC_TRACE_SYS_MAX: c_int = 89;
pub const _SC_TRACE_USER_EVENT_MAX: c_int = 90;
pub const _SC_TYPED_MEMORY_OBJECTS: c_int = 91;
pub const _SC_V7_ILP32_OFF32: c_int = 92;
pub const _SC_V6_ILP32_OFF32: c_int = 92;
pub const _SC_XBS5_ILP32_OFF32: c_int = 92;
pub const _SC_V7_ILP32_OFFBIG: c_int = 93;
pub const _SC_V6_ILP32_OFFBIG: c_int = 93;
pub const _SC_XBS5_ILP32_OFFBIG: c_int = 93;
pub const _SC_V7_LP64_OFF64: c_int = 94;
pub const _SC_V6_LP64_OFF64: c_int = 94;
pub const _SC_XBS5_LP64_OFF64: c_int = 94;
pub const _SC_V7_LPBIG_OFFBIG: c_int = 95;
pub const _SC_V6_LPBIG_OFFBIG: c_int = 95;
pub const _SC_XBS5_LPBIG_OFFBIG: c_int = 95;
pub const _SC_XOPEN_CRYPT: c_int = 96;
pub const _SC_XOPEN_ENH_I18N: c_int = 97;
pub const _SC_XOPEN_LEGACY: c_int = 98;
pub const _SC_XOPEN_REALTIME: c_int = 99;
pub const _SC_STREAM_MAX: c_int = 100;
pub const _SC_PRIORITY_SCHEDULING: c_int = 101;
pub const _SC_XOPEN_REALTIME_THREADS: c_int = 102;
pub const _SC_XOPEN_SHM: c_int = 103;
pub const _SC_XOPEN_STREAMS: c_int = 104;
pub const _SC_XOPEN_UNIX: c_int = 105;
pub const _SC_XOPEN_VERSION: c_int = 106;
pub const _SC_2_CHAR_TERM: c_int = 107;
pub const _SC_2_C_BIND: c_int = 108;
pub const _SC_2_C_DEV: c_int = 109;
pub const _SC_2_FORT_DEV: c_int = 110;
pub const _SC_2_FORT_RUN: c_int = 111;
pub const _SC_2_LOCALEDEF: c_int = 112;
pub const _SC_2_PBS: c_int = 113;
pub const _SC_2_PBS_ACCOUNTING: c_int = 114;
pub const _SC_2_PBS_CHECKPOINT: c_int = 115;
pub const _SC_2_PBS_LOCATE: c_int = 116;
pub const _SC_2_PBS_MESSAGE: c_int = 117;
pub const _SC_2_PBS_TRACK: c_int = 118;
pub const _SC_2_SW_DEV: c_int = 119;
pub const _SC_2_UPE: c_int = 120;
pub const _SC_2_VERSION: c_int = 121;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: c_int = 122;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: c_int = 123;
pub const _SC_XOPEN_UUCP: c_int = 124;
pub const _SC_LEVEL1_ICACHE_SIZE: c_int = 125;
pub const _SC_LEVEL1_ICACHE_ASSOC: c_int = 126;
pub const _SC_LEVEL1_ICACHE_LINESIZE: c_int = 127;
pub const _SC_LEVEL1_DCACHE_SIZE: c_int = 128;
pub const _SC_LEVEL1_DCACHE_ASSOC: c_int = 129;
pub const _SC_LEVEL1_DCACHE_LINESIZE: c_int = 130;
pub const _SC_LEVEL2_CACHE_SIZE: c_int = 131;
pub const _SC_LEVEL2_CACHE_ASSOC: c_int = 132;
pub const _SC_LEVEL2_CACHE_LINESIZE: c_int = 133;
pub const _SC_LEVEL3_CACHE_SIZE: c_int = 134;
pub const _SC_LEVEL3_CACHE_ASSOC: c_int = 135;
pub const _SC_LEVEL3_CACHE_LINESIZE: c_int = 136;
pub const _SC_LEVEL4_CACHE_SIZE: c_int = 137;
pub const _SC_LEVEL4_CACHE_ASSOC: c_int = 138;
pub const _SC_LEVEL4_CACHE_LINESIZE: c_int = 139;
pub const _SC_POSIX_26_VERSION: c_int = 140;
pub const _PC_LINK_MAX: c_int = 0;
pub const _PC_MAX_CANON: c_int = 1;
pub const _PC_MAX_INPUT: c_int = 2;
pub const _PC_NAME_MAX: c_int = 3;
pub const _PC_PATH_MAX: c_int = 4;
pub const _PC_PIPE_BUF: c_int = 5;
pub const _PC_CHOWN_RESTRICTED: c_int = 6;
pub const _PC_NO_TRUNC: c_int = 7;
pub const _PC_VDISABLE: c_int = 8;
pub const _PC_ASYNC_IO: c_int = 9;
pub const _PC_PRIO_IO: c_int = 10;
pub const _PC_SYNC_IO: c_int = 11;
pub const _PC_FILESIZEBITS: c_int = 12;
pub const _PC_2_SYMLINKS: c_int = 13;
pub const _PC_SYMLINK_MAX: c_int = 14;
pub const _PC_ALLOC_SIZE_MIN: c_int = 15;
pub const _PC_REC_INCR_XFER_SIZE: c_int = 16;
pub const _PC_REC_MAX_XFER_SIZE: c_int = 17;
pub const _PC_REC_MIN_XFER_SIZE: c_int = 18;
pub const _PC_REC_XFER_ALIGN: c_int = 19;
pub const _PC_TIMESTAMP_RESOLUTION: c_int = 20;
pub const FIONBIO: c_int = -2147195266;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    type_: PTHREAD_MUTEX_NORMAL,
    inner: pthread_mutex_t_inner {
        normal: __LOCK_INITIALIZER,
    },
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    clock_id: CLOCK_REALTIME,
    cond: __COND_INITIALIZER,
};
pub type __int8_t = c_schar;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __uint16_t = c_ushort;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type __int_least8_t = c_schar;
pub type __uint_least8_t = c_uchar;
pub type __int_least16_t = c_short;
pub type __uint_least16_t = c_ushort;
pub type __int_least32_t = c_int;
pub type __uint_least32_t = c_uint;
pub type __int_least64_t = c_long;
pub type __uint_least64_t = c_ulong;
pub type __intmax_t = c_long;
pub type __uintmax_t = c_ulong;
pub type __intptr_t = c_long;
pub type __uintptr_t = c_ulong;
pub type size_t = usize;
pub type _ssize_t = isize;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = c_int;
pub type wchar_t = c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __max_align_ll: c_longlong,
    pub __bindgen_padding_0: u64,
    pub __max_align_ld: u128,
}
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type int_least8_t = __int_least8_t;
pub type uint_least8_t = __uint_least8_t;
pub type int_least16_t = __int_least16_t;
pub type uint_least16_t = __uint_least16_t;
pub type int_least32_t = __int_least32_t;
pub type uint_least32_t = __uint_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = c_schar;
pub type uint_fast8_t = c_uchar;
pub type int_fast16_t = c_short;
pub type uint_fast16_t = c_ushort;
pub type int_fast32_t = c_int;
pub type uint_fast32_t = c_uint;
pub type int_fast64_t = c_long;
pub type uint_fast64_t = c_ulong;
pub type _LOCK_T = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __lock_t {
    pub lock: _LOCK_T,
    pub thread_tag: u32,
    pub counter: u32,
}
pub type _LOCK_RECURSIVE_T = __lock_t;
pub type _COND_T = u32;
pub type __blkcnt_t = c_long;
pub type __blksize_t = c_long;
pub type __fsblkcnt_t = __uint64_t;
pub type __fsfilcnt_t = __uint32_t;
pub type _off_t = c_long;
pub type __pid_t = c_int;
pub type __dev_t = c_short;
pub type __uid_t = u32;
pub type __gid_t = c_ushort;
pub type __id_t = __uint32_t;
pub type __ino_t = c_ushort;
pub type __mode_t = __uint32_t;
pub type __off_t = _off_t;
pub type __key_t = c_long;
pub type _fpos_t = c_long;
pub type wint_t = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _mbstate_t {
    pub __count: c_int,
    pub __value: _mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _mbstate_t__bindgen_ty_1 {
    pub __wch: wint_t,
    pub __wchb: [c_uchar; 4usize],
    _bindgen_union_align: u32,
}
pub type _flock_t = _LOCK_RECURSIVE_T;
pub type _iconv_t = *mut c_void;
pub type __clockid_t = c_ulong;
pub type __timer_t = c_ulong;
pub type __sa_family_t = __uint8_t;
pub type __socklen_t = __uint32_t;
pub type __nl_item = c_int;
pub type __nlink_t = c_ushort;
pub type __suseconds_t = c_long;
pub type __useconds_t = c_ulong;
pub type __sigset_t = c_ulong;
pub type suseconds_t = __suseconds_t;
pub type time_t = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}
extern "C" {
    pub fn timespec2nsec(ts: *const timespec) -> __uint64_t;
}
extern "C" {
    pub fn abstimespec2nsec(clock_id: __clockid_t, ts: *const timespec) -> __uint64_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
pub type sigset_t = __sigset_t;
pub type fd_mask = c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [fd_mask; 1usize],
}
extern "C" {
    pub fn select(
        __n: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> c_int;
}
extern "C" {
    pub fn pselect(
        __n: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __set: *const sigset_t,
    ) -> c_int;
}
pub type in_addr_t = __uint32_t;
pub type in_port_t = __uint16_t;
pub type u_char = c_uchar;
pub type u_short = c_ushort;
pub type u_int = c_uint;
pub type u_long = c_ulong;
pub type ushort = c_ushort;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type blkcnt_t = __blkcnt_t;
pub type blksize_t = __blksize_t;
pub type clock_t = c_ulong;
pub type daddr_t = c_long;
pub type caddr_t = *mut c_char;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type id_t = __id_t;
pub type ino_t = __ino_t;
pub type off_t = __off_t;
pub type dev_t = __dev_t;
pub type uid_t = __uid_t;
pub type gid_t = __gid_t;
pub type pid_t = __pid_t;
pub type key_t = __key_t;
pub type ssize_t = _ssize_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type useconds_t = __useconds_t;
pub type sbintime_t = __int64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sched_param {
    pub sched_priority: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_t {
    _unused: [u8; 0],
}
pub type pthread_t = *mut __pthread_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_attr_t {
    pub stackaddr: *mut c_void,
    pub stacksize: c_int,
    pub schedparam: sched_param,
    pub detachstate: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pthread_mutex_t {
    pub type_: c_int,
    pub inner: pthread_mutex_t_inner,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t_inner {
    pub normal: _LOCK_T,
    pub recursive: _LOCK_RECURSIVE_T,
    align: [u32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_mutexattr_t {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_cond_t {
    pub clock_id: clockid_t,
    pub cond: _COND_T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_condattr_t {
    pub clock_id: clockid_t,
}
pub type pthread_key_t = __uint32_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_once_t {
    pub status: c_int,
}
extern "C" {
    pub static mut environ: *mut *mut c_char;
}
extern "C" {
    pub fn access(
        __path: *const c_char,
        __amode: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn alarm(__secs: c_uint) -> c_uint;
}
extern "C" {
    pub fn chdir(__path: *const c_char) -> c_int;
}
extern "C" {
    pub fn chmod(__path: *const c_char, __mode: mode_t) -> c_int;
}
extern "C" {
    pub fn chown(
        __path: *const c_char,
        __owner: uid_t,
        __group: gid_t,
    ) -> c_int;
}
extern "C" {
    pub fn chroot(__path: *const c_char) -> c_int;
}
extern "C" {
    pub fn close(__fildes: c_int) -> c_int;
}
extern "C" {
    pub fn confstr(
        __name: c_int,
        __buf: *mut c_char,
        __len: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn daemon(
        nochdir: c_int,
        noclose: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn dup(__fildes: c_int) -> c_int;
}
extern "C" {
    pub fn dup2(
        __fildes: c_int,
        __fildes2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn endusershell();
}
extern "C" {
    pub fn execl(
        __path: *const c_char,
        arg1: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn execle(
        __path: *const c_char,
        arg1: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn execlp(
        __file: *const c_char,
        arg1: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn execlpe(
        __file: *const c_char,
        arg1: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn execv(
        __path: *const c_char,
        __argv: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn execve(
        __path: *const c_char,
        __argv: *const *mut c_char,
        __envp: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn execvp(
        __file: *const c_char,
        __argv: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn faccessat(
        __dirfd: c_int,
        __path: *const c_char,
        __mode: c_int,
        __flags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn fchdir(__fildes: c_int) -> c_int;
}
extern "C" {
    pub fn fchmod(__fildes: c_int, __mode: mode_t) -> c_int;
}
extern "C" {
    pub fn fchown(
        __fildes: c_int,
        __owner: uid_t,
        __group: gid_t,
    ) -> c_int;
}
extern "C" {
    pub fn fchownat(
        __dirfd: c_int,
        __path: *const c_char,
        __owner: uid_t,
        __group: gid_t,
        __flags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn fexecve(
        __fd: c_int,
        __argv: *const *mut c_char,
        __envp: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn fork() -> pid_t;
}
extern "C" {
    pub fn fpathconf(
        __fd: c_int,
        __name: c_int,
    ) -> c_long;
}
extern "C" {
    pub fn fsync(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn fdatasync(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn getcwd(
        __buf: *mut c_char,
        __size: size_t,
    ) -> *mut c_char;
}
extern "C" {
    pub fn getdomainname(
        __name: *mut c_char,
        __len: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn getentropy(arg1: *mut c_void, arg2: size_t) -> c_int;
}
extern "C" {
    pub fn getegid() -> gid_t;
}
extern "C" {
    pub fn geteuid() -> uid_t;
}
extern "C" {
    pub fn getgid() -> gid_t;
}
extern "C" {
    pub fn getgroups(
        __gidsetsize: c_int,
        __grouplist: *mut gid_t,
    ) -> c_int;
}
extern "C" {
    pub fn gethostid() -> c_long;
}
extern "C" {
    pub fn getlogin() -> *mut c_char;
}
extern "C" {
    pub fn getpass(__prompt: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn getpagesize() -> c_int;
}
extern "C" {
    pub fn getpeereid(
        arg1: c_int,
        arg2: *mut uid_t,
        arg3: *mut gid_t,
    ) -> c_int;
}
extern "C" {
    pub fn getpgid(arg1: pid_t) -> pid_t;
}
extern "C" {
    pub fn getpgrp() -> pid_t;
}
extern "C" {
    pub fn getpid() -> pid_t;
}
extern "C" {
    pub fn getppid() -> pid_t;
}
extern "C" {
    pub fn getsid(arg1: pid_t) -> pid_t;
}
extern "C" {
    pub fn getuid() -> uid_t;
}
extern "C" {
    pub fn getusershell() -> *mut c_char;
}
extern "C" {
    pub fn getwd(__buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn iruserok(
        raddr: c_ulong,
        superuser: c_int,
        ruser: *const c_char,
        luser: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn isatty(__fildes: c_int) -> c_int;
}
extern "C" {
    pub fn issetugid() -> c_int;
}
extern "C" {
    pub fn lchown(
        __path: *const c_char,
        __owner: uid_t,
        __group: gid_t,
    ) -> c_int;
}
extern "C" {
    pub fn link(
        __path1: *const c_char,
        __path2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn linkat(
        __dirfd1: c_int,
        __path1: *const c_char,
        __dirfd2: c_int,
        __path2: *const c_char,
        __flags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn nice(__nice_value: c_int) -> c_int;
}
extern "C" {
    pub fn lseek(
        __fildes: c_int,
        __offset: off_t,
        __whence: c_int,
    ) -> off_t;
}
extern "C" {
    pub fn lockf(
        __fd: c_int,
        __cmd: c_int,
        __len: off_t,
    ) -> c_int;
}
extern "C" {
    pub fn pathconf(
        __path: *const c_char,
        __name: c_int,
    ) -> c_long;
}
extern "C" {
    pub fn pause() -> c_int;
}
extern "C" {
    pub fn pthread_atfork(
        arg1: ::Option<unsafe extern "C" fn()>,
        arg2: ::Option<unsafe extern "C" fn()>,
        arg3: ::Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
extern "C" {
    pub fn pipe(__fildes: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pread(
        __fd: c_int,
        __buf: *mut c_void,
        __nbytes: size_t,
        __offset: off_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn pwrite(
        __fd: c_int,
        __buf: *const c_void,
        __nbytes: size_t,
        __offset: off_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn read(
        __fd: c_int,
        __buf: *mut c_void,
        __nbyte: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn rresvport(__alport: *mut c_int) -> c_int;
}
extern "C" {
    pub fn revoke(__path: *mut c_char) -> c_int;
}
extern "C" {
    pub fn rmdir(__path: *const c_char) -> c_int;
}
extern "C" {
    pub fn ruserok(
        rhost: *const c_char,
        superuser: c_int,
        ruser: *const c_char,
        luser: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn sbrk(__incr: isize) -> *mut c_void;
}
extern "C" {
    pub fn setegid(__gid: gid_t) -> c_int;
}
extern "C" {
    pub fn seteuid(__uid: uid_t) -> c_int;
}
extern "C" {
    pub fn setgid(__gid: gid_t) -> c_int;
}
extern "C" {
    pub fn setgroups(
        ngroups: c_int,
        grouplist: *const gid_t,
    ) -> c_int;
}
extern "C" {
    pub fn sethostname(arg1: *const c_char, arg2: size_t) -> c_int;
}
extern "C" {
    pub fn setpgid(__pid: pid_t, __pgid: pid_t) -> c_int;
}
extern "C" {
    pub fn setpgrp() -> c_int;
}
extern "C" {
    pub fn setregid(__rgid: gid_t, __egid: gid_t) -> c_int;
}
extern "C" {
    pub fn setreuid(__ruid: uid_t, __euid: uid_t) -> c_int;
}
extern "C" {
    pub fn setsid() -> pid_t;
}
extern "C" {
    pub fn setuid(__uid: uid_t) -> c_int;
}
extern "C" {
    pub fn setusershell();
}
extern "C" {
    pub fn sleep(__seconds: c_uint) -> c_uint;
}
extern "C" {
    pub fn tcgetpgrp(__fildes: c_int) -> pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fildes: c_int, __pgrp_id: pid_t) -> c_int;
}
extern "C" {
    pub fn ttyname(__fildes: c_int) -> *mut c_char;
}
extern "C" {
    pub fn ttyname_r(
        arg1: c_int,
        arg2: *mut c_char,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn unlink(__path: *const c_char) -> c_int;
}
extern "C" {
    pub fn usleep(__useconds: useconds_t) -> c_int;
}
extern "C" {
    pub fn vhangup() -> c_int;
}
extern "C" {
    pub fn write(
        __fd: c_int,
        __buf: *const c_void,
        __nbyte: size_t,
    ) -> c_int;
}
extern "C" {
    pub static mut optarg: *mut c_char;
}
extern "C" {
    pub static mut optind: c_int;
}
extern "C" {
    pub static mut opterr: c_int;
}
extern "C" {
    pub static mut optopt: c_int;
}
extern "C" {
    pub fn getopt(
        arg1: c_int,
        arg2: *const *mut c_char,
        arg3: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub static mut optreset: c_int;
}
extern "C" {
    pub fn vfork() -> c_int;
}
extern "C" {
    pub fn ftruncate(__fd: c_int, __length: off_t) -> c_int;
}
extern "C" {
    pub fn truncate(arg1: *const c_char, __length: off_t) -> c_int;
}
extern "C" {
    pub fn getdtablesize() -> c_int;
}
extern "C" {
    pub fn ualarm(__useconds: useconds_t, __interval: useconds_t) -> useconds_t;
}
extern "C" {
    pub fn gethostname(__name: *mut c_char, __len: size_t)
        -> c_int;
}
extern "C" {
    pub fn setdtablesize(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn sync();
}
extern "C" {
    pub fn readlink(
        __path: *const c_char,
        __buf: *mut c_char,
        __buflen: size_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn symlink(
        __name1: *const c_char,
        __name2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn readlinkat(
        __dirfd1: c_int,
        __path: *const c_char,
        __buf: *mut c_char,
        __buflen: size_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn symlinkat(
        arg1: *const c_char,
        arg2: c_int,
        arg3: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn unlinkat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
pub type once_flag = pthread_once_t;
pub type tss_t = pthread_key_t;
pub type mtx_t = pthread_mutex_t;
pub type cnd_t = pthread_cond_t;
pub type thrd_t = pthread_t;
pub type __ULong = c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Bigint {
    pub _next: *mut _Bigint,
    pub _k: c_int,
    pub _maxwds: c_int,
    pub _sign: c_int,
    pub _wds: c_int,
    pub _x: [__ULong; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __tm {
    pub __tm_sec: c_int,
    pub __tm_min: c_int,
    pub __tm_hour: c_int,
    pub __tm_mday: c_int,
    pub __tm_mon: c_int,
    pub __tm_year: c_int,
    pub __tm_wday: c_int,
    pub __tm_yday: c_int,
    pub __tm_isdst: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _on_exit_args {
    pub _fnargs: [*mut c_void; 32usize],
    pub _dso_handle: [*mut c_void; 32usize],
    pub _fntypes: __ULong,
    pub _is_cxa: __ULong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _atexit {
    pub _next: *mut _atexit,
    pub _ind: c_int,
    pub _fns: [::Option<unsafe extern "C" fn()>; 32usize],
    pub _on_exit_args: _on_exit_args,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut c_uchar,
    pub _size: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut c_uchar,
    pub _r: c_int,
    pub _w: c_int,
    pub _flags: c_short,
    pub _file: c_short,
    pub _bf: __sbuf,
    pub _lbfsize: c_int,
    pub _cookie: *mut c_void,
    pub _read: ::Option<
        unsafe extern "C" fn(
            arg1: *mut _reent,
            arg2: *mut c_void,
            arg3: *mut c_char,
            arg4: c_int,
        ) -> c_int,
    >,
    pub _write: ::Option<
        unsafe extern "C" fn(
            arg1: *mut _reent,
            arg2: *mut c_void,
            arg3: *const c_char,
            arg4: c_int,
        ) -> c_int,
    >,
    pub _seek: ::Option<
        unsafe extern "C" fn(
            arg1: *mut _reent,
            arg2: *mut c_void,
            arg3: _fpos_t,
            arg4: c_int,
        ) -> _fpos_t,
    >,
    pub _close: ::Option<
        unsafe extern "C" fn(
            arg1: *mut _reent,
            arg2: *mut c_void,
        ) -> c_int,
    >,
    pub _ub: __sbuf,
    pub _up: *mut c_uchar,
    pub _ur: c_int,
    pub _ubuf: [c_uchar; 3usize],
    pub _nbuf: [c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: c_int,
    pub _offset: _off_t,
    pub _data: *mut _reent,
    pub _lock: _flock_t,
    pub _mbstate: _mbstate_t,
    pub _flags2: c_int,
}
pub type __FILE = __sFILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _glue {
    pub _next: *mut _glue,
    pub _niobs: c_int,
    pub _iobs: *mut __FILE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _rand48 {
    pub _seed: [c_ushort; 3usize],
    pub _mult: [c_ushort; 3usize],
    pub _add: c_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _reent {
    pub _errno: c_int,
    pub _stdin: *mut __FILE,
    pub _stdout: *mut __FILE,
    pub _stderr: *mut __FILE,
    pub _inc: c_int,
    pub _emergency: [c_char; 25usize],
    pub _unspecified_locale_info: c_int,
    pub _locale: *mut __locale_t,
    pub __sdidinit: c_int,
    pub __cleanup: ::Option<unsafe extern "C" fn(arg1: *mut _reent)>,
    pub _result: *mut _Bigint,
    pub _result_k: c_int,
    pub _p5s: *mut _Bigint,
    pub _freelist: *mut *mut _Bigint,
    pub _cvtlen: c_int,
    pub _cvtbuf: *mut c_char,
    pub _new: _reent__bindgen_ty_1,
    pub _atexit: *mut _atexit,
    pub _atexit0: _atexit,
    pub _sig_func: *mut ::Option<unsafe extern "C" fn(arg1: c_int)>,
    pub __sglue: _glue,
    pub __sf: [__FILE; 3usize],
    pub deviceData: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _reent__bindgen_ty_1 {
    pub _reent: _reent__bindgen_ty_1__bindgen_ty_1,
    pub _unused: _reent__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u64; 45usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _reent__bindgen_ty_1__bindgen_ty_1 {
    pub _unused_rand: c_uint,
    pub _strtok_last: *mut c_char,
    pub _asctime_buf: [c_char; 26usize],
    pub _localtime_buf: __tm,
    pub _gamma_signgam: c_int,
    pub _rand_next: c_ulonglong,
    pub _r48: _rand48,
    pub _mblen_state: _mbstate_t,
    pub _mbtowc_state: _mbstate_t,
    pub _wctomb_state: _mbstate_t,
    pub _l64a_buf: [c_char; 8usize],
    pub _signal_buf: [c_char; 24usize],
    pub _getdate_err: c_int,
    pub _mbrlen_state: _mbstate_t,
    pub _mbrtowc_state: _mbstate_t,
    pub _mbsrtowcs_state: _mbstate_t,
    pub _wcrtomb_state: _mbstate_t,
    pub _wcsrtombs_state: _mbstate_t,
    pub _h_errno: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _reent__bindgen_ty_1__bindgen_ty_2 {
    pub _nextf: [*mut c_uchar; 30usize],
    pub _nmalloc: [c_uint; 30usize],
}
pub type locale_t = *mut __locale_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn difftime(_time2: time_t, _time1: time_t) -> f64;
}
extern "C" {
    pub fn mktime(_timeptr: *mut tm) -> time_t;
}
extern "C" {
    pub fn time(_timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn asctime(_tblock: *const tm) -> *mut c_char;
}
extern "C" {
    pub fn ctime(_time: *const time_t) -> *mut c_char;
}
extern "C" {
    pub fn gmtime(_timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(_timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn strftime(
        _s: *mut c_char,
        _maxsize: size_t,
        _fmt: *const c_char,
        _t: *const tm,
    ) -> size_t;
}
extern "C" {
    pub fn strftime_l(
        _s: *mut c_char,
        _maxsize: size_t,
        _fmt: *const c_char,
        _t: *const tm,
        _l: locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn asctime_r(
        arg1: *const tm,
        arg2: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn ctime_r(
        arg1: *const time_t,
        arg2: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn gmtime_r(arg1: *const time_t, arg2: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(arg1: *const time_t, arg2: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn tzset();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __tzrule_struct {
    pub ch: c_char,
    pub m: c_int,
    pub n: c_int,
    pub d: c_int,
    pub s: c_int,
    pub change: time_t,
    pub offset: c_long,
}
pub type __tzrule_type = __tzrule_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __tzinfo_struct {
    pub __tznorth: c_int,
    pub __tzyear: c_int,
    pub __tzrule: [__tzrule_type; 2usize],
}
pub type __tzinfo_type = __tzinfo_struct;
pub type tss_dtor_t =
    ::Option<unsafe extern "C" fn(arg1: *mut c_void)>;
pub type thrd_start_t = ::Option<
    unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
>;
pub const mtx_plain: _bindgen_ty_1 = 1;
pub const mtx_recursive: _bindgen_ty_1 = 2;
pub const mtx_timed: _bindgen_ty_1 = 4;
pub type _bindgen_ty_1 = u32;
pub const thrd_busy: _bindgen_ty_2 = 1;
pub const thrd_error: _bindgen_ty_2 = 2;
pub const thrd_nomem: _bindgen_ty_2 = 3;
pub const thrd_success: _bindgen_ty_2 = 4;
pub const thrd_timedout: _bindgen_ty_2 = 5;
pub type _bindgen_ty_2 = u32;
extern "C" {
    pub fn call_once(arg1: *mut once_flag, arg2: ::Option<unsafe extern "C" fn()>);
}
extern "C" {
    pub fn cnd_broadcast(arg1: *mut cnd_t) -> c_int;
}
extern "C" {
    pub fn cnd_destroy(arg1: *mut cnd_t);
}
extern "C" {
    pub fn cnd_init(arg1: *mut cnd_t) -> c_int;
}
extern "C" {
    pub fn cnd_signal(arg1: *mut cnd_t) -> c_int;
}
extern "C" {
    pub fn cnd_timedwait(
        arg1: *mut cnd_t,
        __mtx: *mut mtx_t,
        arg2: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn cnd_wait(arg1: *mut cnd_t, __mtx: *mut mtx_t) -> c_int;
}
extern "C" {
    pub fn mtx_destroy(__mtx: *mut mtx_t);
}
extern "C" {
    pub fn mtx_init(__mtx: *mut mtx_t, arg1: c_int) -> c_int;
}
extern "C" {
    pub fn mtx_lock(__mtx: *mut mtx_t) -> c_int;
}
extern "C" {
    pub fn mtx_timedlock(__mtx: *mut mtx_t, arg1: *const timespec) -> c_int;
}
extern "C" {
    pub fn mtx_trylock(__mtx: *mut mtx_t) -> c_int;
}
extern "C" {
    pub fn mtx_unlock(__mtx: *mut mtx_t) -> c_int;
}
extern "C" {
    pub fn thrd_create(
        arg1: *mut thrd_t,
        arg2: thrd_start_t,
        arg3: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn thrd_current() -> thrd_t;
}
extern "C" {
    pub fn thrd_detach(arg1: thrd_t) -> c_int;
}
extern "C" {
    pub fn thrd_equal(arg1: thrd_t, arg2: thrd_t) -> c_int;
}
extern "C" {
    pub fn thrd_exit(arg1: c_int);
}
extern "C" {
    pub fn thrd_join(arg1: thrd_t, arg2: *mut c_int) -> c_int;
}
extern "C" {
    pub fn thrd_sleep(arg1: *const timespec, arg2: *mut timespec) -> c_int;
}
extern "C" {
    pub fn thrd_yield();
}
extern "C" {
    pub fn tss_create(arg1: *mut tss_t, arg2: tss_dtor_t) -> c_int;
}
extern "C" {
    pub fn tss_delete(arg1: tss_t);
}
extern "C" {
    pub fn tss_get(arg1: tss_t) -> *mut c_void;
}
extern "C" {
    pub fn tss_set(arg1: tss_t, arg2: *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: gid_t,
    pub gr_mem: *mut *mut c_char,
}
extern "C" {
    pub fn getgrgid(arg1: gid_t) -> *mut group;
}
extern "C" {
    pub fn getgrnam(arg1: *const c_char) -> *mut group;
}
extern "C" {
    pub fn getgrnam_r(
        arg1: *const c_char,
        arg2: *mut group,
        arg3: *mut c_char,
        arg4: size_t,
        arg5: *mut *mut group,
    ) -> c_int;
}
extern "C" {
    pub fn getgrgid_r(
        arg1: gid_t,
        arg2: *mut group,
        arg3: *mut c_char,
        arg4: size_t,
        arg5: *mut *mut group,
    ) -> c_int;
}
extern "C" {
    pub fn getgrent() -> *mut group;
}
extern "C" {
    pub fn setgrent();
}
extern "C" {
    pub fn endgrent();
}
extern "C" {
    pub fn initgroups(arg1: *const c_char, arg2: gid_t) -> c_int;
}
extern "C" {
    pub fn atan(arg1: f64) -> f64;
}
extern "C" {
    pub fn cos(arg1: f64) -> f64;
}
extern "C" {
    pub fn sin(arg1: f64) -> f64;
}
extern "C" {
    pub fn tan(arg1: f64) -> f64;
}
extern "C" {
    pub fn tanh(arg1: f64) -> f64;
}
extern "C" {
    pub fn frexp(arg1: f64, arg2: *mut c_int) -> f64;
}
extern "C" {
    pub fn modf(arg1: f64, arg2: *mut f64) -> f64;
}
extern "C" {
    pub fn ceil(arg1: f64) -> f64;
}
extern "C" {
    pub fn fabs(arg1: f64) -> f64;
}
extern "C" {
    pub fn floor(arg1: f64) -> f64;
}
extern "C" {
    pub fn acos(arg1: f64) -> f64;
}
extern "C" {
    pub fn asin(arg1: f64) -> f64;
}
extern "C" {
    pub fn atan2(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn cosh(arg1: f64) -> f64;
}
extern "C" {
    pub fn sinh(arg1: f64) -> f64;
}
extern "C" {
    pub fn exp(arg1: f64) -> f64;
}
extern "C" {
    pub fn ldexp(arg1: f64, arg2: c_int) -> f64;
}
extern "C" {
    pub fn log(arg1: f64) -> f64;
}
extern "C" {
    pub fn log10(arg1: f64) -> f64;
}
extern "C" {
    pub fn pow(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn sqrt(arg1: f64) -> f64;
}
extern "C" {
    pub fn fmod(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn finite(arg1: f64) -> c_int;
}
extern "C" {
    pub fn finitef(arg1: f32) -> c_int;
}
extern "C" {
    pub fn finitel(arg1: u128) -> c_int;
}
extern "C" {
    pub fn isinff(arg1: f32) -> c_int;
}
extern "C" {
    pub fn isnanf(arg1: f32) -> c_int;
}
extern "C" {
    pub fn isinf(arg1: f64) -> c_int;
}
extern "C" {
    pub fn isnan(arg1: f64) -> c_int;
}
pub type float_t = f32;
pub type double_t = f64;
extern "C" {
    pub fn infinity() -> f64;
}
extern "C" {
    pub fn nan(arg1: *const c_char) -> f64;
}
extern "C" {
    pub fn copysign(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn logb(arg1: f64) -> f64;
}
extern "C" {
    pub fn ilogb(arg1: f64) -> c_int;
}
extern "C" {
    pub fn asinh(arg1: f64) -> f64;
}
extern "C" {
    pub fn cbrt(arg1: f64) -> f64;
}
extern "C" {
    pub fn nextafter(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn rint(arg1: f64) -> f64;
}
extern "C" {
    pub fn scalbn(arg1: f64, arg2: c_int) -> f64;
}
extern "C" {
    pub fn exp2(arg1: f64) -> f64;
}
extern "C" {
    pub fn scalbln(arg1: f64, arg2: c_long) -> f64;
}
extern "C" {
    pub fn tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn nearbyint(arg1: f64) -> f64;
}
extern "C" {
    pub fn lrint(arg1: f64) -> c_long;
}
extern "C" {
    pub fn llrint(arg1: f64) -> c_longlong;
}
extern "C" {
    pub fn round(arg1: f64) -> f64;
}
extern "C" {
    pub fn lround(arg1: f64) -> c_long;
}
extern "C" {
    pub fn llround(arg1: f64) -> c_longlong;
}
extern "C" {
    pub fn trunc(arg1: f64) -> f64;
}
extern "C" {
    pub fn remquo(arg1: f64, arg2: f64, arg3: *mut c_int) -> f64;
}
extern "C" {
    pub fn fdim(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn fmax(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn fmin(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn fma(arg1: f64, arg2: f64, arg3: f64) -> f64;
}
extern "C" {
    pub fn log1p(arg1: f64) -> f64;
}
extern "C" {
    pub fn expm1(arg1: f64) -> f64;
}
extern "C" {
    pub fn acosh(arg1: f64) -> f64;
}
extern "C" {
    pub fn atanh(arg1: f64) -> f64;
}
extern "C" {
    pub fn remainder(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn log2(arg1: f64) -> f64;
}
extern "C" {
    pub fn hypot(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn atanf(arg1: f32) -> f32;
}
extern "C" {
    pub fn cosf(arg1: f32) -> f32;
}
extern "C" {
    pub fn sinf(arg1: f32) -> f32;
}
extern "C" {
    pub fn tanf(arg1: f32) -> f32;
}
extern "C" {
    pub fn tanhf(arg1: f32) -> f32;
}
extern "C" {
    pub fn frexpf(arg1: f32, arg2: *mut c_int) -> f32;
}
extern "C" {
    pub fn modff(arg1: f32, arg2: *mut f32) -> f32;
}
extern "C" {
    pub fn ceilf(arg1: f32) -> f32;
}
extern "C" {
    pub fn fabsf(arg1: f32) -> f32;
}
extern "C" {
    pub fn floorf(arg1: f32) -> f32;
}
extern "C" {
    pub fn acosf(arg1: f32) -> f32;
}
extern "C" {
    pub fn asinf(arg1: f32) -> f32;
}
extern "C" {
    pub fn atan2f(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn coshf(arg1: f32) -> f32;
}
extern "C" {
    pub fn sinhf(arg1: f32) -> f32;
}
extern "C" {
    pub fn expf(arg1: f32) -> f32;
}
extern "C" {
    pub fn ldexpf(arg1: f32, arg2: c_int) -> f32;
}
extern "C" {
    pub fn logf(arg1: f32) -> f32;
}
extern "C" {
    pub fn log10f(arg1: f32) -> f32;
}
extern "C" {
    pub fn powf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn sqrtf(arg1: f32) -> f32;
}
extern "C" {
    pub fn fmodf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn exp2f(arg1: f32) -> f32;
}
extern "C" {
    pub fn scalblnf(arg1: f32, arg2: c_long) -> f32;
}
extern "C" {
    pub fn tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn nearbyintf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lrintf(arg1: f32) -> c_long;
}
extern "C" {
    pub fn llrintf(arg1: f32) -> c_longlong;
}
extern "C" {
    pub fn roundf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lroundf(arg1: f32) -> c_long;
}
extern "C" {
    pub fn llroundf(arg1: f32) -> c_longlong;
}
extern "C" {
    pub fn truncf(arg1: f32) -> f32;
}
extern "C" {
    pub fn remquof(arg1: f32, arg2: f32, arg3: *mut c_int) -> f32;
}
extern "C" {
    pub fn fdimf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn fmaxf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn fminf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn fmaf(arg1: f32, arg2: f32, arg3: f32) -> f32;
}
extern "C" {
    pub fn infinityf() -> f32;
}
extern "C" {
    pub fn nanf(arg1: *const c_char) -> f32;
}
extern "C" {
    pub fn copysignf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn logbf(arg1: f32) -> f32;
}
extern "C" {
    pub fn ilogbf(arg1: f32) -> c_int;
}
extern "C" {
    pub fn asinhf(arg1: f32) -> f32;
}
extern "C" {
    pub fn cbrtf(arg1: f32) -> f32;
}
extern "C" {
    pub fn nextafterf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn rintf(arg1: f32) -> f32;
}
extern "C" {
    pub fn scalbnf(arg1: f32, arg2: c_int) -> f32;
}
extern "C" {
    pub fn log1pf(arg1: f32) -> f32;
}
extern "C" {
    pub fn expm1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn acoshf(arg1: f32) -> f32;
}
extern "C" {
    pub fn atanhf(arg1: f32) -> f32;
}
extern "C" {
    pub fn remainderf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn log2f(arg1: f32) -> f32;
}
extern "C" {
    pub fn hypotf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn hypotl(arg1: u128, arg2: u128) -> u128;
}
extern "C" {
    pub fn sqrtl(arg1: u128) -> u128;
}
extern "C" {
    pub fn drem(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn dremf(arg1: f32, arg2: f32) -> f32;
}
extern "C" {
    pub fn gamma_r(arg1: f64, arg2: *mut c_int) -> f64;
}
extern "C" {
    pub fn lgamma_r(arg1: f64, arg2: *mut c_int) -> f64;
}
extern "C" {
    pub fn gammaf_r(arg1: f32, arg2: *mut c_int) -> f32;
}
extern "C" {
    pub fn lgammaf_r(arg1: f32, arg2: *mut c_int) -> f32;
}
extern "C" {
    pub fn y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn yn(arg1: c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn jn(arg1: c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn ynf(arg1: c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn jnf(arg1: c_int, arg2: f32) -> f32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct exception {
    pub type_: c_int,
    pub name: *mut c_char,
    pub arg1: f64,
    pub arg2: f64,
    pub retval: f64,
    pub err: c_int,
}
extern "C" {
    pub fn matherr(e: *mut exception) -> c_int;
}
pub type Elf32_Half = u16;
pub type Elf64_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf32_Xword = u64;
pub type Elf32_Sxword = i64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf32_Addr = u32;
pub type Elf64_Addr = u64;
pub type Elf32_Off = u32;
pub type Elf64_Off = u64;
pub type Elf32_Section = u16;
pub type Elf64_Section = u16;
pub type Elf32_Versym = Elf32_Half;
pub type Elf64_Versym = Elf64_Half;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Ehdr {
    pub e_ident: [c_uchar; 16usize],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Ehdr {
    pub e_ident: [c_uchar; 16usize],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Shdr {
    pub sh_name: Elf32_Word,
    pub sh_type: Elf32_Word,
    pub sh_flags: Elf32_Word,
    pub sh_addr: Elf32_Addr,
    pub sh_offset: Elf32_Off,
    pub sh_size: Elf32_Word,
    pub sh_link: Elf32_Word,
    pub sh_info: Elf32_Word,
    pub sh_addralign: Elf32_Word,
    pub sh_entsize: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Chdr {
    pub ch_type: Elf32_Word,
    pub ch_size: Elf32_Word,
    pub ch_addralign: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Chdr {
    pub ch_type: Elf64_Word,
    pub ch_reserved: Elf64_Word,
    pub ch_size: Elf64_Xword,
    pub ch_addralign: Elf64_Xword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Sym {
    pub st_name: Elf32_Word,
    pub st_value: Elf32_Addr,
    pub st_size: Elf32_Word,
    pub st_info: c_uchar,
    pub st_other: c_uchar,
    pub st_shndx: Elf32_Section,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: c_uchar,
    pub st_other: c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Syminfo {
    pub si_boundto: Elf32_Half,
    pub si_flags: Elf32_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Syminfo {
    pub si_boundto: Elf64_Half,
    pub si_flags: Elf64_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Rel {
    pub r_offset: Elf32_Addr,
    pub r_info: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Rel {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Rela {
    pub r_offset: Elf32_Addr,
    pub r_info: Elf32_Word,
    pub r_addend: Elf32_Sword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Dyn {
    pub d_tag: Elf32_Sword,
    pub d_un: Elf32_Dyn__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf32_Dyn__bindgen_ty_1 {
    pub d_val: Elf32_Word,
    pub d_ptr: Elf32_Addr,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Dyn {
    pub d_tag: Elf64_Sxword,
    pub d_un: Elf64_Dyn__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf64_Dyn__bindgen_ty_1 {
    pub d_val: Elf64_Xword,
    pub d_ptr: Elf64_Addr,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Verdef {
    pub vd_version: Elf32_Half,
    pub vd_flags: Elf32_Half,
    pub vd_ndx: Elf32_Half,
    pub vd_cnt: Elf32_Half,
    pub vd_hash: Elf32_Word,
    pub vd_aux: Elf32_Word,
    pub vd_next: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Verdef {
    pub vd_version: Elf64_Half,
    pub vd_flags: Elf64_Half,
    pub vd_ndx: Elf64_Half,
    pub vd_cnt: Elf64_Half,
    pub vd_hash: Elf64_Word,
    pub vd_aux: Elf64_Word,
    pub vd_next: Elf64_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Verdaux {
    pub vda_name: Elf32_Word,
    pub vda_next: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Verdaux {
    pub vda_name: Elf64_Word,
    pub vda_next: Elf64_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Verneed {
    pub vn_version: Elf32_Half,
    pub vn_cnt: Elf32_Half,
    pub vn_file: Elf32_Word,
    pub vn_aux: Elf32_Word,
    pub vn_next: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Verneed {
    pub vn_version: Elf64_Half,
    pub vn_cnt: Elf64_Half,
    pub vn_file: Elf64_Word,
    pub vn_aux: Elf64_Word,
    pub vn_next: Elf64_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Vernaux {
    pub vna_hash: Elf32_Word,
    pub vna_flags: Elf32_Half,
    pub vna_other: Elf32_Half,
    pub vna_name: Elf32_Word,
    pub vna_next: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Vernaux {
    pub vna_hash: Elf64_Word,
    pub vna_flags: Elf64_Half,
    pub vna_other: Elf64_Half,
    pub vna_name: Elf64_Word,
    pub vna_next: Elf64_Word,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_auxv_t {
    pub a_type: u32,
    pub a_un: Elf32_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf32_auxv_t__bindgen_ty_1 {
    pub a_val: u32,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_auxv_t {
    pub a_type: u64,
    pub a_un: Elf64_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf64_auxv_t__bindgen_ty_1 {
    pub a_val: u64,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Nhdr {
    pub n_namesz: Elf32_Word,
    pub n_descsz: Elf32_Word,
    pub n_type: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Nhdr {
    pub n_namesz: Elf64_Word,
    pub n_descsz: Elf64_Word,
    pub n_type: Elf64_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Move {
    pub m_value: Elf32_Xword,
    pub m_info: Elf32_Word,
    pub m_poffset: Elf32_Word,
    pub m_repeat: Elf32_Half,
    pub m_stride: Elf32_Half,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Move {
    pub m_value: Elf64_Xword,
    pub m_info: Elf64_Xword,
    pub m_poffset: Elf64_Xword,
    pub m_repeat: Elf64_Half,
    pub m_stride: Elf64_Half,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf32_gptab {
    pub gt_header: Elf32_gptab__bindgen_ty_1,
    pub gt_entry: Elf32_gptab__bindgen_ty_2,
    _bindgen_union_align: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_gptab__bindgen_ty_1 {
    pub gt_current_g_value: Elf32_Word,
    pub gt_unused: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_gptab__bindgen_ty_2 {
    pub gt_g_value: Elf32_Word,
    pub gt_bytes: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_RegInfo {
    pub ri_gprmask: Elf32_Word,
    pub ri_cprmask: [Elf32_Word; 4usize],
    pub ri_gp_value: Elf32_Sword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf_Options {
    pub kind: c_uchar,
    pub size: c_uchar,
    pub section: Elf32_Section,
    pub info: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf_Options_Hw {
    pub hwp_flags1: Elf32_Word,
    pub hwp_flags2: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf32_Lib {
    pub l_name: Elf32_Word,
    pub l_time_stamp: Elf32_Word,
    pub l_checksum: Elf32_Word,
    pub l_version: Elf32_Word,
    pub l_flags: Elf32_Word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64_Lib {
    pub l_name: Elf64_Word,
    pub l_time_stamp: Elf64_Word,
    pub l_checksum: Elf64_Word,
    pub l_version: Elf64_Word,
    pub l_flags: Elf64_Word,
}
pub type Elf32_Conflict = Elf32_Addr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Elf_MIPS_ABIFlags_v0 {
    pub version: Elf32_Half,
    pub isa_level: c_uchar,
    pub isa_rev: c_uchar,
    pub gpr_size: c_uchar,
    pub cpr1_size: c_uchar,
    pub cpr2_size: c_uchar,
    pub fp_abi: c_uchar,
    pub isa_ext: Elf32_Word,
    pub ases: Elf32_Word,
    pub flags1: Elf32_Word,
    pub flags2: Elf32_Word,
}
pub const Val_GNU_MIPS_ABI_FP_ANY: _bindgen_ty_3 = 0;
pub const Val_GNU_MIPS_ABI_FP_DOUBLE: _bindgen_ty_3 = 1;
pub const Val_GNU_MIPS_ABI_FP_SINGLE: _bindgen_ty_3 = 2;
pub const Val_GNU_MIPS_ABI_FP_SOFT: _bindgen_ty_3 = 3;
pub const Val_GNU_MIPS_ABI_FP_OLD_64: _bindgen_ty_3 = 4;
pub const Val_GNU_MIPS_ABI_FP_XX: _bindgen_ty_3 = 5;
pub const Val_GNU_MIPS_ABI_FP_64: _bindgen_ty_3 = 6;
pub const Val_GNU_MIPS_ABI_FP_64A: _bindgen_ty_3 = 7;
pub const Val_GNU_MIPS_ABI_FP_MAX: _bindgen_ty_3 = 7;
pub type _bindgen_ty_3 = u32;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type FILE = __FILE;
pub type mbstate_t = _mbstate_t;
extern "C" {
    pub fn btowc(arg1: c_int) -> wint_t;
}
extern "C" {
    pub fn wctob(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn mbrlen(
        arg1: *const c_char,
        arg2: size_t,
        arg3: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbrtowc(
        arg1: *mut wchar_t,
        arg2: *const c_char,
        arg3: size_t,
        arg4: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbsinit(arg1: *const mbstate_t) -> c_int;
}
extern "C" {
    pub fn mbsnrtowcs(
        arg1: *mut wchar_t,
        arg2: *mut *const c_char,
        arg3: size_t,
        arg4: size_t,
        arg5: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbsrtowcs(
        arg1: *mut wchar_t,
        arg2: *mut *const c_char,
        arg3: size_t,
        arg4: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcrtomb(
        arg1: *mut c_char,
        arg2: wchar_t,
        arg3: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcsnrtombs(
        arg1: *mut c_char,
        arg2: *mut *const wchar_t,
        arg3: size_t,
        arg4: size_t,
        arg5: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcsrtombs(
        arg1: *mut c_char,
        arg2: *mut *const wchar_t,
        arg3: size_t,
        arg4: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcscasecmp(arg1: *const wchar_t, arg2: *const wchar_t) -> c_int;
}
extern "C" {
    pub fn wcscat(arg1: *mut wchar_t, arg2: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcschr(
        arg1: *const c_int,
        arg2: c_int,
    ) -> *mut c_int;
}
extern "C" {
    pub fn wcscmp(
        arg1: *const c_int,
        arg2: *const c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wcscoll(arg1: *const wchar_t, arg2: *const wchar_t) -> c_int;
}
extern "C" {
    pub fn wcscpy(arg1: *mut wchar_t, arg2: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcpcpy(arg1: *mut wchar_t, arg2: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsdup(arg1: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscspn(arg1: *const wchar_t, arg2: *const wchar_t) -> size_t;
}
extern "C" {
    pub fn wcsftime(
        arg1: *mut wchar_t,
        arg2: size_t,
        arg3: *const wchar_t,
        arg4: *const tm,
    ) -> size_t;
}
extern "C" {
    pub fn wcslcat(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> size_t;
}
extern "C" {
    pub fn wcslcpy(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> size_t;
}
extern "C" {
    pub fn wcslen(arg1: *const c_int) -> c_ulong;
}
extern "C" {
    pub fn wcsncasecmp(
        arg1: *const wchar_t,
        arg2: *const wchar_t,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn wcsncat(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsncmp(
        arg1: *const c_int,
        arg2: *const c_int,
        arg3: c_ulong,
    ) -> c_int;
}
extern "C" {
    pub fn wcsncpy(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcpncpy(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsnlen(arg1: *const wchar_t, arg2: size_t) -> size_t;
}
extern "C" {
    pub fn wcspbrk(arg1: *const wchar_t, arg2: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsrchr(arg1: *const wchar_t, arg2: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsspn(arg1: *const wchar_t, arg2: *const wchar_t) -> size_t;
}
extern "C" {
    pub fn wcsstr(arg1: *const wchar_t, arg2: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcstok(
        arg1: *mut wchar_t,
        arg2: *const wchar_t,
        arg3: *mut *mut wchar_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn wcstod(arg1: *const wchar_t, arg2: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn wcstof(arg1: *const wchar_t, arg2: *mut *mut wchar_t) -> f32;
}
extern "C" {
    pub fn wcsxfrm(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> size_t;
}
extern "C" {
    pub fn wcscasecmp_l(
        arg1: *const wchar_t,
        arg2: *const wchar_t,
        arg3: locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(
        arg1: *const wchar_t,
        arg2: *const wchar_t,
        arg3: size_t,
        arg4: locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn wcscoll_l(
        arg1: *const wchar_t,
        arg2: *const wchar_t,
        arg3: locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn wcsxfrm_l(
        arg1: *mut wchar_t,
        arg2: *const wchar_t,
        arg3: size_t,
        arg4: locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn wmemchr(
        arg1: *const c_int,
        arg2: c_int,
        arg3: c_ulong,
    ) -> *mut c_int;
}
extern "C" {
    pub fn wmemcmp(
        arg1: *const c_int,
        arg2: *const c_int,
        arg3: c_ulong,
    ) -> c_int;
}
extern "C" {
    pub fn wmemcpy(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wmemmove(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wmemset(arg1: *mut wchar_t, arg2: wchar_t, arg3: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcstol(
        arg1: *const wchar_t,
        arg2: *mut *mut wchar_t,
        arg3: c_int,
    ) -> c_long;
}
extern "C" {
    pub fn wcstoll(
        arg1: *const wchar_t,
        arg2: *mut *mut wchar_t,
        arg3: c_int,
    ) -> c_longlong;
}
extern "C" {
    pub fn wcstoul(
        arg1: *const wchar_t,
        arg2: *mut *mut wchar_t,
        arg3: c_int,
    ) -> c_ulong;
}
extern "C" {
    pub fn wcstoull(
        arg1: *const wchar_t,
        arg2: *mut *mut wchar_t,
        arg3: c_int,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn wcstold(arg1: *const wchar_t, arg2: *mut *mut wchar_t) -> u128;
}
extern "C" {
    pub fn fgetwc(arg1: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn fgetws(
        arg1: *mut wchar_t,
        arg2: c_int,
        arg3: *mut __FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputwc(arg1: wchar_t, arg2: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn fputws(arg1: *const wchar_t, arg2: *mut __FILE) -> c_int;
}
extern "C" {
    pub fn fwide(arg1: *mut __FILE, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn getwc(arg1: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn putwc(arg1: wchar_t, arg2: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(arg1: wchar_t) -> wint_t;
}
extern "C" {
    pub fn ungetwc(wc: wint_t, arg1: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn open_wmemstream(arg1: *mut *mut wchar_t, arg2: *mut size_t) -> *mut __FILE;
}
extern "C" {
    pub fn fwprintf(arg1: *mut __FILE, arg2: *const wchar_t, ...) -> c_int;
}
extern "C" {
    pub fn swprintf(
        arg1: *mut wchar_t,
        arg2: size_t,
        arg3: *const wchar_t,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn vfwprintf(
        arg1: *mut __FILE,
        arg2: *const wchar_t,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vswprintf(
        arg1: *mut wchar_t,
        arg2: size_t,
        arg3: *const wchar_t,
        arg4: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vwprintf(arg1: *const wchar_t, arg2: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn wprintf(arg1: *const wchar_t, ...) -> c_int;
}
extern "C" {
    pub fn fwscanf(arg1: *mut __FILE, arg2: *const wchar_t, ...) -> c_int;
}
extern "C" {
    pub fn swscanf(arg1: *const wchar_t, arg2: *const wchar_t, ...) -> c_int;
}
extern "C" {
    pub fn vfwscanf(
        arg1: *mut __FILE,
        arg2: *const wchar_t,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vswscanf(
        arg1: *const wchar_t,
        arg2: *const wchar_t,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vwscanf(arg1: *const wchar_t, arg2: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn wscanf(arg1: *const wchar_t, ...) -> c_int;
}
pub type jmp_buf = [c_longlong; 8usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut c_char,
    pub we_offs: size_t,
}
pub type wordexp_t = _wordexp_t;
pub const WRDE_SUCCESS: _bindgen_ty_4 = 0;
pub const WRDE_NOSPACE: _bindgen_ty_4 = 1;
pub const WRDE_BADCHAR: _bindgen_ty_4 = 2;
pub const WRDE_BADVAL: _bindgen_ty_4 = 3;
pub const WRDE_CMDSUB: _bindgen_ty_4 = 4;
pub const WRDE_SYNTAX: _bindgen_ty_4 = 5;
pub const WRDE_NOSYS: _bindgen_ty_4 = 6;
pub type _bindgen_ty_4 = u32;
extern "C" {
    pub fn wordexp(
        arg1: *const c_char,
        arg2: *mut wordexp_t,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wordfree(arg1: *mut wordexp_t);
}
extern "C" {
    pub fn fnmatch(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
pub const memory_order_memory_order_relaxed: memory_order = 0;
pub const memory_order_memory_order_consume: memory_order = 1;
pub const memory_order_memory_order_acquire: memory_order = 2;
pub const memory_order_memory_order_release: memory_order = 3;
pub const memory_order_memory_order_acq_rel: memory_order = 4;
pub const memory_order_memory_order_seq_cst: memory_order = 5;
pub type memory_order = u32;
pub type atomic_bool = u8;
pub type atomic_char = u8;
pub type atomic_schar = u8;
pub type atomic_uchar = u8;
pub type atomic_short = u16;
pub type atomic_ushort = u16;
pub type atomic_int = u32;
pub type atomic_uint = u32;
pub type atomic_long = u64;
pub type atomic_ulong = u64;
pub type atomic_llong = u64;
pub type atomic_ullong = u64;
pub type atomic_wchar_t = wchar_t;
pub type atomic_int_least8_t = int_least8_t;
pub type atomic_uint_least8_t = uint_least8_t;
pub type atomic_int_least16_t = int_least16_t;
pub type atomic_uint_least16_t = uint_least16_t;
pub type atomic_int_least32_t = int_least32_t;
pub type atomic_uint_least32_t = uint_least32_t;
pub type atomic_int_least64_t = int_least64_t;
pub type atomic_uint_least64_t = uint_least64_t;
pub type atomic_int_fast8_t = int_fast8_t;
pub type atomic_uint_fast8_t = uint_fast8_t;
pub type atomic_int_fast16_t = int_fast16_t;
pub type atomic_uint_fast16_t = uint_fast16_t;
pub type atomic_int_fast32_t = int_fast32_t;
pub type atomic_uint_fast32_t = uint_fast32_t;
pub type atomic_int_fast64_t = int_fast64_t;
pub type atomic_uint_fast64_t = uint_fast64_t;
pub type atomic_intptr_t = isize;
pub type atomic_uintptr_t = usize;
pub type atomic_size_t = size_t;
pub type atomic_ptrdiff_t = isize;
pub type atomic_intmax_t = intmax_t;
pub type atomic_uintmax_t = uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct atomic_flag {
    pub __flag: atomic_bool,
}
extern "C" {
    pub fn bcmp(
        arg1: *const c_void,
        arg2: *const c_void,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn bcopy(
        arg1: *const c_void,
        arg2: *mut c_void,
        arg3: size_t,
    );
}
extern "C" {
    pub fn bzero(arg1: *mut c_void, arg2: c_ulong);
}
extern "C" {
    pub fn explicit_bzero(arg1: *mut c_void, arg2: size_t);
}
extern "C" {
    pub fn ffs(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn ffsl(arg1: c_long) -> c_int;
}
extern "C" {
    pub fn ffsll(arg1: c_longlong) -> c_int;
}
extern "C" {
    pub fn fls(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn flsl(arg1: c_long) -> c_int;
}
extern "C" {
    pub fn flsll(arg1: c_longlong) -> c_int;
}
extern "C" {
    pub fn index(
        arg1: *const c_char,
        arg2: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn rindex(
        arg1: *const c_char,
        arg2: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strcasecmp(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn strncasecmp(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: size_t,
        arg4: locale_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_atime: time_t,
    pub st_spare1: c_long,
    pub st_mtime: time_t,
    pub st_spare2: c_long,
    pub st_ctime: time_t,
    pub st_spare3: c_long,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_spare4: [c_long; 2usize],
}
extern "C" {
    pub fn fstat(__fd: c_int, __sbuf: *mut stat) -> c_int;
}
extern "C" {
    pub fn mkdir(_path: *const c_char, __mode: mode_t) -> c_int;
}
extern "C" {
    pub fn mkfifo(__path: *const c_char, __mode: mode_t) -> c_int;
}
extern "C" {
    pub fn stat(__path: *const c_char, __sbuf: *mut stat) -> c_int;
}
extern "C" {
    pub fn umask(__mask: mode_t) -> mode_t;
}
extern "C" {
    pub fn lstat(__path: *const c_char, __buf: *mut stat) -> c_int;
}
extern "C" {
    pub fn fchmodat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: mode_t,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn fstatat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *mut stat,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mkdirat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: mode_t,
    ) -> c_int;
}
extern "C" {
    pub fn mkfifoat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: mode_t,
    ) -> c_int;
}
extern "C" {
    pub fn mknodat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: mode_t,
        arg4: dev_t,
    ) -> c_int;
}
extern "C" {
    pub fn utimensat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const timespec,
        arg4: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn futimens(arg1: c_int, arg2: *const timespec) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct statvfs {
    pub f_bsize: c_ulong,
    pub f_frsize: c_ulong,
    pub f_blocks: fsblkcnt_t,
    pub f_bfree: fsblkcnt_t,
    pub f_bavail: fsblkcnt_t,
    pub f_files: fsfilcnt_t,
    pub f_ffree: fsfilcnt_t,
    pub f_favail: fsfilcnt_t,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
}
extern "C" {
    pub fn statvfs(path: *const c_char, buf: *mut statvfs)
        -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bintime {
    pub sec: time_t,
    pub frac: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
extern "C" {
    pub fn utimes(
        __path: *const c_char,
        __tvp: *const timeval,
    ) -> c_int;
}
extern "C" {
    pub fn adjtime(arg1: *const timeval, arg2: *mut timeval) -> c_int;
}
extern "C" {
    pub fn futimes(arg1: c_int, arg2: *const timeval) -> c_int;
}
extern "C" {
    pub fn lutimes(
        arg1: *const c_char,
        arg2: *const timeval,
    ) -> c_int;
}
extern "C" {
    pub fn settimeofday(arg1: *const timeval, arg2: *const timezone) -> c_int;
}
extern "C" {
    pub fn getitimer(
        __which: c_int,
        __value: *mut itimerval,
    ) -> c_int;
}
extern "C" {
    pub fn setitimer(
        __which: c_int,
        __value: *const itimerval,
        __ovalue: *mut itimerval,
    ) -> c_int;
}
extern "C" {
    pub fn gettimeofday(
        __p: *mut timeval,
        __tz: *mut c_void,
    ) -> c_int;
}
pub const STD_IN: _bindgen_ty_5 = 0;
pub const STD_OUT: _bindgen_ty_5 = 1;
pub const STD_ERR: _bindgen_ty_5 = 2;
pub const STD_MAX: _bindgen_ty_5 = 16;
pub type _bindgen_ty_5 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __handle {
    pub device: c_uint,
    pub refcount: c_uint,
    pub fileStruct: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DIR_ITER {
    pub device: c_int,
    pub dirStruct: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct devoptab_t {
    pub name: *const c_char,
    pub structSize: size_t,
    pub open_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fileStruct: *mut c_void,
            path: *const c_char,
            flags: c_int,
            mode: c_int,
        ) -> c_int,
    >,
    pub close_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
        ) -> c_int,
    >,
    pub write_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
            ptr: *const c_char,
            len: size_t,
        ) -> ssize_t,
    >,
    pub read_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
            ptr: *mut c_char,
            len: size_t,
        ) -> ssize_t,
    >,
    pub seek_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
            pos: off_t,
            dir: c_int,
        ) -> off_t,
    >,
    pub fstat_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
            st: *mut stat,
        ) -> c_int,
    >,
    pub stat_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            file: *const c_char,
            st: *mut stat,
        ) -> c_int,
    >,
    pub link_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            existing: *const c_char,
            newLink: *const c_char,
        ) -> c_int,
    >,
    pub unlink_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            name: *const c_char,
        ) -> c_int,
    >,
    pub chdir_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            name: *const c_char,
        ) -> c_int,
    >,
    pub rename_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            oldName: *const c_char,
            newName: *const c_char,
        ) -> c_int,
    >,
    pub mkdir_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            path: *const c_char,
            mode: c_int,
        ) -> c_int,
    >,
    pub dirStateSize: size_t,
    pub diropen_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            dirState: *mut DIR_ITER,
            path: *const c_char,
        ) -> *mut DIR_ITER,
    >,
    pub dirreset_r: ::Option<
        unsafe extern "C" fn(r: *mut _reent, dirState: *mut DIR_ITER) -> c_int,
    >,
    pub dirnext_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            dirState: *mut DIR_ITER,
            filename: *mut c_char,
            filestat: *mut stat,
        ) -> c_int,
    >,
    pub dirclose_r: ::Option<
        unsafe extern "C" fn(r: *mut _reent, dirState: *mut DIR_ITER) -> c_int,
    >,
    pub statvfs_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            path: *const c_char,
            buf: *mut statvfs,
        ) -> c_int,
    >,
    pub ftruncate_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
            len: off_t,
        ) -> c_int,
    >,
    pub fsync_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
        ) -> c_int,
    >,
    pub deviceData: *mut c_void,
    pub chmod_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            path: *const c_char,
            mode: mode_t,
        ) -> c_int,
    >,
    pub fchmod_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut c_void,
            mode: mode_t,
        ) -> c_int,
    >,
    pub rmdir_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            name: *const c_char,
        ) -> c_int,
    >,
    pub lstat_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            file: *const c_char,
            st: *mut stat,
        ) -> c_int,
    >,
    pub utimes_r: ::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            filename: *const c_char,
            times: *const timeval,
        ) -> c_int,
    >,
}
extern "C" {
    pub static mut devoptab_list: [*const devoptab_t; 0usize];
}
extern "C" {
    pub fn AddDevice(device: *const devoptab_t) -> c_int;
}
extern "C" {
    pub fn FindDevice(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn RemoveDevice(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn setDefaultDevice(device: c_int);
}
extern "C" {
    pub fn GetDeviceOpTab(name: *const c_char) -> *const devoptab_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DIR {
    pub position: c_long,
    pub dirData: *mut DIR_ITER,
    pub fileData: dirent,
}
extern "C" {
    pub fn closedir(dirp: *mut DIR) -> c_int;
}
extern "C" {
    pub fn opendir(dirname: *const c_char) -> *mut DIR;
}
extern "C" {
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
}
extern "C" {
    pub fn readdir_r(
        dirp: *mut DIR,
        entry: *mut dirent,
        result: *mut *mut dirent,
    ) -> c_int;
}
extern "C" {
    pub fn rewinddir(dirp: *mut DIR);
}
extern "C" {
    pub fn seekdir(dirp: *mut DIR, loc: c_long);
}
extern "C" {
    pub fn telldir(dirp: *mut DIR) -> c_long;
}
extern "C" {
    pub fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: ::Option<
            unsafe extern "C" fn(arg1: *const dirent) -> c_int,
        >,
        compar: ::Option<
            unsafe extern "C" fn(
                arg1: *mut *const dirent,
                arg2: *mut *const dirent,
            ) -> c_int,
        >,
    ) -> c_int;
}
extern "C" {
    pub fn alphasort(a: *mut *const dirent, b: *mut *const dirent) -> c_int;
}
extern "C" {
    pub fn dirfd(arg1: *mut DIR) -> c_int;
}
extern "C" {
    pub fn fdclosedir(arg1: *mut DIR) -> c_int;
}
extern "C" {
    pub fn fdopendir(arg1: c_int) -> *mut DIR;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: intmax_t,
    pub rem: intmax_t,
}
extern "C" {
    pub fn imaxabs(j: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(numer: intmax_t, denomer: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        arg1: *const wchar_t,
        arg2: *mut *mut wchar_t,
        arg3: c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        arg1: *const wchar_t,
        arg2: *mut *mut wchar_t,
        arg3: c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn strtoimax_l(
        arg1: *const c_char,
        _restrict: *mut *mut c_char,
        arg2: c_int,
        arg3: locale_t,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax_l(
        arg1: *const c_char,
        _restrict: *mut *mut c_char,
        arg2: c_int,
        arg3: locale_t,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax_l(
        arg1: *const wchar_t,
        _restrict: *mut *mut wchar_t,
        arg2: c_int,
        arg3: locale_t,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax_l(
        arg1: *const wchar_t,
        _restrict: *mut *mut wchar_t,
        arg2: c_int,
        arg3: locale_t,
    ) -> uintmax_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lconv {
    pub decimal_point: *mut c_char,
    pub thousands_sep: *mut c_char,
    pub grouping: *mut c_char,
    pub int_curr_symbol: *mut c_char,
    pub currency_symbol: *mut c_char,
    pub mon_decimal_point: *mut c_char,
    pub mon_thousands_sep: *mut c_char,
    pub mon_grouping: *mut c_char,
    pub positive_sign: *mut c_char,
    pub negative_sign: *mut c_char,
    pub int_frac_digits: c_char,
    pub frac_digits: c_char,
    pub p_cs_precedes: c_char,
    pub p_sep_by_space: c_char,
    pub n_cs_precedes: c_char,
    pub n_sep_by_space: c_char,
    pub p_sign_posn: c_char,
    pub n_sign_posn: c_char,
    pub int_n_cs_precedes: c_char,
    pub int_n_sep_by_space: c_char,
    pub int_n_sign_posn: c_char,
    pub int_p_cs_precedes: c_char,
    pub int_p_sep_by_space: c_char,
    pub int_p_sign_posn: c_char,
}
extern "C" {
    pub fn setlocale(
        arg1: c_int,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn localeconv() -> *mut lconv;
}
extern "C" {
    pub fn newlocale(
        arg1: c_int,
        arg2: *const c_char,
        arg3: locale_t,
    ) -> locale_t;
}
extern "C" {
    pub fn freelocale(arg1: locale_t);
}
extern "C" {
    pub fn duplocale(arg1: locale_t) -> locale_t;
}
extern "C" {
    pub fn uselocale(arg1: locale_t) -> locale_t;
}
pub type regoff_t = off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct regex_t {
    pub re_magic: c_int,
    pub re_nsub: size_t,
    pub re_endp: *const c_char,
    pub re_g: *mut re_guts,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
extern "C" {
    pub fn regcomp(
        arg1: *mut regex_t,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn regerror(
        arg1: c_int,
        arg2: *const regex_t,
        arg3: *mut c_char,
        arg4: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn regexec(
        arg1: *const regex_t,
        arg2: *const c_char,
        arg3: size_t,
        arg4: *mut regmatch_t,
        arg5: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn regfree(arg1: *mut regex_t);
}
pub type wctype_t = c_int;
pub type wctrans_t = c_int;
extern "C" {
    pub fn iswalpha(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswalnum(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswblank(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswcntrl(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswctype(arg1: wint_t, arg2: wctype_t) -> c_int;
}
extern "C" {
    pub fn iswdigit(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswgraph(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswlower(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswprint(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswpunct(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswspace(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswupper(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn iswxdigit(arg1: wint_t) -> c_int;
}
extern "C" {
    pub fn towctrans(arg1: wint_t, arg2: wctrans_t) -> wint_t;
}
extern "C" {
    pub fn towupper(arg1: wint_t) -> wint_t;
}
extern "C" {
    pub fn towlower(arg1: wint_t) -> wint_t;
}
extern "C" {
    pub fn wctrans(arg1: *const c_char) -> wctrans_t;
}
extern "C" {
    pub fn wctype(arg1: *const c_char) -> wctype_t;
}
extern "C" {
    pub fn iswalpha_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswalnum_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswblank_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswcntrl_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswctype_l(arg1: wint_t, arg2: wctype_t, arg3: locale_t) -> c_int;
}
extern "C" {
    pub fn iswdigit_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswgraph_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswlower_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswprint_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswpunct_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswspace_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswupper_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iswxdigit_l(arg1: wint_t, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn towctrans_l(arg1: wint_t, arg2: wctrans_t, arg3: locale_t) -> wint_t;
}
extern "C" {
    pub fn towupper_l(arg1: wint_t, arg2: locale_t) -> wint_t;
}
extern "C" {
    pub fn towlower_l(arg1: wint_t, arg2: locale_t) -> wint_t;
}
extern "C" {
    pub fn wctrans_l(arg1: *const c_char, arg2: locale_t) -> wctrans_t;
}
extern "C" {
    pub fn wctype_l(arg1: *const c_char, arg2: locale_t) -> wctype_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: c_int,
    pub rem: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: c_long,
    pub rem: c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: c_longlong,
    pub rem: c_longlong,
}
pub type __compar_fn_t = ::Option<
    unsafe extern "C" fn(
        arg1: *const c_void,
        arg2: *const c_void,
    ) -> c_int,
>;
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn abs(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn arc4random() -> __uint32_t;
}
extern "C" {
    pub fn arc4random_uniform(arg1: __uint32_t) -> __uint32_t;
}
extern "C" {
    pub fn arc4random_buf(arg1: *mut c_void, arg2: size_t);
}
extern "C" {
    pub fn atexit(__func: ::Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn atof(__nptr: *const c_char) -> f64;
}
extern "C" {
    pub fn atoff(__nptr: *const c_char) -> f32;
}
extern "C" {
    pub fn atoi(__nptr: *const c_char) -> c_int;
}
extern "C" {
    pub fn atol(__nptr: *const c_char) -> c_long;
}
extern "C" {
    pub fn bsearch(
        __key: *const c_void,
        __base: *const c_void,
        __nmemb: size_t,
        __size: size_t,
        _compar: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn calloc(
        arg1: c_ulong,
        arg2: c_ulong,
    ) -> *mut c_void;
}
extern "C" {
    pub fn div(__numer: c_int, __denom: c_int) -> div_t;
}
extern "C" {
    pub fn exit(__status: c_int);
}
extern "C" {
    pub fn free(arg1: *mut c_void);
}
extern "C" {
    pub fn getenv(__string: *const c_char) -> *mut c_char;
}
extern "C" {
    pub static mut suboptarg: *mut c_char;
}
extern "C" {
    pub fn getsubopt(
        arg1: *mut *mut c_char,
        arg2: *const *mut c_char,
        arg3: *mut *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn labs(arg1: c_long) -> c_long;
}
extern "C" {
    pub fn ldiv(__numer: c_long, __denom: c_long) -> ldiv_t;
}
extern "C" {
    pub fn malloc(arg1: c_ulong) -> *mut c_void;
}
extern "C" {
    pub fn mblen(arg1: *const c_char, arg2: size_t) -> c_int;
}
extern "C" {
    pub fn mbtowc(
        arg1: *mut wchar_t,
        arg2: *const c_char,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn wctomb(arg1: *mut c_char, arg2: wchar_t) -> c_int;
}
extern "C" {
    pub fn mbstowcs(
        arg1: *mut wchar_t,
        arg2: *const c_char,
        arg3: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcstombs(
        arg1: *mut c_char,
        arg2: *const wchar_t,
        arg3: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn mkdtemp(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn mkstemp(arg1: *mut c_char) -> c_int;
}
extern "C" {
    pub fn mkstemps(
        arg1: *mut c_char,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn mktemp(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn qsort(
        __base: *mut c_void,
        __nmemb: size_t,
        __size: size_t,
        _compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn rand() -> c_int;
}
extern "C" {
    pub fn realloc(
        arg1: *mut c_void,
        arg2: c_ulong,
    ) -> *mut c_void;
}
extern "C" {
    pub fn reallocarray(
        arg1: *mut c_void,
        arg2: size_t,
        arg3: size_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn reallocf(arg1: *mut c_void, arg2: size_t)
        -> *mut c_void;
}
extern "C" {
    pub fn realpath(
        path: *const c_char,
        resolved_path: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn rpmatch(response: *const c_char) -> c_int;
}
extern "C" {
    pub fn srand(__seed: c_uint);
}
extern "C" {
    pub fn strtod(
        __n: *const c_char,
        __end_PTR: *mut *mut c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __n: *const c_char,
        __end_PTR: *mut *mut c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtol(
        __n: *const c_char,
        __end_PTR: *mut *mut c_char,
        __base: c_int,
    ) -> c_long;
}
extern "C" {
    pub fn strtoul(
        __n: *const c_char,
        __end_PTR: *mut *mut c_char,
        __base: c_int,
    ) -> c_ulong;
}
extern "C" {
    pub fn system(__string: *const c_char) -> c_int;
}
extern "C" {
    pub fn a64l(__input: *const c_char) -> c_long;
}
extern "C" {
    pub fn l64a(__input: c_long) -> *mut c_char;
}
extern "C" {
    pub fn on_exit(
        __func: ::Option<
            unsafe extern "C" fn(arg1: c_int, arg2: *mut c_void),
        >,
        __arg: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn putenv(__string: *mut c_char) -> c_int;
}
extern "C" {
    pub fn setenv(
        __string: *const c_char,
        __value: *const c_char,
        __overwrite: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn itoa(
        arg1: c_int,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn utoa(
        arg1: c_uint,
        arg2: *mut c_char,
        arg3: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn rand_r(__seed: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(arg1: *mut c_ushort) -> f64;
}
extern "C" {
    pub fn jrand48(arg1: *mut c_ushort) -> c_long;
}
extern "C" {
    pub fn lcong48(arg1: *mut c_ushort);
}
extern "C" {
    pub fn lrand48() -> c_long;
}
extern "C" {
    pub fn mrand48() -> c_long;
}
extern "C" {
    pub fn nrand48(arg1: *mut c_ushort) -> c_long;
}
extern "C" {
    pub fn seed48(arg1: *mut c_ushort) -> *mut c_ushort;
}
extern "C" {
    pub fn srand48(arg1: c_long);
}
extern "C" {
    pub fn initstate(
        arg1: c_uint,
        arg2: *mut c_char,
        arg3: size_t,
    ) -> *mut c_char;
}
extern "C" {
    pub fn random() -> c_long;
}
extern "C" {
    pub fn setstate(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn srandom(arg1: c_uint);
}
extern "C" {
    pub fn atoll(__nptr: *const c_char) -> c_longlong;
}
extern "C" {
    pub fn llabs(arg1: c_longlong) -> c_longlong;
}
extern "C" {
    pub fn lldiv(
        __numer: c_longlong,
        __denom: c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn strtoll(
        __n: *const c_char,
        __end_PTR: *mut *mut c_char,
        __base: c_int,
    ) -> c_longlong;
}
extern "C" {
    pub fn strtoull(
        __n: *const c_char,
        __end_PTR: *mut *mut c_char,
        __base: c_int,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn cfree(arg1: *mut c_void);
}
extern "C" {
    pub fn unsetenv(__string: *const c_char) -> c_int;
}
extern "C" {
    pub fn posix_memalign(
        arg1: *mut *mut c_void,
        arg2: size_t,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}__bsd_qsort_r"]
    pub fn qsort_r(
        __base: *mut c_void,
        __nmemb: size_t,
        __size: size_t,
        __thunk: *mut c_void,
        _compar: ::Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *const c_void,
                arg3: *const c_void,
            ) -> c_int,
        >,
    );
}
extern "C" {
    pub fn strtold(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
    ) -> u128;
}
extern "C" {
    pub fn aligned_alloc(arg1: size_t, arg2: size_t) -> *mut c_void;
}
extern "C" {
    pub fn at_quick_exit(
        arg1: ::Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
extern "C" {
    pub fn quick_exit(arg1: c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ar_hdr {
    pub ar_name: [c_char; 16usize],
    pub ar_date: [c_char; 12usize],
    pub ar_uid: [c_char; 6usize],
    pub ar_gid: [c_char; 6usize],
    pub ar_mode: [c_char; 8usize],
    pub ar_size: [c_char; 10usize],
    pub ar_fmag: [c_char; 2usize],
}
pub type error_t = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
extern "C" {
    pub fn utime(
        path: *const c_char,
        times: *const utimbuf,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glob_t {
    pub gl_pathc: c_int,
    pub gl_matchc: c_int,
    pub gl_offs: c_int,
    pub gl_flags: c_int,
    pub gl_pathv: *mut *mut c_char,
    pub gl_errfunc: ::Option<
        unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: c_int,
        ) -> c_int,
    >,
    pub gl_closedir: ::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub gl_readdir: ::Option<
        unsafe extern "C" fn(arg1: *mut c_void) -> *mut dirent,
    >,
    pub gl_opendir: ::Option<
        unsafe extern "C" fn(arg1: *const c_char) -> *mut c_void,
    >,
    pub gl_lstat: ::Option<
        unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *mut stat,
        ) -> c_int,
    >,
    pub gl_stat: ::Option<
        unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *mut stat,
        ) -> c_int,
    >,
}
extern "C" {
    pub fn glob(
        arg1: *const c_char,
        arg2: c_int,
        arg3: ::Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: c_int,
            ) -> c_int,
        >,
        arg4: *mut glob_t,
    ) -> c_int;
}
extern "C" {
    pub fn globfree(arg1: *mut glob_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_comment: *mut c_char,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}
extern "C" {
    pub fn getpwuid(arg1: uid_t) -> *mut passwd;
}
extern "C" {
    pub fn getpwnam(arg1: *const c_char) -> *mut passwd;
}
extern "C" {
    pub fn getpwnam_r(
        arg1: *const c_char,
        arg2: *mut passwd,
        arg3: *mut c_char,
        arg4: size_t,
        arg5: *mut *mut passwd,
    ) -> c_int;
}
extern "C" {
    pub fn getpwuid_r(
        arg1: uid_t,
        arg2: *mut passwd,
        arg3: *mut c_char,
        arg4: size_t,
        arg5: *mut *mut passwd,
    ) -> c_int;
}
extern "C" {
    pub fn getpwent() -> *mut passwd;
}
extern "C" {
    pub fn setpwent();
}
extern "C" {
    pub fn endpwent();
}
extern "C" {
    pub fn setpassent(arg1: c_int) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}
extern "C" {
    pub fn getopt_long(
        __argc: c_int,
        __argv: *const *mut c_char,
        __shortopts: *const c_char,
        __longopts: *const option,
        __longind: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn getopt_long_only(
        __argc: c_int,
        __argv: *const *mut c_char,
        __shortopts: *const c_char,
        __longopts: *const option,
        __longind: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn envz_entry(
        envz: *const c_char,
        envz_len: size_t,
        name: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn envz_get(
        envz: *const c_char,
        envz_len: size_t,
        name: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn envz_add(
        envz: *mut *mut c_char,
        envz_len: *mut size_t,
        name: *const c_char,
        value: *const c_char,
    ) -> error_t;
}
extern "C" {
    pub fn envz_merge(
        envz: *mut *mut c_char,
        envz_len: *mut size_t,
        envz2: *const c_char,
        envz2_len: size_t,
        override_: c_int,
    ) -> error_t;
}
extern "C" {
    pub fn envz_remove(
        envz: *mut *mut c_char,
        envz_len: *mut size_t,
        name: *const c_char,
    );
}
extern "C" {
    pub fn envz_strip(envz: *mut *mut c_char, envz_len: *mut size_t);
}
extern "C" {
    pub fn argz_create(
        argv: *const *mut c_char,
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
    ) -> error_t;
}
extern "C" {
    pub fn argz_create_sep(
        string: *const c_char,
        sep: c_int,
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
    ) -> error_t;
}
extern "C" {
    pub fn argz_count(argz: *const c_char, argz_len: size_t) -> size_t;
}
extern "C" {
    pub fn argz_extract(
        argz: *mut c_char,
        argz_len: size_t,
        argv: *mut *mut c_char,
    );
}
extern "C" {
    pub fn argz_stringify(
        argz: *mut c_char,
        argz_len: size_t,
        sep: c_int,
    );
}
extern "C" {
    pub fn argz_add(
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
        str: *const c_char,
    ) -> error_t;
}
extern "C" {
    pub fn argz_add_sep(
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
        str: *const c_char,
        sep: c_int,
    ) -> error_t;
}
extern "C" {
    pub fn argz_append(
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
        buf: *const c_char,
        buf_len: size_t,
    ) -> error_t;
}
extern "C" {
    pub fn argz_delete(
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
        entry: *mut c_char,
    ) -> error_t;
}
extern "C" {
    pub fn argz_insert(
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
        before: *mut c_char,
        entry: *const c_char,
    ) -> error_t;
}
extern "C" {
    pub fn argz_next(
        argz: *mut c_char,
        argz_len: size_t,
        entry: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn argz_replace(
        argz: *mut *mut c_char,
        argz_len: *mut size_t,
        str: *const c_char,
        with: *const c_char,
        replace_count: *mut c_uint,
    ) -> error_t;
}
pub type fpos_t = _fpos_t;
extern "C" {
    pub fn ctermid(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn tempnam(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn fclose(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fflush(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn freopen(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(arg1: *mut FILE, arg2: *mut c_char);
}
extern "C" {
    pub fn setvbuf(
        arg1: *mut FILE,
        arg2: *mut c_char,
        arg3: c_int,
        arg4: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn fprintf(
        arg1: *mut FILE,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn fscanf(
        arg1: *mut FILE,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn printf(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn scanf(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn sscanf(
        arg1: *const c_char,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn vfprintf(
        arg1: *mut FILE,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vprintf(
        arg1: *const c_char,
        arg2: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vsprintf(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn fgetc(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fgets(
        arg1: *mut c_char,
        arg2: c_int,
        arg3: *mut FILE,
    ) -> *mut c_char;
}
extern "C" {
    pub fn fputc(arg1: c_int, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fputs(arg1: *const c_char, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getc(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getchar() -> c_int;
}
extern "C" {
    pub fn gets(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn putc(arg1: c_int, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putchar(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn puts(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn ungetc(arg1: c_int, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fread(
        arg1: *mut c_void,
        _size: c_ulong,
        _n: c_ulong,
        arg2: *mut FILE,
    ) -> c_ulong;
}
extern "C" {
    pub fn fwrite(
        arg1: *const c_void,
        _size: c_ulong,
        _n: c_ulong,
        arg2: *mut FILE,
    ) -> c_ulong;
}
extern "C" {
    pub fn fgetpos(arg1: *mut FILE, arg2: *mut fpos_t) -> c_int;
}
extern "C" {
    pub fn fseek(
        arg1: *mut FILE,
        arg2: c_long,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn fsetpos(arg1: *mut FILE, arg2: *const fpos_t) -> c_int;
}
extern "C" {
    pub fn ftell(arg1: *mut FILE) -> c_long;
}
extern "C" {
    pub fn rewind(arg1: *mut FILE);
}
extern "C" {
    pub fn clearerr(arg1: *mut FILE);
}
extern "C" {
    pub fn feof(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn ferror(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn perror(arg1: *const c_char);
}
extern "C" {
    pub fn fopen(
        _name: *const c_char,
        _type: *const c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn sprintf(
        arg1: *mut c_char,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn remove(arg1: *const c_char) -> c_int;
}
extern "C" {
    pub fn rename(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn fseeko(
        arg1: *mut FILE,
        arg2: off_t,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn ftello(arg1: *mut FILE) -> off_t;
}
extern "C" {
    pub fn snprintf(
        arg1: *mut c_char,
        arg2: c_ulong,
        arg3: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn vsnprintf(
        arg1: *mut c_char,
        arg2: c_ulong,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vfscanf(
        arg1: *mut FILE,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vscanf(
        arg1: *const c_char,
        arg2: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vsscanf(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn asiprintf(
        arg1: *mut *mut c_char,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn asniprintf(
        arg1: *mut c_char,
        arg2: *mut size_t,
        arg3: *const c_char,
        ...
    ) -> *mut c_char;
}
extern "C" {
    pub fn asnprintf(
        arg1: *mut c_char,
        arg2: *mut size_t,
        arg3: *const c_char,
        ...
    ) -> *mut c_char;
}
extern "C" {
    pub fn diprintf(
        arg1: c_int,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn fiprintf(
        arg1: *mut FILE,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn fiscanf(
        arg1: *mut FILE,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn iprintf(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn iscanf(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn siprintf(
        arg1: *mut c_char,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn siscanf(
        arg1: *const c_char,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn sniprintf(
        arg1: *mut c_char,
        arg2: size_t,
        arg3: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn vasiprintf(
        arg1: *mut *mut c_char,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vasniprintf(
        arg1: *mut c_char,
        arg2: *mut size_t,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> *mut c_char;
}
extern "C" {
    pub fn vasnprintf(
        arg1: *mut c_char,
        arg2: *mut size_t,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> *mut c_char;
}
extern "C" {
    pub fn vdiprintf(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vfiprintf(
        arg1: *mut FILE,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vfiscanf(
        arg1: *mut FILE,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn viprintf(
        arg1: *const c_char,
        arg2: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn viscanf(
        arg1: *const c_char,
        arg2: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vsiprintf(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vsiscanf(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vsniprintf(
        arg1: *mut c_char,
        arg2: size_t,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn fdopen(arg1: c_int, arg2: *const c_char) -> *mut FILE;
}
extern "C" {
    pub fn fileno(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn pclose(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn popen(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuffer(
        arg1: *mut FILE,
        arg2: *mut c_char,
        arg3: c_int,
    );
}
extern "C" {
    pub fn setlinebuf(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getw(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putw(arg1: c_int, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getc_unlocked(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> c_int;
}
extern "C" {
    pub fn flockfile(arg1: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn funlockfile(arg1: *mut FILE);
}
extern "C" {
    pub fn putc_unlocked(arg1: c_int, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putchar_unlocked(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn dprintf(
        arg1: c_int,
        arg2: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn fmemopen(
        arg1: *mut c_void,
        arg2: size_t,
        arg3: *const c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(arg1: *mut *mut c_char, arg2: *mut size_t) -> *mut FILE;
}
extern "C" {
    pub fn vdprintf(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn renameat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: c_int,
        arg4: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn fpurge(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn clearerr_unlocked(arg1: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn ferror_unlocked(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fileno_unlocked(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fflush_unlocked(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fgetc_unlocked(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fputc_unlocked(arg1: c_int, arg2: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fread_unlocked(
        arg1: *mut c_void,
        _size: size_t,
        _n: size_t,
        arg2: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fwrite_unlocked(
        arg1: *const c_void,
        _size: size_t,
        _n: size_t,
        arg2: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn funopen(
        __cookie: *const c_void,
        __readfn: ::Option<
            unsafe extern "C" fn(
                __cookie: *mut c_void,
                __buf: *mut c_char,
                __n: c_int,
            ) -> c_int,
        >,
        __writefn: ::Option<
            unsafe extern "C" fn(
                __cookie: *mut c_void,
                __buf: *const c_char,
                __n: c_int,
            ) -> c_int,
        >,
        __seekfn: ::Option<
            unsafe extern "C" fn(
                __cookie: *mut c_void,
                __off: fpos_t,
                __whence: c_int,
            ) -> fpos_t,
        >,
        __closefn: ::Option<
            unsafe extern "C" fn(__cookie: *mut c_void) -> c_int,
        >,
    ) -> *mut FILE;
}
extern "C" {
    pub fn memchr(
        arg1: *const c_void,
        arg2: c_int,
        arg3: c_ulong,
    ) -> *mut c_void;
}
extern "C" {
    pub fn memcmp(
        arg1: *const c_void,
        arg2: *const c_void,
        arg3: c_ulong,
    ) -> c_int;
}
extern "C" {
    pub fn memcpy(
        arg1: *mut c_void,
        arg2: *const c_void,
        arg3: c_ulong,
    ) -> *mut c_void;
}
extern "C" {
    pub fn memmove(
        arg1: *mut c_void,
        arg2: *const c_void,
        arg3: c_ulong,
    ) -> *mut c_void;
}
extern "C" {
    pub fn memset(
        arg1: *mut c_void,
        arg2: c_int,
        arg3: c_ulong,
    ) -> *mut c_void;
}
extern "C" {
    pub fn strcat(
        arg1: *mut c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strchr(
        arg1: *const c_char,
        arg2: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strcmp(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn strcoll(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn strcpy(
        arg1: *mut c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strcspn(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_ulong;
}
extern "C" {
    pub fn strerror(arg1: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strlen(arg1: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn strncat(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strncmp(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_int;
}
extern "C" {
    pub fn strncpy(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strpbrk(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strrchr(
        arg1: *const c_char,
        arg2: c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strspn(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_ulong;
}
extern "C" {
    pub fn strstr(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strtok(
        arg1: *mut c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strxfrm(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    pub fn strcoll_l(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn strerror_l(arg1: c_int, arg2: locale_t) -> *mut c_char;
}
extern "C" {
    pub fn strxfrm_l(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: size_t,
        arg4: locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn strtok_r(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: *mut *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn timingsafe_bcmp(
        arg1: *const c_void,
        arg2: *const c_void,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn timingsafe_memcmp(
        arg1: *const c_void,
        arg2: *const c_void,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn memccpy(
        arg1: *mut c_void,
        arg2: *const c_void,
        arg3: c_int,
        arg4: size_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn stpcpy(
        arg1: *mut c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn stpncpy(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strdup(arg1: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strndup(
        arg1: *const c_char,
        arg2: c_ulong,
    ) -> *mut c_char;
}
extern "C" {
    #[link_name = "\u{1}__xpg_strerror_r"]
    pub fn strerror_r(
        arg1: c_int,
        arg2: *mut c_char,
        arg3: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn strlcat(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    pub fn strlcpy(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    pub fn strnlen(arg1: *const c_char, arg2: size_t) -> size_t;
}
extern "C" {
    pub fn strsep(
        arg1: *mut *mut c_char,
        arg2: *const c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strnstr(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: size_t,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strlwr(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn strupr(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn strsignal(__signo: c_int) -> *mut c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ctlname {
    pub ctl_name: *mut c_char,
    pub ctl_type: c_int,
}
extern "C" {
    pub fn sysctl(
        name: *const c_int,
        namelen: c_uint,
        oldp: *mut c_void,
        oldlenp: *mut size_t,
        newp: *const c_void,
        newlen: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut size_t,
        newp: *const c_void,
        newlen: size_t,
    ) -> c_int;
}
extern "C" {
    pub fn sysctlnametomib(
        name: *const c_char,
        mibp: *mut c_int,
        sizep: *mut size_t,
    ) -> c_int;
}
extern "C" {
    pub fn wait(arg1: *mut c_int) -> pid_t;
}
extern "C" {
    pub fn waitpid(
        arg1: pid_t,
        arg2: *mut c_int,
        arg3: c_int,
    ) -> pid_t;
}
extern "C" {
    pub fn ioctl(
        fd: c_int,
        request: c_int,
        ...
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
extern "C" {
    pub fn times(arg1: *mut tms) -> clock_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: c_long,
    pub l_len: c_long,
    pub l_pid: c_short,
    pub l_xxx: c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct eflock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: c_long,
    pub l_len: c_long,
    pub l_pid: c_short,
    pub l_xxx: c_short,
    pub l_rpid: c_long,
    pub l_rsys: c_long,
}
extern "C" {
    pub fn open(
        arg1: *const c_char,
        arg2: c_int,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn openat(
        arg1: c_int,
        arg2: *const c_char,
        arg3: c_int,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn creat(arg1: *const c_char, arg2: mode_t) -> c_int;
}
extern "C" {
    pub fn fcntl(
        arg1: c_int,
        arg2: c_int,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn flock(arg1: c_int, arg2: c_int)
        -> c_int;
}
pub type iconv_t = _iconv_t;
extern "C" {
    pub fn iconv_open(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> iconv_t;
}
extern "C" {
    pub fn iconv(
        arg1: iconv_t,
        arg2: *mut *mut c_char,
        arg3: *mut size_t,
        arg4: *mut *mut c_char,
        arg5: *mut size_t,
    ) -> size_t;
}
extern "C" {
    pub fn iconv_close(arg1: iconv_t) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}
pub type sa_family_t = __sa_family_t;
pub type socklen_t = __socklen_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct accept_filter_arg {
    pub af_name: [c_char; 16usize],
    pub af_arg: [c_char; 240usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_len: c_uchar,
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockproto {
    pub sp_family: c_ushort,
    pub sp_protocol: c_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_len: c_uchar,
    pub ss_family: sa_family_t,
    pub __ss_pad1: [c_char; 6usize],
    pub __ss_align: i64,
    pub __ss_pad2: [c_char; 112usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_control: *mut c_void,
    pub msg_controllen: socklen_t,
    pub msg_flags: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmsghdr {
    pub cmsg_len: socklen_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sock_timestamp_info {
    pub st_info_flags: u32,
    pub st_info_pad0: u32,
    pub st_info_rsv: [u64; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osockaddr {
    pub sa_family: c_ushort,
    pub sa_data: [c_char; 14usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct omsghdr {
    pub msg_name: *mut c_char,
    pub msg_namelen: c_int,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_accrights: *mut c_char,
    pub msg_accrightslen: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sf_hdtr {
    pub headers: *mut iovec,
    pub hdr_cnt: c_int,
    pub trailers: *mut iovec,
    pub trl_cnt: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: ssize_t,
}
extern "C" {
    pub fn socket(
        domain: c_int,
        type_: c_int,
        protocol: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn recv(
        sockfd: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn send(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn accept(
        sockfd: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn bind(
        sockfd: c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn connect(
        sockfd: c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn getpeername(
        sockfd: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn getsockname(
        sockfd: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn listen(
        sockfd: c_int,
        backlog: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn shutdown(
        sockfd: c_int,
        how: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn sockatmark(sockfd: c_int) -> c_int;
}
extern "C" {
    pub fn socketpair(
        domain: c_int,
        type_: c_int,
        protocol: c_int,
        sv: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn recvmsg(
        sockfd: c_int,
        msg: *mut msghdr,
        flags: c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn sendmsg(
        sockfd: c_int,
        msg: *const msghdr,
        flags: c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn sendmmsg(
        sockfd: c_int,
        msgvec: *mut mmsghdr,
        vlen: c_uint,
        flags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn recvmmsg(
        sockfd: c_int,
        msgvec: *mut mmsghdr,
        vlen: c_uint,
        flags: c_int,
        timeout: *mut timespec,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
}
extern "C" {
    pub fn getrusage(arg1: c_int, arg2: *mut rusage) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeb {
    pub time: time_t,
    pub millitm: c_ushort,
    pub timezone: c_short,
    pub dstflag: c_short,
}
extern "C" {
    pub fn ftime(arg1: *mut timeb) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fiodgname_arg {
    pub len: c_int,
    pub buf: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: c_int,
    pub sigev_signo: c_int,
    pub sigev_value: sigval,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_code: c_int,
    pub si_value: sigval,
}
pub type _sig_func_ptr = ::Option<unsafe extern "C" fn(arg1: c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigaction {
    pub sa_handler: _sig_func_ptr,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigaltstack {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: size_t,
}
pub type stack_t = sigaltstack;
extern "C" {
    pub fn sigprocmask(
        arg1: c_int,
        arg2: *const sigset_t,
        arg3: *mut sigset_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_sigmask(
        arg1: c_int,
        arg2: *const sigset_t,
        arg3: *mut sigset_t,
    ) -> c_int;
}
extern "C" {
    pub fn kill(arg1: pid_t, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn killpg(arg1: pid_t, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn sigaction(
        arg1: c_int,
        arg2: *const sigaction,
        arg3: *mut sigaction,
    ) -> c_int;
}
extern "C" {
    pub fn sigaddset(arg1: *mut sigset_t, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn sigdelset(arg1: *mut sigset_t, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn sigismember(arg1: *const sigset_t, arg2: c_int)
        -> c_int;
}
extern "C" {
    pub fn sigfillset(arg1: *mut sigset_t) -> c_int;
}
extern "C" {
    pub fn sigemptyset(arg1: *mut sigset_t) -> c_int;
}
extern "C" {
    pub fn sigpending(arg1: *mut sigset_t) -> c_int;
}
extern "C" {
    pub fn sigsuspend(arg1: *const sigset_t) -> c_int;
}
extern "C" {
    pub fn sigwait(
        arg1: *const sigset_t,
        arg2: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn sigpause(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn sigaltstack(arg1: *const stack_t, arg2: *mut stack_t) -> c_int;
}
extern "C" {
    pub fn pthread_kill(arg1: pthread_t, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn sigwaitinfo(arg1: *const sigset_t, arg2: *mut siginfo_t) -> c_int;
}
extern "C" {
    pub fn sigtimedwait(
        arg1: *const sigset_t,
        arg2: *mut siginfo_t,
        arg3: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn sigqueue(
        arg1: pid_t,
        arg2: c_int,
        arg3: sigval,
    ) -> c_int;
}
pub type sig_atomic_t = c_int;
pub type sig_t = _sig_func_ptr;
extern "C" {
    pub fn signal(arg1: c_int, arg2: _sig_func_ptr) -> _sig_func_ptr;
}
extern "C" {
    pub fn raise(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn psignal(arg1: c_int, arg2: *const c_char);
}
pub type nfds_t = c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
extern "C" {
    pub fn poll(
        pfd: *mut pollfd,
        nfds: nfds_t,
        timeout: c_int,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __posix_spawnattr {
    _unused: [u8; 0],
}
pub type posix_spawnattr_t = *mut __posix_spawnattr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __posix_spawn_file_actions {
    _unused: [u8; 0],
}
pub type posix_spawn_file_actions_t = *mut __posix_spawn_file_actions;
extern "C" {
    pub fn posix_spawn(
        arg1: *mut pid_t,
        arg2: *const c_char,
        arg3: *const posix_spawn_file_actions_t,
        arg4: *const posix_spawnattr_t,
        arg5: *const *mut c_char,
        arg6: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnp(
        arg1: *mut pid_t,
        arg2: *const c_char,
        arg3: *const posix_spawn_file_actions_t,
        arg4: *const posix_spawnattr_t,
        arg5: *const *mut c_char,
        arg6: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawn_file_actions_init(
        arg1: *mut posix_spawn_file_actions_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawn_file_actions_destroy(
        arg1: *mut posix_spawn_file_actions_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawn_file_actions_addopen(
        arg1: *mut posix_spawn_file_actions_t,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
        arg5: mode_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawn_file_actions_adddup2(
        arg1: *mut posix_spawn_file_actions_t,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawn_file_actions_addclose(
        arg1: *mut posix_spawn_file_actions_t,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_init(arg1: *mut posix_spawnattr_t) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_destroy(arg1: *mut posix_spawnattr_t) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_getflags(
        arg1: *const posix_spawnattr_t,
        arg2: *mut c_short,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_getpgroup(
        arg1: *const posix_spawnattr_t,
        arg2: *mut pid_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_getschedparam(
        arg1: *const posix_spawnattr_t,
        arg2: *mut sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_getschedpolicy(
        arg1: *const posix_spawnattr_t,
        arg2: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_getsigdefault(
        arg1: *const posix_spawnattr_t,
        arg2: *mut sigset_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_getsigmask(
        arg1: *const posix_spawnattr_t,
        arg2: *mut sigset_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_setflags(
        arg1: *mut posix_spawnattr_t,
        arg2: c_short,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_setpgroup(
        arg1: *mut posix_spawnattr_t,
        arg2: pid_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_setschedparam(
        arg1: *mut posix_spawnattr_t,
        arg2: *const sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_setschedpolicy(
        arg1: *mut posix_spawnattr_t,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_setsigdefault(
        arg1: *mut posix_spawnattr_t,
        arg2: *const sigset_t,
    ) -> c_int;
}
extern "C" {
    pub fn posix_spawnattr_setsigmask(
        arg1: *mut posix_spawnattr_t,
        arg2: *const sigset_t,
    ) -> c_int;
}
extern "C" {
    pub fn dirname(arg1: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn longjmp(__jmpb: *mut c_longlong, __retval: c_int);
}
extern "C" {
    pub fn setjmp(__jmpb: *mut c_longlong) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct entry {
    pub key: *mut c_char,
    pub data: *mut c_void,
}
pub type ENTRY = entry;
pub const ACTION_FIND: ACTION = 0;
pub const ACTION_ENTER: ACTION = 1;
pub type ACTION = u32;
pub const VISIT_preorder: VISIT = 0;
pub const VISIT_postorder: VISIT = 1;
pub const VISIT_endorder: VISIT = 2;
pub const VISIT_leaf: VISIT = 3;
pub type VISIT = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsearch_data {
    pub htable: *mut internal_head,
    pub htablesize: size_t,
}
extern "C" {
    pub fn hcreate(arg1: size_t) -> c_int;
}
extern "C" {
    pub fn hdestroy();
}
extern "C" {
    pub fn hsearch(arg1: ENTRY, arg2: ACTION) -> *mut ENTRY;
}
extern "C" {
    pub fn hcreate_r(arg1: size_t, arg2: *mut hsearch_data) -> c_int;
}
extern "C" {
    pub fn hdestroy_r(arg1: *mut hsearch_data);
}
extern "C" {
    pub fn hsearch_r(
        arg1: ENTRY,
        arg2: ACTION,
        arg3: *mut *mut ENTRY,
        arg4: *mut hsearch_data,
    ) -> c_int;
}
extern "C" {
    pub fn tdelete(
        arg1: *const c_void,
        arg2: *mut *mut c_void,
        arg3: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn tdestroy(
        arg1: *mut c_void,
        arg2: ::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    );
}
extern "C" {
    pub fn tfind(
        arg1: *const c_void,
        arg2: *mut *mut c_void,
        arg3: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn tsearch(
        arg1: *const c_void,
        arg2: *mut *mut c_void,
        arg3: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn twalk(
        arg1: *const c_void,
        arg2: ::Option<
            unsafe extern "C" fn(
                arg1: *const c_void,
                arg2: VISIT,
                arg3: c_int,
            ),
        >,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mallinfo {
    pub arena: size_t,
    pub ordblks: size_t,
    pub smblks: size_t,
    pub hblks: size_t,
    pub hblkhd: size_t,
    pub usmblks: size_t,
    pub fsmblks: size_t,
    pub uordblks: size_t,
    pub fordblks: size_t,
    pub keepcost: size_t,
}
extern "C" {
    pub fn memalign(arg1: size_t, arg2: size_t) -> *mut c_void;
}
extern "C" {
    pub fn mallinfo() -> mallinfo;
}
extern "C" {
    pub fn malloc_stats();
}
extern "C" {
    pub fn mallopt(
        arg1: c_int,
        arg2: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn malloc_usable_size(arg1: *mut c_void) -> size_t;
}
extern "C" {
    pub fn valloc(arg1: size_t) -> *mut c_void;
}
extern "C" {
    pub fn pvalloc(arg1: size_t) -> *mut c_void;
}
extern "C" {
    pub fn malloc_trim(arg1: size_t) -> c_int;
}
extern "C" {
    pub fn mstats(arg1: *mut c_char);
}
pub type nl_item = __nl_item;
pub const _NL_CTYPE_CODESET_NAME: _bindgen_ty_6 = 0;
pub const D_T_FMT: _bindgen_ty_6 = 1;
pub const D_FMT: _bindgen_ty_6 = 2;
pub const T_FMT: _bindgen_ty_6 = 3;
pub const T_FMT_AMPM: _bindgen_ty_6 = 4;
pub const AM_STR: _bindgen_ty_6 = 5;
pub const PM_STR: _bindgen_ty_6 = 6;
pub const DAY_1: _bindgen_ty_6 = 7;
pub const DAY_2: _bindgen_ty_6 = 8;
pub const DAY_3: _bindgen_ty_6 = 9;
pub const DAY_4: _bindgen_ty_6 = 10;
pub const DAY_5: _bindgen_ty_6 = 11;
pub const DAY_6: _bindgen_ty_6 = 12;
pub const DAY_7: _bindgen_ty_6 = 13;
pub const ABDAY_1: _bindgen_ty_6 = 14;
pub const ABDAY_2: _bindgen_ty_6 = 15;
pub const ABDAY_3: _bindgen_ty_6 = 16;
pub const ABDAY_4: _bindgen_ty_6 = 17;
pub const ABDAY_5: _bindgen_ty_6 = 18;
pub const ABDAY_6: _bindgen_ty_6 = 19;
pub const ABDAY_7: _bindgen_ty_6 = 20;
pub const MON_1: _bindgen_ty_6 = 21;
pub const MON_2: _bindgen_ty_6 = 22;
pub const MON_3: _bindgen_ty_6 = 23;
pub const MON_4: _bindgen_ty_6 = 24;
pub const MON_5: _bindgen_ty_6 = 25;
pub const MON_6: _bindgen_ty_6 = 26;
pub const MON_7: _bindgen_ty_6 = 27;
pub const MON_8: _bindgen_ty_6 = 28;
pub const MON_9: _bindgen_ty_6 = 29;
pub const MON_10: _bindgen_ty_6 = 30;
pub const MON_11: _bindgen_ty_6 = 31;
pub const MON_12: _bindgen_ty_6 = 32;
pub const ABMON_1: _bindgen_ty_6 = 33;
pub const ABMON_2: _bindgen_ty_6 = 34;
pub const ABMON_3: _bindgen_ty_6 = 35;
pub const ABMON_4: _bindgen_ty_6 = 36;
pub const ABMON_5: _bindgen_ty_6 = 37;
pub const ABMON_6: _bindgen_ty_6 = 38;
pub const ABMON_7: _bindgen_ty_6 = 39;
pub const ABMON_8: _bindgen_ty_6 = 40;
pub const ABMON_9: _bindgen_ty_6 = 41;
pub const ABMON_10: _bindgen_ty_6 = 42;
pub const ABMON_11: _bindgen_ty_6 = 43;
pub const ABMON_12: _bindgen_ty_6 = 44;
pub const ERA: _bindgen_ty_6 = 45;
pub const ERA_D_FMT: _bindgen_ty_6 = 46;
pub const ERA_D_T_FMT: _bindgen_ty_6 = 47;
pub const ERA_T_FMT: _bindgen_ty_6 = 48;
pub const ALT_DIGITS: _bindgen_ty_6 = 49;
pub const RADIXCHAR: _bindgen_ty_6 = 50;
pub const THOUSEP: _bindgen_ty_6 = 51;
pub const YESEXPR: _bindgen_ty_6 = 52;
pub const NOEXPR: _bindgen_ty_6 = 53;
pub const YESSTR: _bindgen_ty_6 = 54;
pub const NOSTR: _bindgen_ty_6 = 55;
pub const CRNCYSTR: _bindgen_ty_6 = 56;
pub const D_MD_ORDER: _bindgen_ty_6 = 57;
pub const _NL_TIME_DATE_FMT: _bindgen_ty_6 = 84;
pub type _bindgen_ty_6 = u32;
extern "C" {
    pub fn nl_langinfo(arg1: nl_item) -> *mut c_char;
}
extern "C" {
    pub fn nl_langinfo_l(arg1: nl_item, arg2: locale_t) -> *mut c_char;
}
extern "C" {
    pub fn isalnum(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isalpha(__c: c_int) -> c_int;
}
extern "C" {
    pub fn iscntrl(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isdigit(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isgraph(__c: c_int) -> c_int;
}
extern "C" {
    pub fn islower(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isprint(__c: c_int) -> c_int;
}
extern "C" {
    pub fn ispunct(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isspace(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isupper(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isxdigit(__c: c_int) -> c_int;
}
extern "C" {
    pub fn tolower(__c: c_int) -> c_int;
}
extern "C" {
    pub fn toupper(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isblank(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isascii(__c: c_int) -> c_int;
}
extern "C" {
    pub fn toascii(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isalnum_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isalpha_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isblank_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn iscntrl_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isdigit_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isgraph_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn islower_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isprint_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn ispunct_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isspace_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isupper_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isxdigit_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn tolower_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn toupper_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn isascii_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn toascii_l(__c: c_int, __l: locale_t) -> c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct re_guts {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct internal_head {
    pub _address: u8,
}
extern "C" {
    pub static mut h_errno: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostent {
    pub h_name: *mut c_char,
    pub h_aliases: *mut *mut c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *mut *mut c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct netent {
    pub n_name: *mut c_char,
    pub n_aliases: *mut *mut c_char,
    pub n_addrtype: c_int,
    pub n_net: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct servent {
    pub s_name: *mut c_char,
    pub s_aliases: *mut *mut c_char,
    pub s_port: c_int,
    pub s_proto: *mut c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct protoent {
    pub p_name: *mut c_char,
    pub p_aliases: *mut *mut c_char,
    pub p_proto: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_canonname: *mut c_char,
    pub ai_addr: *mut sockaddr,
    pub ai_next: *mut addrinfo,
}
extern "C" {
    pub fn endhostent();
}
extern "C" {
    pub fn endnetent();
}
extern "C" {
    pub fn endprotoent();
}
extern "C" {
    pub fn endservent();
}
extern "C" {
    pub fn gethostbyname(name: *const c_char) -> *mut hostent;
}
extern "C" {
    pub fn gethostbyaddr(
        addr: *const c_void,
        len: socklen_t,
        type_: c_int,
    ) -> *mut hostent;
}
extern "C" {
    pub fn gethostent() -> *mut hostent;
}
extern "C" {
    pub fn getnetbyaddr(arg1: u32, arg2: c_int) -> *mut netent;
}
extern "C" {
    pub fn getnetbyname(arg1: *const c_char) -> *mut netent;
}
extern "C" {
    pub fn getnetent() -> *mut netent;
}
extern "C" {
    pub fn getprotobyname(arg1: *const c_char) -> *mut protoent;
}
extern "C" {
    pub fn getprotobynumber(arg1: c_int) -> *mut protoent;
}
extern "C" {
    pub fn getprotoent() -> *mut protoent;
}
extern "C" {
    pub fn getservbyname(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> *mut servent;
}
extern "C" {
    pub fn getservbyport(
        arg1: c_int,
        arg2: *const c_char,
    ) -> *mut servent;
}
extern "C" {
    pub fn getservent() -> *mut servent;
}
extern "C" {
    pub fn sethostent(arg1: c_int);
}
extern "C" {
    pub fn setnetent(arg1: c_int);
}
extern "C" {
    pub fn setprotoent(arg1: c_int);
}
extern "C" {
    pub fn getnameinfo(
        sa: *const sockaddr,
        salen: socklen_t,
        host: *mut c_char,
        hostlen: socklen_t,
        serv: *mut c_char,
        servlen: socklen_t,
        flags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
}
extern "C" {
    pub fn freeaddrinfo(ai: *mut addrinfo);
}
extern "C" {
    pub fn gai_strerror(err: c_int) -> *const c_char;
}
extern "C" {
    pub fn setservent(arg1: c_int);
}
extern "C" {
    pub fn freehostent(he: *mut hostent);
}
extern "C" {
    pub fn herror(s: *const c_char);
}
extern "C" {
    pub fn hstrerror(err: c_int) -> *const c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sem_t {
    pub lock: _LOCK_T,
    pub cond: _COND_T,
    pub value: c_int,
}
extern "C" {
    pub fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> c_int;
}
extern "C" {
    pub fn pthread_create(
        __pthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void,
        __arg: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_join(
        __pthread: pthread_t,
        __value_ptr: *mut *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_detach(__pthread: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_exit(__value_ptr: *mut c_void);
}
extern "C" {
    pub fn pthread_self() -> pthread_t;
}
extern "C" {
    pub fn pthread_equal(__t1: pthread_t, __t2: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_getcpuclockid(
        thread: pthread_t,
        clock_id: *mut clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setconcurrency(new_level: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_getconcurrency() -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_rwlock_t {
    pub lock: _LOCK_T,
    pub cond_r: _COND_T,
    pub cond_w: _COND_T,
    pub data: [u8; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_rwlockattr_t {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_un {
    pub sun_len: u8,
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 104]
}
extern "C" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setpshared(
        __attr: *mut pthread_condattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __attr: *const pthread_condattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_destroy(__mutex: *mut pthread_cond_t) -> c_int;
}
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: sa_family_t,
    pub sin_port: ::in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [c_char; 8],
}
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_len: u8,
    pub sin6_family: sa_family_t,
    pub sin6_port: ::in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: ::in6_addr,
    pub sin6_scope_id: u32,
}

pub fn WIFSTOPPED(status: ::c_int) -> bool {
    (status & 0xff) == 0x7f
}

pub fn WSTOPSIG(status: ::c_int) -> ::c_int {
    (status >> 8) & 0xff
}

pub fn WIFCONTINUED(status: ::c_int) -> bool {
    status == 0xffff
}

pub fn WIFSIGNALED(status: ::c_int) -> bool {
    ((status & 0x7f) + 1) as i8 >= 2
}

pub fn WTERMSIG(status: ::c_int) -> ::c_int {
    status & 0x7f
}

pub fn WIFEXITED(status: ::c_int) -> bool {
    (status & 0x7f) == 0
}

pub fn WEXITSTATUS(status: ::c_int) -> ::c_int {
    (status >> 8) & 0xff
}

pub fn WCOREDUMP(status: ::c_int) -> bool {
    (status & 0x80) != 0
}
