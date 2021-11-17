//! `HandleOrSocket` variants of `{As,Into}{Handle,Socket}`.

use super::types::{BorrowedHandleOrSocket, OwnedHandleOrSocket};
use super::AsRawHandleOrSocket;
use io_lifetimes::{AsHandle, AsSocket, IntoHandle, IntoSocket};
#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
use std::fs::File;
use std::io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::process::{ChildStderr, ChildStdin, ChildStdout};

/// Like [`AsHandle`] and [`AsSocket`], but implementable by types which
/// can implement either one.
pub trait AsHandleOrSocket {
    /// Like [`AsHandle::as_handle`] and [`AsSocket::as_socket`]
    /// but can return either type.
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_>;
}

/// Like [`IntoHandle`] and [`IntoSocket`], but implementable by types
/// which can implement either one.
pub trait IntoHandleOrSocket {
    /// Like [`IntoHandle::into_handle`] and
    /// [`IntoSocket::into_socket`] but can return either type.
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket;
}

/// Like [`FromHandle`] and [`FromSocket`], but implementable by types
/// which can implement both.
///
/// Note: Don't implement this trait for types which can only implement one
/// or the other, such that it would need to panic if passed the wrong form.
///
/// [`FromHandle`]: io_lifetimes::FromHandle
/// [`FromSocket`]: io_lifetimes::FromSocket
pub trait FromHandleOrSocket {
    /// Like [`FromHandle::from_handle`] and
    /// [`FromSocket::from_socket`] but can be passed either type.
    ///
    /// # Safety
    ///
    /// `handle_or_socket` must be valid and otherwise unowned.
    ///
    /// [`FromHandle::from_handle`]: io_lifetimes::FromHandle::from_handle
    /// [`FromSocket::from_socket`]: io_lifetimes::FromSocket::from_socket
    fn from_handle_or_socket(handle_or_socket: OwnedHandleOrSocket) -> Self;
}

impl AsHandleOrSocket for OwnedHandleOrSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        unsafe {
            BorrowedHandleOrSocket::borrow_raw_handle_or_socket(self.as_raw_handle_or_socket())
        }
    }
}

impl AsHandleOrSocket for BorrowedHandleOrSocket<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        *self
    }
}

impl AsHandleOrSocket for Stdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for StdinLock<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for Stdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for StdoutLock<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for Stderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for StderrLock<'_> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for ChildStdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for ChildStdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for ChildStderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl AsHandleOrSocket for TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl AsHandleOrSocket for TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl AsHandleOrSocket for UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::io::Stdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::io::Stdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::io::Stderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::fs::File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

// async_std's `ChildStdin`, `ChildStdout`, and `ChildStderr` don't implement
// `AsFd` or `AsHandle`.

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl AsHandleOrSocket for async_std::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::io::Stdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::io::Stdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::io::Stderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::fs::File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::process::ChildStdin {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::process::ChildStdout {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl AsHandleOrSocket for tokio::process::ChildStderr {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsHandleOrSocket for PipeReader {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsHandleOrSocket for PipeWriter {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "socket2")]
impl AsHandleOrSocket for socket2::Socket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl AsHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl AsHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl AsHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl IntoHandleOrSocket for File {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

impl IntoHandleOrSocket for ChildStdin {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

impl IntoHandleOrSocket for ChildStdout {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

impl IntoHandleOrSocket for ChildStderr {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

impl IntoHandleOrSocket for TcpStream {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

impl IntoHandleOrSocket for TcpListener {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

impl IntoHandleOrSocket for UdpSocket {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "os_pipe")]
impl IntoHandleOrSocket for PipeReader {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl IntoHandleOrSocket for PipeWriter {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

#[cfg(feature = "socket2")]
impl IntoHandleOrSocket for socket2::Socket {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl IntoHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl IntoHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl IntoHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl IntoHandleOrSocket for async_std::fs::File {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_handle(Self::into_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl IntoHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl IntoHandleOrSocket for async_std::net::TcpListener {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl IntoHandleOrSocket for async_std::net::UdpSocket {
    #[inline]
    fn into_handle_or_socket(self) -> OwnedHandleOrSocket {
        OwnedHandleOrSocket::from_socket(Self::into_socket(self))
    }
}
