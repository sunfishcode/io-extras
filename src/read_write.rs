//! This defines [`AsUnsafeReadWriteHandle`], and supporting utilities, which
//! is similar to [`AsUnsafeHandle`] except that instead of just having one
//! `as_unsafe_handle` function, it has separate `as_unsafe_read_handle` and
//! `as_unsafe_write_handle` functions, so that it can be implemented by
//! types which contain two handles, one for reading and one for writing.

#[cfg(not(windows))]
use crate::os::posish::{AsRawFd, RawFd};
#[cfg(windows)]
use crate::os::windows::{AsRawHandleOrSocket, RawHandleOrSocket};
use crate::{AsUnsafeHandle, OwnsRaw, UnsafeHandle};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::{fs::File, net::TcpStream};

/// Like [`AsUnsafeHandle`], but for types which may have one or two handles,
/// for reading and writing.
///
/// For types that only have one, both functions return the same value.
pub trait AsUnsafeReadWriteHandle {
    /// Extracts the unsafe handle for reading.
    fn as_unsafe_read_handle(&self) -> UnsafeHandle;

    /// Extracts the unsafe handle for writing.
    fn as_unsafe_write_handle(&self) -> UnsafeHandle;
}

/// Like [`AsRawFd`], but for types which may have one or two file descriptors,
/// for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(not(windows))]
pub trait AsRawReadWriteFd {
    /// Extracts the raw file descriptor for reading.
    ///
    /// Like [`AsRawFd::as_raw_fd`], but returns the reading file descriptor.
    fn as_raw_read_fd(&self) -> RawFd;

    /// Extracts the raw file descriptor for writing.
    ///
    /// Like [`AsRawFd::as_raw_fd`], but returns the writing file descriptor.
    fn as_raw_write_fd(&self) -> RawFd;
}

/// Like [`AsRawHandleOrSocket`], but for types which may have one or two
/// handles or sockets, for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(windows)]
pub trait AsRawReadWriteHandleOrSocket {
    /// Extracts the raw handle or socket for reading.
    ///
    /// Like [`AsRawHandleOrSocket::as_raw_handle_or_socket`], but returns the
    /// reading handle.
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket;

    /// Extracts the raw handle or socket for writing.
    ///
    /// Like [`AsRawHandleOrSocket::as_raw_handle_or_socket`], but returns the
    /// writing handle.
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket;
}

#[cfg(not(windows))]
impl<T: AsRawReadWriteFd + OwnsRaw> AsUnsafeReadWriteHandle for T {
    #[inline]
    fn as_unsafe_read_handle(&self) -> UnsafeHandle {
        UnsafeHandle::unowned_from_raw_fd(self.as_raw_read_fd())
    }

    #[inline]
    fn as_unsafe_write_handle(&self) -> UnsafeHandle {
        UnsafeHandle::unowned_from_raw_fd(self.as_raw_write_fd())
    }
}

#[cfg(windows)]
impl<T: AsRawReadWriteHandleOrSocket + OwnsRaw> AsUnsafeReadWriteHandle for T {
    #[inline]
    fn as_unsafe_read_handle(&self) -> UnsafeHandle {
        UnsafeHandle::unowned_from_raw_handle_or_socket(self.as_raw_read_handle_or_socket())
    }

    #[inline]
    fn as_unsafe_write_handle(&self) -> UnsafeHandle {
        UnsafeHandle::unowned_from_raw_handle_or_socket(self.as_raw_write_handle_or_socket())
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

#[cfg(all(feature = "async-std", not(windows)))]
impl AsRawReadWriteFd for async_std::fs::File {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "async-std", windows))]
impl AsRawReadWriteHandleOrSocket for async_std::fs::File {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(feature = "async-std", not(windows)))]
impl AsRawReadWriteFd for async_std::net::TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "async-std", windows))]
impl AsRawReadWriteHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(feature = "async-std", unix))]
impl AsRawReadWriteFd for async_std::os::unix::net::UnixStream {
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

/// Adapt an `AsUnsafeReadWriteHandle` implementation to implement
/// `AsUnsafeHandle` with the read handle.
#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct ReadHalf<'a, RW: AsUnsafeReadWriteHandle>(pub &'a RW);

// Safety: `ReadHalf` implements `AsUnsafeHandle` if `RW` does.
unsafe impl<RW: AsUnsafeReadWriteHandle> AsUnsafeHandle for ReadHalf<'_, RW> {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        self.0.as_unsafe_read_handle()
    }
}

/// Adapt an `AsUnsafeReadWriteHandle` implementation to implement
/// `AsUnsafeHandle` with the write handle.
#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct WriteHalf<'a, RW: AsUnsafeReadWriteHandle>(pub &'a RW);

// Safety: `WriteHalf` implements `AsUnsafeHandle` if `RW` does.
unsafe impl<RW: AsUnsafeReadWriteHandle> AsUnsafeHandle for WriteHalf<'_, RW> {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        self.0.as_unsafe_write_handle()
    }
}
