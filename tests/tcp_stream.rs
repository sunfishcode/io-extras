//! This test is similar to the hello example, but writes to a `TcpStream`.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd};
use std::{
    io::{Read, Write},
    mem::ManuallyDrop,
    net::{TcpListener, TcpStream},
    thread,
};
use unsafe_io::{AsUnsafeHandle, AsUnsafeSocket, FromUnsafeSocket};
#[cfg(windows)]
use {std::os::windows::io::FromRawSocket, unsafe_io::AsRawHandleOrSocket};

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_write() -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:0")?;
    let addr = listener.local_addr()?;

    let _t = thread::spawn(move || -> anyhow::Result<()> {
        let stream = TcpStream::connect(addr)?;

        // Obtain an `UnsafeWriteable` and use it to write.
        writeln!(
            unsafe { stream.as_unsafe_handle().as_writeable() },
            "Write via UnsafeWriteable"
        )?;

        // Obtain an `UnsafeSocket` and use it to construct a temporary manually-drop
        // `TcpStream` to write.
        writeln!(stream.as_tcp_stream(), "Write via as_tcp_stream")?;

        // Similar, but do it manually.
        writeln!(
            ManuallyDrop::new(unsafe {
                std::net::TcpStream::from_unsafe_socket(stream.as_unsafe_socket())
            }),
            "Write via unsafe socket"
        )?;

        // Similar, but use the Posix-ish-specific type.
        #[cfg(not(windows))]
        writeln!(
            ManuallyDrop::new(unsafe {
                std::net::TcpStream::from_raw_fd(stream.as_unsafe_handle().as_raw_fd())
            }),
            "Write via raw fd"
        )?;

        // Similar, but use the Windows-specific type.
        #[cfg(windows)]
        writeln!(
            ManuallyDrop::new(unsafe {
                std::net::TcpStream::from_raw_socket(
                    stream
                        .as_unsafe_socket()
                        .as_raw_handle_or_socket()
                        .as_raw_socket()
                        .unwrap(),
                )
            }),
            "Write via raw socket"
        )?;

        Ok(())
    });

    let mut stream = listener.accept()?.0;
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    #[cfg(not(windows))]
    assert_eq!(
        buf,
        "Write via UnsafeWriteable\n\
                Write via as_tcp_stream\n\
                Write via unsafe socket\n\
                Write via raw fd\n"
    );

    #[cfg(windows)]
    assert_eq!(
        buf,
        "Write via UnsafeWriteable\n\
                Write via as_tcp_stream\n\
                Write via unsafe socket\n\
                Write via raw socket\n"
    );

    Ok(())
}

fn accept() -> anyhow::Result<TcpStream> {
    let listener = TcpListener::bind("localhost:0")?;
    let addr = listener.local_addr()?;

    let _t = thread::spawn(move || -> anyhow::Result<()> {
        let mut stream = TcpStream::connect(addr)?;
        write!(stream, "hello, world")?;
        Ok(())
    });

    Ok(listener.accept()?.0)
}

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_read() -> anyhow::Result<()> {
    // Obtain an `UnsafeReadable` and use it to read.
    let stream = accept()?;
    let mut buf = String::new();
    unsafe { stream.as_unsafe_handle().as_readable() }.read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Obtain an `UnsafeSocket` and use it to construct a temporary manually-drop
    // `TcpStream` to read.
    let stream = accept()?;
    let mut buf = String::new();
    stream.as_tcp_stream().read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Similar, but do it manually.
    let stream = accept()?;
    let mut buf = String::new();
    ManuallyDrop::new(unsafe {
        std::net::TcpStream::from_unsafe_socket(stream.as_unsafe_socket())
    })
    .read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Similar, but use the Posix-ish-specific type.
    #[cfg(not(windows))]
    {
        let stream = accept()?;
        let mut buf = String::new();
        ManuallyDrop::new(unsafe {
            std::net::TcpStream::from_raw_fd(stream.as_unsafe_handle().as_raw_fd())
        })
        .read_to_string(&mut buf)?;
        assert_eq!(buf, "hello, world");
    }

    // Similar, but use the Windows-specific type.
    #[cfg(windows)]
    {
        let stream = accept()?;
        let mut buf = String::new();
        ManuallyDrop::new(unsafe {
            std::net::TcpStream::from_raw_socket(
                stream
                    .as_unsafe_socket()
                    .as_raw_handle_or_socket()
                    .as_raw_socket()
                    .unwrap(),
            )
        })
        .read_to_string(&mut buf)?;
        assert_eq!(buf, "hello, world");
    }

    Ok(())
}
