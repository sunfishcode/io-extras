//! `HandleOrSocket` variants of `{As,Into}{Handle,Socket}`.

use super::types::{BorrowedHandleOrSocket, OwnedHandleOrSocket};
use super::AsRawHandleOrSocket;
use io_lifetimes::{AsHandle, AsSocket, IntoHandle, IntoSocket};
#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
use std::{
    fs::File,
    io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    net::{TcpListener, TcpStream, UdpSocket},
    process::{ChildStderr, ChildStdin, ChildStdout},
};

/// Like [`AsHandle`] and [`AsSocket`], but implementable by types which
/// can implement either one.
pub trait AsHandleOrSocket<'a> {
    /// Like [`AsHandle::as_handle`] and [`AsSocket::as_socket`]
    /// but can return either type.
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a>;
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
    fn from_raw_handle_or_socket(handle_or_socket: OwnedHandleOrSocket) -> Self;
}

impl<'a> AsHandleOrSocket<'a> for &'a OwnedHandleOrSocket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        unsafe {
            BorrowedHandleOrSocket::borrow_raw_handle_or_socket(self.as_raw_handle_or_socket())
        }
    }
}

impl<'a> AsHandleOrSocket<'a> for BorrowedHandleOrSocket<'a> {
    #[inline]
    fn as_handle_or_socket(self) -> Self {
        self
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a Stdin {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a StdinLock<'_> {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a Stdout {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a StdoutLock<'_> {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a Stderr {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a StderrLock<'_> {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a File {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a ChildStdin {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a ChildStdout {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a ChildStderr {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a TcpStream {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a TcpListener {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

impl<'a> AsHandleOrSocket<'a> for &'a UdpSocket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::io::Stdin {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::io::Stdout {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::io::Stderr {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::fs::File {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

// async_std's `ChildStdin`, `ChildStdout`, and `ChildStderr` don't implement
// `AsFd` or `AsHandle`.

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "async-std")]
impl<'a> AsHandleOrSocket<'a> for &'a async_std::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::io::Stdin {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::io::Stdout {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::io::Stderr {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::fs::File {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::process::ChildStdin {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::process::ChildStdout {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "tokio")]
impl<'a> AsHandleOrSocket<'a> for &'a tokio::process::ChildStderr {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl<'a> AsHandleOrSocket<'a> for &'a PipeReader {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl<'a> AsHandleOrSocket<'a> for &'a PipeWriter {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_handle(Self::as_handle(self))
    }
}

#[cfg(feature = "socket2")]
impl<'a> AsHandleOrSocket<'a> for &'a socket2::Socket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl<'a> AsHandleOrSocket<'a> for &'a mio::net::TcpStream {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl<'a> AsHandleOrSocket<'a> for &'a mio::net::TcpListener {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl<'a> AsHandleOrSocket<'a> for &'a mio::net::TcpSocket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
        BorrowedHandleOrSocket::from_socket(Self::as_socket(self))
    }
}

#[cfg(feature = "use_mio_net")]
impl<'a> AsHandleOrSocket<'a> for &'a mio::net::UdpSocket {
    #[inline]
    fn as_handle_or_socket(self) -> BorrowedHandleOrSocket<'a> {
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
impl IntoHandleOrSocket for mio::net::TcpSocket {
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
