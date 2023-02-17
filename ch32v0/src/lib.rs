//! Peripheral access API for CH32V0 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.26.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.26.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ch32-rs](https://github.com/ch32-rs/ch32-rs)
//!
//! This crate supports all CH32V0 devices; for the complete list please
//! see:
//! [ch32v0](https://crates.io/crates/ch32v0)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ch32v003")]
pub mod ch32v003;

