//! This example isn't faster, smaller, simpler, more efficient, more portable,
//! or more desireable than regular hello world in any practical way. It just
//! demonstrates the API of this crate.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use io_extras::grip::{AsRawGrip, FromRawGrip};
use io_extras::raw::RawWriteable;
use io_lifetimes::AsFilelike;
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    let stdout = stdout();
    let stdout = stdout.lock();

    // Obtain an `RawWriteable` and use it to write.
    writeln!(
        unsafe { RawWriteable::from_raw_grip(stdout.as_raw_grip()) },
        "hello, world"
    )?;

    // Obtain a `FilelikeView` and use it to write.
    writeln!(stdout.as_filelike_view::<std::fs::File>(), "hello, world")?;

    Ok(())
}
