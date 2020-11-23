//! The `UnsafeHandle` type and supporting API.

#[cfg(all(windows, feature = "os_pipe"))]
use os_pipe::{PipeReader, PipeWriter};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{
    fmt,
    fs::File,
    io::{self, IoSlice, IoSliceMut, Read, Write},
    mem::ManuallyDrop,
};
#[cfg(windows)]
use {
    crate::{AsRawHandleOrSocket, RawHandleOrSocket},
    std::{
        io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
        net::TcpStream,
        os::windows::io::{
            AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
            RawHandle, RawSocket,
        },
        process::{ChildStderr, ChildStdin, ChildStdout},
    },
};

/// A trait for types which contain an unsafe handle and can expose it.
pub trait AsUnsafeHandle {
    /// Return the contained unsafe handle.
    fn as_unsafe_handle(&self) -> UnsafeHandle;
}

/// A trait for types which can be converted into an unsafe handle.
pub trait IntoUnsafeHandle {
    /// Convert `self` into an unsafe handle.
    fn into_unsafe_handle(self) -> UnsafeHandle;
}

/// A trait for types which contain an unsafe file and can expose it.
pub trait AsUnsafeFile {
    /// Return the contained unsafe file.
    fn as_unsafe_file(&self) -> UnsafeFile;
}

/// A trait for types which can be converted into unsafe files.
pub trait IntoUnsafeFile {
    /// Convert `self` into an unsafe file.
    fn into_unsafe_file(self) -> UnsafeFile;
}

/// A trait for types which contain an unsafe socket and can expose it.
pub trait AsUnsafeSocket {
    /// Return the contained unsafe socket.
    fn as_unsafe_socket(&self) -> UnsafeSocket;
}

/// A trait for types which can be converted into unsafe sockets.
pub trait IntoUnsafeSocket {
    /// Convert `self` into an unsafe socket.
    fn into_unsafe_socket(self) -> UnsafeSocket;
}

/// A trait for types which can be constructed from unsafe files.
pub trait FromUnsafeFile {
    /// Convert an unsafe file into a `Self`.
    ///
    /// # Safety
    ///
    /// The return value of this function may be used to dereference the given
    /// unsafe handle without using unsafe, so the caller must ensure that it
    /// doesn't outlive the underlying resource.
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self;
}

/// A trait for types which can be constructed from unsafe sockets.
pub trait FromUnsafeSocket {
    /// Convert an unsafe socket into a `Self`.
    ///
    /// # Safety
    ///
    /// The return value of this function may be used to dereference the given
    /// unsafe handle without using unsafe, so the caller must ensure that it
    /// doesn't outlive the underlying resource.
    unsafe fn from_unsafe_socket(unsafe_socket: UnsafeSocket) -> Self;
}

/// A non-owning unsafe I/O handle.
///
/// On Posix-ish platforms this is just a `RawFd`. On Windows it is a minimal
/// abstraction over `RawHandle` and `RawSocket`. Similar to Rust raw pointers,
/// it is considered safe to construct these, but unsafe to do any I/O with
/// them (effectively dereferencing them).
#[derive(Copy, Clone)]
pub struct UnsafeHandle(Either);

/// A non-owning unsafe I/O handle that implements [`Read`]. `Read` functions are
/// considered safe, so this type requires `unsafe` to construct.
///
/// [`Read`]: std::io::Read
#[derive(Copy, Clone)]
pub struct UnsafeReadable(Either);

/// A non-owning unsafe I/O handle that implements [`Write`]. `Write` functions
/// considered are safe, so this type requires `unsafe` to construct.
///
/// [`Write`]: std::io::Write
#[derive(Copy, Clone)]
pub struct UnsafeWriteable(Either);

/// A non-owning unsafe I/O handle which on Windows is limited to handling what
/// Windows considers to be `RawHandle`s—mainly files and pipes.
#[derive(Copy, Clone)]
pub struct UnsafeFile(InnerFile);

/// A non-owning unsafe I/O handle which on Windows is limited to handling what
/// Windows considers to be `RawSocket`s—mainly TCP listeners and streams
/// and UDP sockets.
#[derive(Copy, Clone)]
pub struct UnsafeSocket(InnerSocket);

/// Posix-ish platforms use a single "file descriptor" type for all the kinds
/// of resources we're abstracting over, so we can just use that.
#[cfg(not(windows))]
type Either = RawFd;
#[cfg(not(windows))]
type InnerFile = RawFd;
#[cfg(not(windows))]
type InnerSocket = RawFd;

#[cfg(windows)]
type Either = RawHandleOrSocket;
#[cfg(windows)]
type InnerFile = RawHandle;
#[cfg(windows)]
type InnerSocket = RawSocket;

impl UnsafeHandle {
    /// Obtain `self` with a type that implements `Read`.
    ///
    /// # Safety
    ///
    /// The resulting value must not outlive the underlying resource.
    #[inline]
    pub unsafe fn as_readable(self) -> UnsafeReadable {
        UnsafeReadable(self.0)
    }

