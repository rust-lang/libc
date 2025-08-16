#[cfg(target_arch = "mips")]
mod mips;

#[cfg(target_arch = "mips64")]
mod mips64;

mod generic;

cfg_if! {
    if #[cfg(target_arch = "mips")] {
        pub use mips::socket;
    } else if #[cfg(target_arch = "mips64")] {
        mod mips64;
        pub use mips64::socket;
    } else {
        pub use generic::socket;
    }
}
