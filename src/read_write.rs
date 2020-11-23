//! Traits for types which may have separate `Read` and `Write` handles.

use crate::UnsafeHandle;
#[cfg(windows)]
use crate::{AsRawHandleOrSocket, RawHandleOrSocket};
#[cfg(unix)]
use std::os::unix::{
    io::{AsRawFd, RawFd},
    net::UnixStream,
};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, RawFd};
use std::{fs::File, net::TcpStream};

/// An analog of [`AsUnsafeHandle`] for `ReadWrite` streams which may have two
/// handles, one for reading and one for writing.
pub trait AsUnsafeReadWriteHandle {
    /// Extracts the unsafe handle for reading.
    fn as_unsafe_read_handle(&self) -> UnsafeHandle;

    /// Extracts the unsafe handle for writing.
    fn as_unsafe_write_handle(&self) -> UnsafeHandle;
}

/// Like [`std::os::unix::io::AsRawFd`], but specifically for use with
/// [`ReadWrite`] implementations which may contain both reading and writing
/// file descriptors.
///
/// [`std::os::unix::io::AsRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html
#[cfg(not(windows))]
pub trait AsRawReadWriteFd {
    /// Extracts the raw file descriptor for reading.
    ///
    /// Like [`std::os::unix::io::AsRawFd::as_raw_fd`], but returns the
    /// reading file descriptor of a [`ReadWrite`] implementation.
    ///
    /// [`std::os::unix::io::AsRawFd::as_raw_fd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html#tymethod.as_raw_fd
    fn as_raw_read_fd(&self) -> RawFd;

    /// Extracts the raw file descriptor for writing.
    ///
    /// Like [`std::os::unix::io::AsRawFd::as_raw_fd`], but returns the
    /// writing file descriptor of a [`ReadWrite`] implementation.
    ///
    /// [`std::os::unix::io::AsRawFd::as_raw_fd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html#tymethod.as_raw_fd
    fn as_raw_write_fd(&self) -> RawFd;
}

/// Like [`AsRawHandleOrSocket`], but specifically for use with [`ReadWrite`]
/// implementations which may contain both reading and writing file
/// descriptors.
#[cfg(windows)]
pub trait AsRawReadWriteHandleOrSocket {
    /// Extracts the raw handle or socket for reading.
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket;

    /// Extracts the raw handle or socket for writing.
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket;
}

#[cfg(not(windows))]
impl<T: AsRawReadWriteFd> AsUnsafeReadWriteHandle for T {
    #[inline]
    fn as_unsafe_read_handle(&self) -> UnsafeHandle {
        UnsafeHandle::from_raw_fd(self.as_raw_read_fd())
    }

    #[inline]
    fn as_unsafe_write_handle(&self) -> UnsafeHandle {
        UnsafeHandle::from_raw_fd(self.as_raw_write_fd())
    }
}

#[cfg(windows)]
impl<T: AsRawReadWriteHandleOrSocket> AsUnsafeReadWriteHandle for T {
    #[inline]
    fn as_unsafe_read_handle(&self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle_or_socket(self.as_raw_read_handle_or_socket())
    }

    #[inline]
    fn as_unsafe_write_handle(&self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle_or_socket(self.as_raw_write_handle_or_socket())
    }
}

#[cfg(not(windows))]
impl AsRawReadWriteFd for File {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawReadWriteHandleOrSocket for File {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl AsRawReadWriteFd for TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawReadWriteHandleOrSocket for TcpStream {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(unix)]
impl AsRawReadWriteFd for UnixStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl<T: AsRawReadWriteFd> AsRawReadWriteFd for Box<T> {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        (**self).as_raw_read_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        (**self).as_raw_write_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawReadWriteHandleOrSocket> AsRawReadWriteHandleOrSocket for Box<T> {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        (**self).as_raw_read_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        (**self).as_raw_write_handle_or_socket()
    }
}