    /// Obtain `self` with a type that implements `Write`.
    ///
    /// # Safety
    ///
    /// The resulting value must not outlive the underlying resource.
    #[inline]
    pub unsafe fn as_writeable(self) -> UnsafeWriteable {
        UnsafeWriteable(self.0)
    }

    /// Like `FromRawFd::from_raw_fd`, but isn't unsafe because it doesn't
    /// imply a dereference.
    #[cfg(not(windows))]
    #[inline]
    pub fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }

    /// Like `FromRawHandle::from_raw_handle`, but isn't unsafe because it
    /// doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    pub fn from_raw_handle(raw_handle: RawHandle) -> Self {
        Self(RawHandleOrSocket::Handle(raw_handle))
    }

    /// Like `FromRawSocket::from_raw_socket`, but isn't unsafe because it
    /// doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    pub fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(RawHandleOrSocket::Socket(raw_socket))
    }

    /// Like `FromRawHandle::from_raw_handle` and
    /// `FromRawSocket::from_raw_socket` combined.
    #[cfg(windows)]
    #[inline]
    pub fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

impl UnsafeFile {
    /// Like `FromRawFd::from_raw_fd`, but isn't unsafe because it doesn't
    /// imply a dereference.
    #[cfg(not(windows))]
    #[inline]
    pub fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }

    /// Like `FromRawHandle::from_raw_handle`, but isn't unsafe because it
    /// doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    pub fn from_raw_handle(raw_handle: RawHandle) -> Self {
        Self(raw_handle)
    }
}

impl UnsafeSocket {
    /// Like `FromRawFd::from_raw_fd`, but isn't unsafe because it doesn't
    /// imply a dereference.
    #[cfg(not(windows))]
    #[inline]
    pub fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }

    /// Like `FromRawSocket::from_raw_socket`, but isn't unsafe because it
    /// doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    pub fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(raw_socket)
    }
}

impl UnsafeReadable {
    #[cfg(not(windows))]
    #[inline]
    unsafe fn as_file(&self) -> ManuallyDrop<File> {
        ManuallyDrop::new(File::from_raw_fd(self.0))
    }

    #[cfg(windows)]
    #[inline]
    unsafe fn as_file(raw_handle: RawHandle) -> ManuallyDrop<File> {
        ManuallyDrop::new(File::from_raw_handle(raw_handle))
    }

    #[cfg(windows)]
    #[inline]
    unsafe fn as_tcp_stream(raw_socket: RawSocket) -> ManuallyDrop<TcpStream> {
        ManuallyDrop::new(TcpStream::from_raw_socket(raw_socket))
    }
}

impl UnsafeWriteable {
    #[cfg(not(windows))]
    #[inline]
    unsafe fn as_file(&self) -> ManuallyDrop<File> {
        ManuallyDrop::new(File::from_raw_fd(self.0))
    }

    #[cfg(windows)]
    #[inline]
    unsafe fn as_file(raw_handle: RawHandle) -> ManuallyDrop<File> {
        ManuallyDrop::new(File::from_raw_handle(raw_handle))
    }

    #[cfg(windows)]
    #[inline]
    unsafe fn as_tcp_stream(raw_socket: RawSocket) -> ManuallyDrop<TcpStream> {
        ManuallyDrop::new(TcpStream::from_raw_socket(raw_socket))
    }
}

// Posix-ish implementations.

