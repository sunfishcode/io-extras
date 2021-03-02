//! This example is similar to the tcp_stream test, but writes to a
//! `PipeWriter`.

#![cfg(all(not(target_os = "wasi"), feature = "os_pipe"))]

use os_pipe::{pipe, PipeReader};
use std::{
    io::{self, Read, Write},
    mem::ManuallyDrop,
    thread,
};
#[cfg(not(windows))]
use unsafe_io::os::posish::{AsRawFd, FromRawFd};
use unsafe_io::{AsUnsafeFile, AsUnsafeHandle, FromUnsafeFile};
#[cfg(windows)]
use {std::os::windows::io::FromRawHandle, unsafe_io::os::windows::AsRawHandleOrSocket};

#[test]
#[cfg_attr(miri, ignore)] // pipe I/O calls foreign functions
fn os_pipe_write() -> io::Result<()> {
    let (mut input, output) = pipe()?;

    let _t = thread::spawn(move || -> io::Result<()> {
        // Obtain an `UnsafeWriteable` and use it to write.
        writeln!(
            unsafe { output.as_unsafe_handle().as_writeable() },
            "Write via UnsafeWriteable"
        )?;

        // Obtain an `UnsafeSocket` and use it to construct a temporary manually-drop
        // `PipeWriter` to write.
        writeln!(output.as_file_view(), "Write via as_file_view")?;

        // Similar, but do it manually.
        writeln!(
            ManuallyDrop::new(unsafe { std::fs::File::from_unsafe_file(output.as_unsafe_file()) }),
            "Write via unsafe handle"
        )?;

        // Similar, but gratuitously pass stdout through `from_filelike`.
        writeln!(
            ManuallyDrop::new(std::fs::File::from_filelike(unsafe {
                std::fs::File::from_unsafe_file(output.as_unsafe_file())
            })),
            "Write via unsafe handle using from_filelike"
        )?;

        // Similar, but use the Posix-ish-specific type.
        #[cfg(not(windows))]
        writeln!(
            ManuallyDrop::new(unsafe {
                std::fs::File::from_raw_fd(output.as_unsafe_handle().as_raw_fd())
            }),
            "Write via raw fd"
        )?;

        // Similar, but use the Windows-specific type.
        #[cfg(windows)]
        writeln!(
            ManuallyDrop::new(unsafe {
                std::fs::File::from_raw_handle(
                    output
                        .as_unsafe_handle()
                        .as_raw_handle_or_socket()
                        .as_unowned_raw_handle()
                        .unwrap(),
                )
            }),
            "Write via raw socket"
        )?;

        Ok(())
    });

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    #[cfg(not(windows))]
    assert_eq!(
        buf,
        "Write via UnsafeWriteable\n\
                Write via as_file_view\n\
                Write via unsafe handle\n\
                Write via unsafe handle using from_filelike\n\
                Write via raw fd\n"
    );

    #[cfg(windows)]
    assert_eq!(
        buf,
        "Write via UnsafeWriteable\n\
                Write via as_file_view\n\
                Write via unsafe handle\n\
                Write via unsafe handle using from_filelike\n\
                Write via raw socket\n"
    );

    Ok(())
}

fn write_to_pipe() -> io::Result<PipeReader> {
    let (input, mut output) = pipe()?;

    let _t = thread::spawn(move || -> io::Result<()> {
        write!(output, "hello, world")?;
        Ok(())
    });

    Ok(input)
}

#[test]
#[cfg_attr(miri, ignore)] // pipe I/O calls foreign functions
fn os_pipe_read() -> io::Result<()> {
    // Obtain an `UnsafeReadable` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    unsafe { stream.as_unsafe_handle().as_readable() }.read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Obtain an `UnsafeSocket` and use it to construct a temporary manually-drop
    // `PipeReader` to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    stream.as_file_view().read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Similar, but do it manually.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    ManuallyDrop::new(unsafe { std::fs::File::from_unsafe_file(stream.as_unsafe_file()) })
        .read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Similar, but use the Posix-ish-specific type.
    #[cfg(not(windows))]
    {
        let stream = write_to_pipe()?;
        let mut buf = String::new();
        ManuallyDrop::new(unsafe {
            std::fs::File::from_raw_fd(stream.as_unsafe_handle().as_raw_fd())
        })
        .read_to_string(&mut buf)?;
        assert_eq!(buf, "hello, world");
    }

    // Similar, but use the Windows-specific type.
    #[cfg(windows)]
    {
        let stream = write_to_pipe()?;
        let mut buf = String::new();
        ManuallyDrop::new(unsafe {
            std::fs::File::from_raw_handle(
                stream
                    .as_unsafe_handle()
                    .as_raw_handle_or_socket()
                    .as_unowned_raw_handle()
                    .unwrap(),
            )
        })
        .read_to_string(&mut buf)?;
        assert_eq!(buf, "hello, world");
    }

    Ok(())
}
