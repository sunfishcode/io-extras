//! `OwnedReadable` and `OwnedWriteable`.

use crate::grip::{AsRawGrip, FromRawGrip, OwnedGrip};
use crate::raw::{RawReadable, RawWriteable};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
use std::fmt;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
#[cfg(all(doc, not(windows)))]
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use {
    crate::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, FromHandleOrSocket,
        FromRawHandleOrSocket, IntoHandleOrSocket, IntoRawHandleOrSocket, OwnedHandleOrSocket,
    },
    io_lifetimes::{FromHandle, OwnedHandle},
    std::os::windows::io::{FromRawHandle, IntoRawHandle},
};

/// An owning I/O handle that implements [`Read`].
///
/// This doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this reads from the handle as if it were a
/// [`File`]. On Windows, this reads from a file-like handle as if it were a
/// [`File`], and from a socket-like handle as if it were a [`TcpStream`].
#[repr(transparent)]
pub struct OwnedReadable(RawReadable);

/// An owning I/O handle that implements [`Write`].
///
/// This doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this writes to the handle as if it were a
/// [`File`]. On Windows, this writes to a file-like handle as if it were a
/// [`File`], and to a socket-like handle as if it were a [`TcpStream`].
#[repr(transparent)]
pub struct OwnedWriteable(RawWriteable);

/// `OwnedReadable` owns its handle.
#[cfg(not(windows))]
impl AsFd for OwnedReadable {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.0.as_raw_fd()) }
    }
}

/// `OwnedReadable` owns its handle.
#[cfg(not(windows))]
impl IntoFd for OwnedReadable {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.0.into_raw_fd()) }
    }
}

/// `OwnedReadable` owns its handle.
#[cfg(not(windows))]
impl FromFd for OwnedReadable {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        unsafe { Self(RawReadable::from_raw_fd(fd.into_raw_fd())) }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(not(windows))]
impl AsFd for OwnedWriteable {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.0.as_raw_fd()) }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(not(windows))]
impl IntoFd for OwnedWriteable {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.0.as_raw_fd()) }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(not(windows))]
impl FromFd for OwnedWriteable {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        unsafe { Self(RawWriteable::from_raw_fd(fd.into_raw_fd())) }
    }
}

// Windows implementations.

/// `OwnedReadable` owns its handle.
#[cfg(windows)]
impl AsHandleOrSocket for OwnedReadable {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        unsafe { BorrowedHandleOrSocket::borrow_raw(self.0.as_raw_handle_or_socket()) }
    }
}

/// `OwnedReadable` owns its handle.
#[cfg(windows)]
impl IntoHandleOrSocket for OwnedReadable {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        unsafe {
            OwnedHandleOrSocket::from_raw_handle_or_socket(self.0.into_raw_handle_or_socket())
        }
    }
}

/// `OwnedReadable` owns its handle.
#[cfg(windows)]
impl FromHandleOrSocket for OwnedReadable {
    #[inline]
    fn from_handle_or_socket(handle_or_socket: OwnedHandleOrSocket) -> Self {
        unsafe {
            Self(RawReadable::from_raw_handle_or_socket(
                handle_or_socket.into_raw_handle_or_socket(),
            ))
        }
    }
}

/// `OwnedReadable` owns its handle.
#[cfg(windows)]
impl FromHandle for OwnedReadable {
    #[inline]
    fn from_handle(handle: OwnedHandle) -> Self {
        unsafe { Self(RawReadable::from_raw_handle(handle.into_raw_handle())) }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(windows)]
impl AsHandleOrSocket for OwnedWriteable {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        unsafe { BorrowedHandleOrSocket::borrow_raw(self.0.as_raw_handle_or_socket()) }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(windows)]
impl IntoHandleOrSocket for OwnedWriteable {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        unsafe {
            OwnedHandleOrSocket::from_raw_handle_or_socket(self.0.into_raw_handle_or_socket())
        }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(windows)]
impl FromHandleOrSocket for OwnedWriteable {
    #[inline]
    fn from_handle_or_socket(handle_or_socket: OwnedHandleOrSocket) -> Self {
        unsafe {
            Self(RawWriteable::from_raw_handle_or_socket(
                handle_or_socket.into_raw_handle_or_socket(),
            ))
        }
    }
}

/// `OwnedWriteable` owns its handle.
#[cfg(windows)]
impl FromHandle for OwnedWriteable {
    #[inline]
    fn from_handle(handle: OwnedHandle) -> Self {
        unsafe { Self(RawWriteable::from_raw_handle(handle.into_raw_handle())) }
    }
}

#[cfg(not(windows))]
impl Read for OwnedReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.0.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buf)
    }
}

#[cfg(windows)]
impl Read for OwnedReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.0.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buf)
    }
}

#[cfg(not(windows))]
impl Write for OwnedWriteable {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        self.0.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.0.write_fmt(fmt)
    }
}

#[cfg(windows)]
impl Write for OwnedWriteable {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        self.0.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.0.write_fmt(fmt)
    }
}

impl Drop for OwnedReadable {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _owned = OwnedGrip::from_raw_grip(self.0.as_raw_grip());
        }
    }
}

impl Drop for OwnedWriteable {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _owned = OwnedGrip::from_raw_grip(self.0.as_raw_grip());
        }
    }
}

#[cfg(not(windows))]
impl fmt::Debug for OwnedReadable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw fd number.
        f.debug_struct("OwnedReadable")
            .field("fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for OwnedReadable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket.
        f.debug_struct("OwnedReadable")
            .field("handle_or_socket", &self.0)
            .finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for OwnedWriteable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw fd number.
        f.debug_struct("OwnedWriteable")
            .field("fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for OwnedWriteable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket.
        f.debug_struct("OwnedWriteable")
            .field("handle_or_socket", &self.0)
            .finish()
    }
}
