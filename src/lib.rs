#[macro_use]
extern crate cfg_if;

#[no_std]

cfg_if! {
    if #[cfg(target_arch = "mips")] {
        mod mips;
        pub use self::mips::*;
    } else if #[cfg(target_arch = "mips64")] {
        mod mips64;
        pub use self::mips64::*;
    } else if #[cfg(target_arch = "powerpc")] {
        mod powerpc;
        pub use self::powerpc::*;
    } else if #[cfg(target_arch = "powerpc64")] {
        mod powerpc64;
        pub use self::powerpc64::*;
    } else {
        mod generic;
        pub use self::generic::*;
    }
}
