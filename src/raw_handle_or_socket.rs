//! The `RawHandleOrSocket` type and `AsRawHandleOrSocket` trait, providing a
//! minimal Windows analog for the Posix-ish `RawFd` type and `AsRawFd` trait.

#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
use std::{
    fs::File,
    io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    net::{TcpListener, TcpStream, UdpSocket},
    os::windows::io::{
        AsRawHandle, AsRawSocket, IntoRawHandle, IntoRawSocket, RawHandle, RawSocket,
    },
    process::{ChildStderr, ChildStdin, ChildStdout},
};

/// A Windows analog for the Posix-ish `AsRawFd` type. Unlike Posix-ish
/// platforms which have a single type for files and sockets, Windows has
/// distinct types, so we use an enum to abstract over them.
///
/// It's reasonable to worry that this might be trying too hard to make Windows
/// work like Unix, however in this case, the number of types is small, so the
/// enum is simple and the overhead is relatively low, and the benefit is that
/// we can abstract over major [`Read`] and [`Write`] resources.
///
/// [`Read`]: std::io::Read
/// [`Write`]: std::io::Write
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub enum RawHandleOrSocket {
    /// A `RawHandle`.
    Handle(RawHandle),

    /// A `RawSocket`.
    Socket(RawSocket),
}

/// Like [`AsRawHandle`] and [`AsRawSocket`], but implementable by types which
/// can implement either one.
pub trait AsRawHandleOrSocket {
    /// Like [`AsRawHandle::as_raw_handle`] and [`AsRawSocket::as_raw_socket`]
    /// but can return either type.
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket;
}

/// Like [`IntoRawHandle`] and [`IntoRawSocket`], but implementable by types
/// which can implement either one.
pub trait IntoRawHandleOrSocket {
    /// Like [`IntoRawHandle::into_raw_handle`] and
    /// [`IntoRawSocket::into_raw_socket`] but can return either type.
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket;
}

/// The Windows [`HANDLE`] and [`SOCKET`] types may be sent between threads.
///
/// [`HANDLE`]: https://doc.rust-lang.org/std/os/windows/raw/type.HANDLE.html
/// [`SOCKET`]: https://doc.rust-lang.org/std/os/windows/raw/type.SOCKET.html
unsafe impl Send for RawHandleOrSocket {}

/// The Windows `HANDLE` and `SOCKET` types may be shared between threads.
///
/// [`HANDLE`]: https://doc.rust-lang.org/std/os/windows/raw/type.HANDLE.html
/// [`SOCKET`]: https://doc.rust-lang.org/std/os/windows/raw/type.SOCKET.html
unsafe impl Sync for RawHandleOrSocket {}

impl AsRawHandleOrSocket for RawHandleOrSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> Self {
        *self
    }
}

impl AsRawHandleOrSocket for Stdin {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl<'a> AsRawHandleOrSocket for StdinLock<'a> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for Stdout {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl<'a> AsRawHandleOrSocket for StdoutLock<'a> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for Stderr {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl<'a> AsRawHandleOrSocket for StderrLock<'a> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for File {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for ChildStdin {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for ChildStdout {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for ChildStderr {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for TcpStream {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(Self::as_raw_socket(self))
    }
}

impl AsRawHandleOrSocket for TcpListener {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(Self::as_raw_socket(self))
    }
}

impl AsRawHandleOrSocket for UdpSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(Self::as_raw_socket(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsRawHandleOrSocket for PipeReader {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsRawHandleOrSocket for PipeWriter {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::as_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for File {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for ChildStdin {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for ChildStdout {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for ChildStderr {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for TcpStream {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(Self::into_raw_socket(self))
    }
}

impl IntoRawHandleOrSocket for TcpListener {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(Self::into_raw_socket(self))
    }
}

impl IntoRawHandleOrSocket for UdpSocket {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(Self::into_raw_socket(self))
    }
}

#[cfg(feature = "os_pipe")]
impl IntoRawHandleOrSocket for PipeReader {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::into_raw_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl IntoRawHandleOrSocket for PipeWriter {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(Self::into_raw_handle(self))
    }
}
