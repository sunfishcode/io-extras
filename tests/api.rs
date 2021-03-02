//! This example isn't faster, smaller, simpler, more efficient, more portable,
//! or more desireable than regular hello world in any practical way. It just
//! demonstrates the API of this crate.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use std::io::{stderr, stdout};
#[cfg(not(windows))]
use unsafe_io::os::posish::{AsRawFd, AsRawReadWriteFd, RawFd};
#[cfg(windows)]
use unsafe_io::os::windows::{
    AsRawHandleOrSocket, AsRawReadWriteHandleOrSocket, RawHandleOrSocket,
};
use unsafe_io::{AsUnsafeFile, AsUnsafeHandle, OwnsRaw, ReadHalf, WriteHalf};

#[test]
fn eq() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let stderr = stderr();
    let stderr = stderr.lock();

    // Trivially assert that stdout and stderr has the same handles as
    // themselves and different handles from each other.
    assert!(stdout.eq_handle(&stdout));
    assert!(stderr.eq_handle(&stderr));
    assert!(!stdout.eq_handle(&stderr));
    assert!(!stderr.eq_handle(&stdout));

    // The same is true of file-like views of their handles.
    assert!(stdout.eq_file(&stdout));
    assert!(stderr.eq_file(&stderr));
    assert!(!stdout.eq_file(&stderr));
    assert!(!stderr.eq_file(&stdout));
}

struct Stdio {}

#[cfg(not(windows))]
impl AsRawReadWriteFd for Stdio {
    fn as_raw_read_fd(&self) -> RawFd {
        std::io::stdin().as_raw_fd()
    }

    fn as_raw_write_fd(&self) -> RawFd {
        std::io::stdout().as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawReadWriteHandleOrSocket for Stdio {
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        std::io::stdin().as_raw_handle_or_socket()
    }

    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        std::io::stdout().as_raw_handle_or_socket()
    }
}

// Safety: stdin and stdout will outlive `Stdio` instances.
unsafe impl OwnsRaw for Stdio {}

#[test]
fn read_write() {
    let stdio = Stdio {};
    assert!(ReadHalf(&stdio).eq_handle(&std::io::stdin()));
    assert!(WriteHalf(&stdio).eq_handle(&std::io::stdout()));
    assert!(unsafe {
        ReadHalf(&stdio)
            .as_unsafe_handle()
            .eq(std::io::stdin().as_unsafe_handle())
    });
    assert!(unsafe {
        WriteHalf(&stdio)
            .as_unsafe_handle()
            .eq(std::io::stdout().as_unsafe_handle())
    });
}
