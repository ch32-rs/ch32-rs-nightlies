//! Peripheral access API for CH58X microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.31.5)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.31.5/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [ch32-rs](https://github.com/ch32-rs/ch32-rs)
//!
//! This crate supports all CH58X devices; for the complete list please
//! see:
//! [ch58x](https://crates.io/crates/ch58x)
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "ch58x")]
pub mod ch58x;

