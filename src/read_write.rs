//! This defines [`AsUnsafeReadWriteHandle`], and supporting utilities, which
//! is similar to [`AsUnsafeHandle`] except that instead of just having one
//! `as_unsafe_handle` function, it has separate `as_unsafe_read_handle` and
//! `as_unsafe_write_handle` functions, so that it can be implemented by
//! types which contain two handles, one for reading and one for writing.

#[cfg(windows)]
use crate::os::windows::{
    AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, RawHandleOrSocket,
};
use crate::{AsUnsafeHandle, OwnsRaw, UnsafeHandle};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::{fs::File, net::TcpStream};
#[cfg(not(windows))]
use {
    crate::os::rsix::{AsRawFd, RawFd},
    io_lifetimes::{AsFd, BorrowedFd},
};

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

/// Like [`AsFd`], but for types which may have one or two file descriptors,
/// for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(not(windows))]
pub trait AsReadWriteFd {
    /// Extracts the file descriptor for reading.
    ///
    /// Like [`AsFd::as_fd`], but returns the reading file descriptor.
    fn as_read_fd(&self) -> BorrowedFd<'_>;

    /// Extracts the file descriptor for writing.
    ///
    /// Like [`AsFd::as_fd`], but returns the writing file descriptor.
    fn as_write_fd(&self) -> BorrowedFd<'_>;
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

/// Like [`AsHandleOrSocket`], but for types which may have one or two
/// handles or sockets, for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(windows)]
pub trait AsReadWriteHandleOrSocket {
    /// Extracts the handle or socket for reading.
    ///
    /// Like [`AsHandleOrSocket::as_handle_or_socket`], but returns the
    /// reading handle.
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_>;

    /// Extracts the handle or socket for writing.
    ///
    /// Like [`AsHandleOrSocket::as_handle_or_socket`], but returns the
    /// writing handle.
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_>;
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

#[cfg(not(windows))]
impl AsReadWriteFd for File {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
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

#[cfg(windows)]
impl AsReadWriteHandleOrSocket for File {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
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

#[cfg(not(windows))]
impl AsReadWriteFd for TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
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

#[cfg(windows)]
impl AsReadWriteHandleOrSocket for TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
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

#[cfg(unix)]
impl AsReadWriteFd for UnixStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
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

#[cfg(all(feature = "async-std", not(windows)))]
impl AsReadWriteFd for async_std::fs::File {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
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

#[cfg(all(feature = "async-std", windows))]
impl AsReadWriteHandleOrSocket for async_std::fs::File {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
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

#[cfg(all(feature = "async-std", not(windows)))]
impl AsReadWriteFd for async_std::net::TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
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

#[cfg(all(feature = "async-std", windows))]
impl AsReadWriteHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
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

#[cfg(all(feature = "async-std", unix))]
impl AsReadWriteFd for async_std::os::unix::net::UnixStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsRawReadWriteFd for tokio::fs::File {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsReadWriteFd for tokio::fs::File {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "tokio", windows))]
impl AsRawReadWriteHandleOrSocket for tokio::fs::File {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(feature = "tokio", windows))]
impl AsReadWriteHandleOrSocket for tokio::fs::File {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsRawReadWriteFd for tokio::net::TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsReadWriteFd for tokio::net::TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "tokio", windows))]
impl AsReadWriteHandleOrSocket for tokio::net::TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(feature = "tokio", unix))]
impl AsRawReadWriteFd for tokio::net::UnixStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "tokio", unix))]
impl AsReadWriteFd for tokio::net::UnixStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
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

#[cfg(not(windows))]
impl<T: AsReadWriteFd> AsReadWriteFd for Box<T> {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        (**self).as_read_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        (**self).as_write_fd()
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

#[cfg(windows)]
impl<T: AsReadWriteHandleOrSocket> AsReadWriteHandleOrSocket for Box<T> {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        (**self).as_read_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        (**self).as_write_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "socket2"))]
impl AsRawReadWriteFd for socket2::Socket {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "socket2"))]
impl AsReadWriteFd for socket2::Socket {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "socket2"))]
impl AsRawReadWriteHandleOrSocket for socket2::Socket {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "socket2"))]
impl AsReadWriteHandleOrSocket for socket2::Socket {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::TcpListener {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::TcpListener {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsReadWriteHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsReadWriteHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::TcpSocket {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::TcpSocket {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::TcpSocket {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsReadWriteHandleOrSocket for mio::net::TcpSocket {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::UdpSocket {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::UdpSocket {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsReadWriteHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

/// Adapt an `AsUnsafeReadWriteHandle` implementation to implement
/// `AsUnsafeHandle` with the read handle.
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Copy, Clone)]
pub struct ReadHalf<'a, RW>(pub &'a RW);

// Safety: `ReadHalf` implements `AsUnsafeHandle` if `RW` does.
unsafe impl<RW: AsUnsafeReadWriteHandle> AsUnsafeHandle for ReadHalf<'_, RW> {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        self.0.as_unsafe_read_handle()
    }
}

#[cfg(not(windows))]
impl<RW: AsReadWriteFd> AsFd for ReadHalf<'_, RW> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_read_fd()
    }
}

#[cfg(windows)]
impl<RW: AsReadWriteHandleOrSocket> AsHandleOrSocket for ReadHalf<'_, RW> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.0.as_read_handle_or_socket()
    }
}

/// Adapt an `AsUnsafeReadWriteHandle` implementation to implement
/// `AsUnsafeHandle` with the write handle.
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Copy, Clone)]
pub struct WriteHalf<'a, RW>(pub &'a RW);

// Safety: `WriteHalf` implements `AsUnsafeHandle` if `RW` does.
unsafe impl<RW: AsUnsafeReadWriteHandle> AsUnsafeHandle for WriteHalf<'_, RW> {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        self.0.as_unsafe_write_handle()
    }
}

#[cfg(not(windows))]
impl<RW: AsReadWriteFd> AsFd for WriteHalf<'_, RW> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_write_fd()
    }
}

#[cfg(windows)]
impl<RW: AsReadWriteHandleOrSocket> AsHandleOrSocket for WriteHalf<'_, RW> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.0.as_write_handle_or_socket()
    }
}
