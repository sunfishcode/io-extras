<div align="center">
  <h1><code>unsafe-io</code></h1>

  <p>
    <strong>Non-owning unsafe I/O</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/unsafe-io/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/unsafe-io/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/unsafe_io"><img src="https://img.shields.io/crates/v/unsafe_io.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/unsafe-io"><img src="https://docs.rs/unsafe-io/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

Have you ever found yourself writing essentially the same code twice, once
for [`RawFd`] for Posix-ish platforms and once for [`RawHandle`] or
[`RawSocket`] for Windows platforms? This crate abstracts over those platform
differences.

Being non-owning, these handles operate much like raw pointers in Rust. They
are considered safe to construct, but unsafe to use in any way that depends on
the resource they point to.

For a safe owning API, see the [io-streams] crate.

[`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
[`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
[`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
[io-streams]: https://github.com/sunfishcode/io-streams/
