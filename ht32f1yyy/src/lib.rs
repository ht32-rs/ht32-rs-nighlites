//! Peripheral access API for HT32F1YYY microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.29.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ht32-rs](https://github.com/ht32-rs/ht32-rs)
//!
//! This crate supports all HT32F1YYY devices; for the complete list please
//! see:
//! [ht32f1yyy](https://github.com/ht32-rs/ht32-rs/tree/master/ht32f1yyy)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ht32f125x")]
pub mod ht32f125x;

#[cfg(feature = "ht32f1653_54")]
pub mod ht32f1653_54;

#[cfg(feature = "ht32f1655_56")]
pub mod ht32f1655_56;

#[cfg(feature = "ht32f175x")]
pub mod ht32f175x;

