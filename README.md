<div align="center">
  <h1><code>unsafe-io</code></h1>

  <p>
    <strong>Non-owning unsafe I/O</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/unsafe-io/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/unsafe-io/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/unsafe-io"><img src="https://img.shields.io/crates/v/unsafe-io.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/unsafe-io"><img src="https://docs.rs/unsafe-io/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

Have you ever found yourself writing essentially the same code twice, once for
[`RawFd`] for Posix-ish platforms and once for [`RawHandle`] or [`RawSocket`]
for Windows platforms? This crate abstracts over those platform differences.

Being non-owning, these handles operate much like raw pointers in Rust. They
are considered safe to construct, but unsafe to use in any way that depends on
the resource they point to.

This library is meant to be a building block for higher-level libraries, such
as the [io-streams] crate.

## Brief Overview

The central type of this library is [`UnsafeHandle`]. On Posix-ish platforms
it just contains a [`RawFd`]. On Windows, it contains an enum of either a
`RawHandle` (for files and pipes), a `RawSocket` (for sockets), or a stdio
handle (for stdin, stdout, stderr), allowing it to abstract over different
types of I/O in a similar manner.

[`UnsafeFile`] and [`UnsafeSocket`] are similar non-owning types, but which
only contain one type, instead of an enum, which allow them to be used in
contexts that only support one type.

The [crate documentation] has a complete overview and examples.

[`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
[`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
[`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
[io-streams]: https://github.com/sunfishcode/io-streams/
[`UnsafeHandle`]: https://docs.rs/unsafe-io/latest/unsafe_io/struct.UnsafeHandle.html
[`UnsafeFile`]: https://docs.rs/unsafe-io/latest/unsafe_io/struct.UnsafeFile.html
[`UnsafeSocket`]: https://docs.rs/unsafe-io/latest/unsafe_io/struct.UnsafeSocket.html
[crate documentation]: https://docs.rs/unsafe-io/latest/unsafe_io/
