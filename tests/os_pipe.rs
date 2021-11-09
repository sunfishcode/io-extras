//! This example is similar to the tcp_stream test, but writes to a
//! `PipeWriter`.

#![cfg(not(target_os = "wasi"))]
#![cfg(feature = "os_pipe")]

use io_extras::grip::{AsRawGrip, FromRawGrip};
use io_extras::raw::{RawReadable, RawWriteable};
use io_lifetimes::AsFilelike;
use os_pipe::{pipe, PipeReader};
use std::io::{self, Read, Write};
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
            output.as_filelike_view::<std::fs::File>(),
            "Write via FilelikeView"
        )?;

        Ok(())
    });

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    assert_eq!(
        buf,
        "Write via RawWriteable\n\
                Write via FilelikeView\n"
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
    // Obtain an `RawReadable` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    unsafe { RawReadable::from_raw_grip(stream.as_raw_grip()) }.read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    // Obtain a `FilelikeView` and use it to read.
    let stream = write_to_pipe()?;
    let mut buf = String::new();
    stream
        .as_filelike_view::<std::fs::File>()
        .read_to_string(&mut buf)?;
    assert_eq!(buf, "hello, world");

    Ok(())
}
