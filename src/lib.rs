//! This crate provides a few miscellaneous utilities related to I/O:
//!
//! - `HandleOrSocket` types and traits for Windows, which abstract over Windows
//!   `*Handle*` and their corresponding Windows `*Socket*` types and traits.
//!
//! - `Grip` types and traits, which abstract over the aforementioned Windows
//!   `HandleOrSocket` types and traits and their corresponding non-Windows `Fd`
//!   types and traits.
//!
//! - `RawReadable` and `RawWritable`, which adapt a raw `Fd`/`Handle` to
//!   implement the `Read` and `Write` traits, respectively.
//!
//! - `ReadWrite` traits, and supporting types, which provide abstractions over
//!   types with one or two I/O resources, for reading and for writing.

#![deny(missing_docs)]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

pub mod grip;
pub mod os;
pub mod raw;
pub mod read_write;
