//! This example isn't faster, smaller, simpler, more efficient, more portable,
//! or more desireable than regular hello world in any practical way. It just
//! demonstrates the API of this crate.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd};
use std::{
    io::{stdout, Write},
    mem::ManuallyDrop,
};
use unsafe_io::{AsUnsafeFile, AsUnsafeHandle, FromUnsafeFile};
#[cfg(windows)]
use {
    std::os::windows::io::FromRawHandle,
    unsafe_io::{AsRawHandleOrSocket, RawHandleOrSocket},
};

fn main() -> anyhow::Result<()> {
    let stdout = stdout();
    let stdout = stdout.lock();

    // Obtain an `UnsafeWriteable` and use it to write.
    writeln!(
        unsafe { stdout.as_unsafe_handle().as_writeable() },
        "hello, world"
    )?;

    // Obtain an `UnsafeFile` and use it to construct a temporary manually-drop
    // `File` to write.
    writeln!(stdout.as_file(), "hello, world")?;

    // Similar, but do it manually.
    writeln!(
        ManuallyDrop::new(unsafe { std::fs::File::from_unsafe_file(stdout.as_unsafe_file()) }),
        "hello, world"
    )?;

    // Similar, but use the Posix-ish-specific type.
    #[cfg(not(windows))]
    writeln!(
        ManuallyDrop::new(unsafe {
            std::fs::File::from_raw_fd(stdout.as_unsafe_handle().as_raw_fd())
        }),
        "hello, world"
    )?;

    // Similar, but use the Windows-specific type.
    #[cfg(windows)]
    writeln!(
        ManuallyDrop::new(unsafe {
            std::fs::File::from_raw_handle(
                match stdout.as_unsafe_handle().as_raw_handle_or_socket() {
                    RawHandleOrSocket::Handle(handle) => handle,
                    RawHandleOrSocket::Socket(_) => panic!(),
                },
            )
        }),
        "hello, world"
    )?;

    Ok(())
}
