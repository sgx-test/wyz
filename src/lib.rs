/*! `wyz` – myrrlyn’s wyzyrdly library

This crate consolidates all the small tools and conveniences I’ve built up in my
experience building Rust crates.

Each module has more documentation about what it contains. The modules are
largely independent, and can be used individually.
!*/

#![no_std]
#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod conv;
pub mod fmt;
pub mod pipe;
pub mod tap;

#[cfg(feature = "std")]
#[macro_use]
pub mod exit;

pub use conv::*;
pub use fmt::*;
pub use pipe::*;
pub use tap::*;

#[cfg(feature = "std")]
pub use exit::*;
