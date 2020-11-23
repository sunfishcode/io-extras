//! The `RawHandleOrSocket` type, providing a minimal Windows analog for the
//! Posix-ish `AsRawFd` type.

#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
use std::{
    fmt,
    fs::File,
    io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    net::TcpStream,
    os::windows::io::{AsRawHandle, AsRawSocket, RawHandle, RawSocket},
    process::{ChildStderr, ChildStdin, ChildStdout},
};

/// Windows has multiple types, so we use an enum to abstract over them. It's
/// reasonable to worry that this might be trying too hard to make Windows work
/// like Unix, however in this case, the number of types is small, so the enum
/// is simple and the overhead is relatively low, and the benefit is that we
/// can abstract over all the major `Read` and `Write` resources.
#[derive(Copy, Clone)]
pub enum RawHandleOrSocket {
    /// A `RawHandle`.
    Handle(RawHandle),

    /// A `RawSocket`.
    Socket(RawSocket),
}

/// The windows `HANDLE` and `SOCKET` types may be sent between threads.
unsafe impl Send for RawHandleOrSocket {}

/// Like [`std::os::windows::io::AsRawHandle`] and
/// [`std::os::windows::io::AsRawSocket`], but implementable by types which
/// can implement either one.
pub trait AsRawHandleOrSocket {
    /// Like [`std::os::windows::io::AsRawHandle::as_raw_handle`] and
    /// [`std::os::windows::io::AsRawSocket::as_raw_socket`] but can return
    /// either type.
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket;
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

#[cfg(windows)]
impl fmt::Debug for RawHandleOrSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("RawHandleOrSocket");

        // Just print the raw sockets; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        match self {
            RawHandleOrSocket::Handle(raw_handle) => b.field("raw_handle", &raw_handle),
            RawHandleOrSocket::Socket(raw_socket) => b.field("raw_socket", &raw_socket),
        };

        b.finish()
    }
}
