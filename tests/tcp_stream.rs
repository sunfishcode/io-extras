//! This test is similar to the hello example, but writes to a `TcpStream`.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use io_extras::borrowed::{BorrowedReadable, BorrowedWriteable};
use io_extras::grip::{AsGrip, AsRawGrip, FromGrip, FromRawGrip, IntoGrip};
use io_extras::owned::OwnedReadable;
use io_extras::raw::{RawReadable, RawWriteable};
use io_lifetimes::AsSocketlike;
use std::io::{self, Read, Write};
use std::mem::forget;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_write() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:0")?;
    let addr = listener.local_addr()?;

    let _t = thread::spawn(move || -> io::Result<()> {
        let stream = TcpStream::connect(addr)?;

        // Obtain an `RawWriteable` and use it to write.
        writeln!(
            unsafe { RawWriteable::from_raw_grip(stream.as_raw_grip()) },
            "Write via RawWriteable"
        )?;

        // Obtain a `SocketlikeView` and use it to write.
        writeln!(
            &*stream.as_socketlike_view::<TcpStream>(),
            "Write via SocketlikeView"
        )?;

        // Obtain an `BorrowedWriteable` and use it to write.
        writeln!(
            BorrowedWriteable::borrow(stream.as_grip()),
            "Write via BorrowedWriteable"
        )?;

        Ok(())
    });

    let mut stream = listener.accept()?.0;
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    assert_eq!(
        buf,
        "Write via RawWriteable\n\
                Write via SocketlikeView\n\
                Write via BorrowedWriteable\n"
    );

    Ok(())
}

fn accept() -> io::Result<TcpStream> {
    let listener = TcpListener::bind("localhost:0")?;
    let addr = listener.local_addr()?;

    let _t = thread::spawn(move || -> io::Result<()> {
        let mut stream = TcpStream::connect(addr)?;
        write!(stream, "hello, world")?;
        Ok(())
    });

    Ok(listener.accept()?.0)
}

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_raw_readable() -> io::Result<()> {
    // Obtain an `RawReadable` and use it to read.
    let stream = accept()?;
    let mut buf = String::new();
    unsafe { RawReadable::from_raw_grip(stream.as_raw_grip()) }.read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_socketlike_view() -> io::Result<()> {
    // Obtain a `SocketlikeView` and use it to read.
    let stream = accept()?;
    let mut buf = String::new();
    (&*stream.as_socketlike_view::<TcpStream>()).read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_borrowed_readable() -> io::Result<()> {
    // Obtain a `BorrowedReadable` and use it to read.
    let stream = accept()?;
    let mut buf = String::new();
    BorrowedReadable::borrow(stream.as_grip()).read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)] // TCP I/O calls foreign functions
fn tcp_stream_owned_readable() -> io::Result<()> {
    // Obtain a `OwnedReadable` and use it to read.
    let stream = accept()?;
    let mut buf = String::new();
    let mut owned = OwnedReadable::from_grip(stream.into_grip());
    owned.read_to_string(&mut buf)?;
    // Avoid calling drop so that we don't depend on io-lifetimes' "close"
    // feature.
    forget(owned);
    assert_eq!(buf, "hello, world");
    Ok(())
}
