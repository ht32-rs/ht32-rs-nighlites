//! Peripheral access API for HT32F1XXXX microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.29.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ht32-rs](https://github.com/ht32-rs/ht32-rs)
//!
//! This crate supports all HT32F1XXXX devices; for the complete list please
//! see:
//! [ht32f1xxxx](https://github.com/ht32-rs/ht32-rs/tree/master/ht32f1xxxx)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ht32f12345")]
pub mod ht32f12345;

#[cfg(feature = "ht32f12364")]
pub mod ht32f12364;

#[cfg(feature = "ht32f12365_66")]
pub mod ht32f12365_66;

