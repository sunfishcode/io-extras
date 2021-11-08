use crate::grip::RawGrip;
#[cfg(not(windows))]
use crate::os::rustix::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use io_lifetimes::views::FilelikeView;
#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
use std::fmt;
use std::fs::File;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
#[cfg(windows)]
use {
    crate::os::windows::{
        AsRawHandleOrSocket, FromRawHandleOrSocket, IntoRawHandleOrSocket, RawEnum,
        RawHandleOrSocket,
    },
    std::net::TcpStream,
    std::os::windows::io::{
        AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
        RawHandle, RawSocket,
    },
};

/// A non-owning unsafe I/O handle that implements [`Read`]. `Read` functions
/// are considered safe, so this type requires `unsafe` to construct.
///
/// This doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this reads from the handle as if it were a
/// [`File`]. On Windows, this reads from a file-like handle as if it were a
/// [`File`], and from a socket-like handle as if it were a [`TcpStream`].
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeReadable(RawGrip);

/// A non-owning unsafe I/O handle that implements [`Write`]. `Write` functions
/// considered are safe, so this type requires `unsafe` to construct.
///
/// This doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this writes to the handle as if it were a
/// [`File`]. On Windows, this writes to a file-like handle as if it were a
/// [`File`], and to a socket-like handle as if it were a [`TcpStream`].
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeWriteable(RawGrip);

/// `UnsafeReadable` doesn't own its handle.
#[cfg(not(windows))]
impl AsRawFd for UnsafeReadable {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeReadable {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle.
#[cfg(not(windows))]
impl FromRawFd for UnsafeReadable {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

/// `UnsafeWriteable` doesn't own its handle.
#[cfg(not(windows))]
impl AsRawFd for UnsafeWriteable {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeWriteable {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle.
#[cfg(not(windows))]
impl FromRawFd for UnsafeWriteable {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

// Windows implementations.

/// `UnsafeReadable` doesn't own its handle.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeReadable {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeReadable {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle.
#[cfg(windows)]
impl FromRawHandleOrSocket for UnsafeReadable {
    #[inline]
    unsafe fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

/// `UnsafeWriteable` doesn't own its handle.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle.
#[cfg(windows)]
impl FromRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    unsafe fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

#[inline]
unsafe fn as_file_view<'a>(file: RawFd) -> FilelikeView<'a, File> {
    FilelikeView::<'a>::view_raw(file)
}

#[cfg(windows)]
#[inline]
unsafe fn as_socket_view<'a>(socket: RawSocket) -> SocketlikeView<'a, TcpStream> {
    SocketlikeView::<'a>::view_raw(socket)
}

#[cfg(not(windows))]
impl Read for UnsafeReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Safety: The caller of `as_readable()`, which is unsafe, is expected
        // to ensure that the underlying resource outlives this
        // `UnsafeReadable`.
        unsafe { as_file_view(self.0) }.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsafe { as_file_view(self.0) }.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        unsafe { as_file_view(self.0) }.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unsafe { as_file_view(self.0) }.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        unsafe { as_file_view(self.0) }.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        unsafe { as_file_view(self.0) }.read_exact(buf)
    }
}

#[cfg(windows)]
impl Read for UnsafeReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.read(buf),
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.read(buf)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.read(buf),
        }
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.read_vectored(bufs)
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.read_vectored(bufs)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.read_vectored(bufs),
        }
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.is_read_vectored()
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.is_read_vectored()
            }
            RawEnum::Stdio(ref stdio) => stdio.is_read_vectored(),
        }
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.read_to_end(buf)
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.read_to_end(buf)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.read_to_end(buf),
        }
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.read_to_string(buf)
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.read_to_string(buf)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.read_to_string(buf),
        }
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.read_exact(buf)
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.read_exact(buf)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.read_exact(buf),
        }
    }
}

#[cfg(not(windows))]
impl Write for UnsafeWriteable {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Safety: The caller of `as_writeable()`, which is unsafe, is expected
        // to ensure that the underlying resource outlives this
        // `UnsafeReadable`.
        unsafe { as_file_view(self.0) }.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        unsafe { as_file_view(self.0) }.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsafe { as_file_view(self.0) }.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        unsafe { as_file_view(self.0) }.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        unsafe { as_file_view(self.0) }.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        unsafe { as_file_view(self.0) }.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        unsafe { as_file_view(self.0) }.write_fmt(fmt)
    }
}

#[cfg(windows)]
impl Write for UnsafeWriteable {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.write(buf),
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write(buf)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.write(buf),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.flush(),
            RawEnum::Socket(raw_socket) => unsafe { as_tcp_stream_view(self, raw_socket) }.flush(),
            RawEnum::Stdio(ref mut stdio) => stdio.flush(),
        }
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.write_vectored(bufs)
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write_vectored(bufs)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.write_vectored(bufs),
        }
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.is_write_vectored()
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.is_write_vectored()
            }
            RawEnum::Stdio(ref stdio) => stdio.is_write_vectored(),
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.write_all(buf),
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write_all(buf)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.write_all(buf),
        }
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => {
                unsafe { as_file_view(self, raw_handle) }.write_all_vectored(bufs)
            }
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write_all_vectored(bufs)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.write_all_vectored(bufs),
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.write_fmt(fmt),
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write_fmt(fmt)
            }
            RawEnum::Stdio(ref mut stdio) => stdio.write_fmt(fmt),
        }
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeReadable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw fd number.
        f.debug_struct("UnsafeReadable")
            .field("raw_fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeReadable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket.
        f.debug_struct("UnsafeReadable")
            .field("raw_handle_or_socket", &self.0)
            .finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeWriteable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw fd number.
        f.debug_struct("UnsafeWriteable")
            .field("raw_fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeWriteable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket.
        f.debug_struct("UnsafeWriteable")
            .field("raw_handle_or_socket", &self.0)
            .finish()
    }
}
