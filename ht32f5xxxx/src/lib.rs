//! Peripheral access API for HT32F5XXXX microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.29.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ht32-rs](https://github.com/ht32-rs/ht32-rs)
//!
//! This crate supports all HT32F5XXXX devices; for the complete list please
//! see:
//! [ht32f5xxxx](https://github.com/ht32-rs/ht32-rs/tree/master/ht32f5xxxx)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ht32f50220_30")]
pub mod ht32f50220_30;

#[cfg(feature = "ht32f50231_41")]
pub mod ht32f50231_41;

#[cfg(feature = "ht32f50343")]
pub mod ht32f50343;

#[cfg(feature = "ht32f52142")]
pub mod ht32f52142;

#[cfg(feature = "ht32f52220_30")]
pub mod ht32f52220_30;

#[cfg(feature = "ht32f52231_41")]
pub mod ht32f52231_41;

#[cfg(feature = "ht32f52243_53")]
pub mod ht32f52243_53;

#[cfg(feature = "ht32f52331_41")]
pub mod ht32f52331_41;

#[cfg(feature = "ht32f52342_52")]
pub mod ht32f52342_52;

#[cfg(feature = "ht32f52344_54")]
pub mod ht32f52344_54;

#[cfg(feature = "ht32f52357_67")]
pub mod ht32f52357_67;

#[cfg(feature = "ht32f57331_41")]
pub mod ht32f57331_41;

#[cfg(feature = "ht32f57342_52")]
pub mod ht32f57342_52;

#[cfg(feature = "ht32f59041")]
pub mod ht32f59041;

#[cfg(feature = "ht32f59741")]
pub mod ht32f59741;

