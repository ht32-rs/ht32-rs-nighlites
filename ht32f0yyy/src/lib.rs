//! Peripheral access API for HT32F0YYY microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.29.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ht32-rs](https://github.com/ht32-rs/ht32-rs)
//!
//! This crate supports all HT32F0YYY devices; for the complete list please
//! see:
//! [ht32f0yyy](https://github.com/ht32-rs/ht32-rs/tree/master/ht32f0yyy)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ht32f0006")]
pub mod ht32f0006;

#[cfg(feature = "ht32f0008")]
pub mod ht32f0008;

