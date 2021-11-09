<div align="center">
  <h1><code>unsafe-io</code></h1>

  <p>
    <strong>File/socket handle/descriptor utilities</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/unsafe-io/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/unsafe-io/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/unsafe-io"><img src="https://img.shields.io/crates/v/unsafe-io.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/unsafe-io"><img src="https://docs.rs/unsafe-io/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate has evolved a lot and no longer is really about "unsafe I/O".
Right now it holds a few miscellaneous utilities related to I/O:

 - `HandleOrSocket` types and traits for Windows, which abstract over Windows
   `*Handle*` and their corresponding Windows `*Socket*` types and traits.

 - `Grip` types and traits, which abstract over the aforementioned Windows
   `HandleOrSocket` types and traits and their corresponding non-Windows `Fd`
   types and traits.

 - `RawReadable` and `RawWritable`, which adapt a raw `Fd`/`Handle` to
   implement the `Read` and `Write` traits, respectively.

 - `ReadWrite` traits, and supporting types, which provide abstractions over
   types with one or two I/O resources, for reading and for writing.
