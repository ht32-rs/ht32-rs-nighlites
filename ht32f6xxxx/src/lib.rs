//! Peripheral access API for HT32F6XXXX microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.29.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ht32-rs](https://github.com/ht32-rs/ht32-rs)
//!
//! This crate supports all HT32F6XXXX devices; for the complete list please
//! see:
//! [ht32f6xxxx](https://github.com/ht32-rs/ht32-rs/tree/master/ht32f6xxxx)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ht32f61352")]
pub mod ht32f61352;

#[cfg(feature = "ht32f65230_40")]
pub mod ht32f65230_40;