#[cfg(not(windows))]
impl<T: AsRawFd> AsUnsafeHandle for T {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        UnsafeHandle(self.as_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: AsRawFd> AsUnsafeFile for T {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(self.as_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: AsRawFd> AsUnsafeSocket for T {
    #[inline]
    fn as_unsafe_socket(&self) -> UnsafeSocket {
        UnsafeSocket(self.as_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: IntoRawFd> IntoUnsafeHandle for T {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle(self.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: IntoRawFd> IntoUnsafeFile for T {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(self.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: IntoRawFd> IntoUnsafeSocket for T {
    #[inline]
    fn into_unsafe_socket(self) -> UnsafeSocket {
        UnsafeSocket(self.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: FromRawFd> FromUnsafeFile for T {
    #[inline]
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self {
        Self::from_raw_fd(unsafe_file.0)
    }
}

#[cfg(not(windows))]
impl<T: FromRawFd> FromUnsafeSocket for T {
    #[inline]
    unsafe fn from_unsafe_socket(unsafe_socket: UnsafeSocket) -> Self {
        Self::from_raw_fd(unsafe_socket.0)
    }
}

#[cfg(not(windows))]
impl AsRawFd for UnsafeHandle {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl IntoRawFd for UnsafeHandle {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl AsRawFd for UnsafeReadable {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl AsRawFd for UnsafeWriteable {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl AsRawFd for UnsafeFile {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl IntoRawFd for UnsafeFile {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl AsRawFd for UnsafeSocket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

#[cfg(not(windows))]
impl IntoRawFd for UnsafeSocket {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

// Windows implementations.

#[cfg(windows)]
impl<T: AsRawHandleOrSocket> AsUnsafeHandle for T {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        UnsafeHandle(self.as_raw_handle_or_socket())
    }
}

#[cfg(windows)]
impl AsUnsafeFile for Stdin {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl<'a> AsUnsafeFile for StdinLock<'a> {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl AsUnsafeFile for Stdout {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl<'a> AsUnsafeFile for StdoutLock<'a> {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl AsUnsafeFile for Stderr {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl<'a> AsUnsafeFile for StderrLock<'a> {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeHandle for File {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl AsUnsafeFile for File {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeFile for File {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl FromUnsafeFile for File {
    #[inline]
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self {
        Self::from_raw_handle(unsafe_file.0)
    }
}

#[cfg(windows)]
impl IntoUnsafeHandle for ChildStdin {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl AsUnsafeFile for ChildStdin {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeFile for ChildStdin {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeHandle for ChildStdout {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl AsUnsafeFile for ChildStdout {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeFile for ChildStdout {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeHandle for ChildStderr {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl AsUnsafeFile for ChildStderr {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeFile for ChildStderr {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeHandle for TcpStream {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_socket(Self::into_raw_socket(self))
    }
}

#[cfg(windows)]
impl AsUnsafeSocket for TcpStream {
    #[inline]
    fn as_unsafe_socket(&self) -> UnsafeSocket {
        UnsafeSocket(AsRawSocket::as_raw_socket(self))
    }
}

#[cfg(windows)]
impl IntoUnsafeSocket for TcpStream {
    #[inline]
    fn into_unsafe_socket(self) -> UnsafeSocket {
        UnsafeSocket(IntoRawSocket::into_raw_socket(self))
    }
}

#[cfg(windows)]
impl FromUnsafeSocket for TcpStream {
    #[inline]
    unsafe fn from_unsafe_socket(unsafe_socket: UnsafeSocket) -> Self {
        TcpStream::from_raw_socket(unsafe_socket.0)
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl IntoUnsafeHandle for PipeReader {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl AsUnsafeFile for PipeReader {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl IntoUnsafeFile for PipeReader {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl FromUnsafeFile for PipeReader {
    #[inline]
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self {
        Self::from_raw_handle(unsafe_file.0)
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl IntoUnsafeHandle for PipeWriter {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl AsUnsafeFile for PipeWriter {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl IntoUnsafeFile for PipeWriter {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl FromUnsafeFile for PipeWriter {
    #[inline]
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self {
        Self::from_raw_handle(unsafe_file.0)
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeHandle {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeReadable {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeFile {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Handle(self.0)
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::Socket(self.0)
    }
}

#[cfg(not(windows))]
impl Read for UnsafeReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe { self.as_file() }.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        unsafe { self.as_file() }.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        unsafe { self.as_file() }.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unsafe { self.as_file() }.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        unsafe { self.as_file() }.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        unsafe { self.as_file() }.read_exact(buf)
    }
}

#[cfg(windows)]
impl Read for UnsafeReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => unsafe { Self::as_file(raw_handle) }.read(buf),
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.read(buf)
            }
        }
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.read_vectored(bufs)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.read_vectored(bufs)
            }
        }
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.is_read_vectored()
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.is_read_vectored()
            }
        }
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.read_to_end(buf)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.read_to_end(buf)
            }
        }
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.read_to_string(buf)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.read_to_string(buf)
            }
        }
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.read_exact(buf)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.read_exact(buf)
            }
        }
    }
}

#[cfg(not(windows))]
impl Write for UnsafeWriteable {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { self.as_file() }.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        unsafe { self.as_file() }.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        unsafe { self.as_file() }.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        unsafe { self.as_file() }.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        unsafe { self.as_file() }.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        unsafe { self.as_file() }.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
        unsafe { self.as_file() }.write_fmt(fmt)
    }
}

#[cfg(windows)]
impl Write for UnsafeWriteable {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.write(buf)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.write(buf)
            }
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => unsafe { Self::as_file(raw_handle) }.flush(),
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.flush()
            }
        }
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.write_vectored(bufs)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.write_vectored(bufs)
            }
        }
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.is_write_vectored()
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.is_write_vectored()
            }
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.write_all(buf)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.write_all(buf)
            }
        }
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.write_all_vectored(bufs)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.write_all_vectored(bufs)
            }
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
        match self.0 {
            RawHandleOrSocket::Handle(raw_handle) => {
                unsafe { Self::as_file(raw_handle) }.write_fmt(fmt)
            }
            RawHandleOrSocket::Socket(raw_socket) => {
                unsafe { Self::as_tcp_stream(raw_socket) }.write_fmt(fmt)
            }
        }
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("UnsafeHandle");

        // Just print the fd number; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        b.field("raw_fd", &self.as_raw_fd());

        b.finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("UnsafeHandle");

        // Just print the raw handles; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        b.field("raw_handle_or_socket", &self.as_raw_handle_or_socket());

        b.finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("UnsafeFile");

        // Just print the fd number; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        b.field("raw_fd", &self.0);

        b.finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("UnsafeFile");

        // Just print the raw handles; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        b.field("raw_handle", &self.0);

        b.finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("UnsafeSocket");

        // Just print the fd number; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        b.field("raw_fd", &self.0);

        b.finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("UnsafeSocket");

        // Just print the raw sockets; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        b.field("raw_socket", &self.0);

        b.finish()
    }
}
