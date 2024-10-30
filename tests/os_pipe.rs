//! This example is similar to the tcp_stream test, but writes to a
//! `PipeWriter`.

#![cfg(not(target_os = "wasi"))]
#![cfg(feature = "os_pipe")]

use io_extras::borrowed::{BorrowedReadable, BorrowedWriteable};
use io_extras::grip::{AsGrip, AsRawGrip, FromGrip, FromRawGrip, IntoGrip};
use io_extras::owned::OwnedReadable;
use io_extras::raw::{RawReadable, RawWriteable};
use io_lifetimes::AsFilelike;
use os_pipe::{pipe, PipeReader};
use std::io::{self, Read, Write};
use std::mem::forget;
use std::thread;

#[test]
#[cfg_attr(miri, ignore)] // pipe I/O calls foreign functions
fn os_pipe_write() -> io::Result<()> {
    let (mut input, output) = pipe()?;

    let _t = thread::spawn(move || -> io::Result<()> {
        // Obtain an `RawWriteable` and use it to write.
        writeln!(
            unsafe { RawWriteable::from_raw_grip(output.as_raw_grip()) },
            "Write via RawWriteable"
        )?;

        // Obtain a `FilelikeView` and use it to write.
        writeln!(
            &*output.as_filelike_view::<std::fs::File>(),
            "Write via FilelikeView"
        )?;

        // Obtain an `BorrowedWriteable` and use it to write.
        writeln!(
            BorrowedWriteable::borrow(output.as_grip()),
            "Write via BorrowedWriteable"
        )?;

        Ok(())
    });

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    assert_eq!(
        buf,
        "Write via RawWriteable\n\
                Write via FilelikeView\n\
                Write via BorrowedWriteable\n"
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
fn os_pipe_raw_readable() -> io::Result<()> {
    // Obtain an `RawReadable` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    unsafe { RawReadable::from_raw_grip(stream.as_raw_grip()) }.read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)] // pipe I/O calls foreign functions
fn os_pipe_filelike_view() -> io::Result<()> {
    // Obtain a `FilelikeView` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    (&*stream.as_filelike_view::<std::fs::File>()).read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)] // pipe I/O calls foreign functions
fn os_pipe_borrowed_readable() -> io::Result<()> {
    // Obtain a `BorrowedReadable` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    BorrowedReadable::borrow(stream.as_grip()).read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)] // pipe I/O calls foreign functions
fn os_pipe_owned_readable() -> io::Result<()> {
    // Obtain a `OwnedReadable` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    let mut owned = OwnedReadable::from_grip(stream.into_grip());
    owned.read_to_string(&mut buf)?;
    // Avoid calling drop so that we don't depend on io-lifetimes' "close"
    // feature.
    forget(owned);
    assert_eq!(buf, "hello, world");
    Ok(())
}
