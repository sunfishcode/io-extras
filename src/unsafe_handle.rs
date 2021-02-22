//! The [`UnsafeHandle`] type and supporting types and traits.

#[cfg(not(windows))]
use crate::os::posish::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::OwnsRaw;
#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::{
    fmt,
    fs::File,
    io::{self, IoSlice, IoSliceMut, Read, Write},
    marker::PhantomData,
    mem::ManuallyDrop,
    net::{TcpListener, TcpStream, UdpSocket},
    ops::{Deref, DerefMut},
};
#[cfg(windows)]
use {
    crate::os::windows::{
        AsRawHandleOrSocket, FromRawHandleOrSocket, IntoRawHandleOrSocket, RawEnum,
        RawHandleOrSocket,
    },
    std::{
        os::windows::io::{
            AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
            RawHandle, RawSocket,
        },
        process::{ChildStderr, ChildStdin, ChildStdout},
    },
};

/// A trait for types which contain an unsafe handle and can expose it.
///
/// A type implementing `AsUnsafeHandle` guarantees that the return value from
/// `as_unsafe_handle` on an instance of the type is a copy of a handle which
/// is owned.
///
/// # Safety
///
/// This trait is `unsafe` because types implementing it must guarantee they
/// own their handle.
#[allow(clippy::module_name_repetitions)]
pub unsafe trait AsUnsafeHandle {
    /// Return the contained unsafe handle.
    fn as_unsafe_handle(&self) -> UnsafeHandle;

    /// Test whether `self.as_unsafe_handle()` is equal to
    /// `other.as_unsafe_handle()`.
    ///
    /// That this depends on the guarantee that types that implement
    /// `AsUnsafeHandle` own their resources, so we won't erroneously compare
    /// dangling handles.
    #[inline]
    fn eq_handle<Handlelike: AsUnsafeHandle>(&self, other: &Handlelike) -> bool {
        unsafe { self.as_unsafe_handle().eq(other.as_unsafe_handle()) }
    }
}

/// A trait for types which can be converted into an unsafe handle.
///
/// A type implementing `IntoUnsafeHandle` guarantees that the return value
/// from `into_unsafe_handle` on an instance of the type is a copy of a handle
/// which is owned by that instance.
///
/// # Safety
///
/// This trait is `unsafe` because types implementing it must guarantee they
/// own their handle.
#[allow(clippy::module_name_repetitions)]
pub unsafe trait IntoUnsafeHandle {
    /// Convert `self` into an unsafe handle.
    fn into_unsafe_handle(self) -> UnsafeHandle;
}

/// A trait for types which can be constructed from an unsafe handle.
///
/// Note: Don't implement this trait for types which require an
/// `UnsafeFile`-like handle or an `UnsafeSocket`-like handle, such that it
/// would need to panic if passed the wrong form.
#[allow(clippy::module_name_repetitions)]
pub trait FromUnsafeHandle {
    /// Constructs a new instance of `Self` from the given unsafe handle.
    ///
    /// # Safety
    ///
    /// This function consumes ownership of the specified file descriptor. The
    /// returned object will take responsibility for closing it when the object
    /// goes out of scope.
    unsafe fn from_unsafe_handle(unsafe_handle: UnsafeHandle) -> Self;
}

/// A trait for types which contain an unsafe file and can expose it.
///
/// A type implementing `AsUnsafeFile` guarantees that the return value
/// from `as_unsafe_file` on an instance of the type is a copy of a handle
/// which is owned.
///
/// # Safety
///
/// This trait is `unsafe` because types implementing it must guarantee they
/// own their handle.
pub unsafe trait AsUnsafeFile: AsUnsafeHandle {
    /// Return the contained unsafe file.
    fn as_unsafe_file(&self) -> UnsafeFile;

    /// Utility for returning a value which dereferences to a `&File` or
    /// `&mut File`.
    ///
    /// Note that `AsUnsafeFile` may be implemented for types which are not
    /// normal files, and which don't support all the methods on `File`.
    #[inline]
    fn as_file_view(&self) -> View<'_, File> {
        let unsafe_file = self.as_unsafe_file();
        let file = unsafe { File::from_unsafe_file(unsafe_file) };
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_file_view`, but returns a value which is not explicitly tied
    /// to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[inline]
    unsafe fn as_unscoped_file_view(&self) -> View<'static, File> {
        let unsafe_file = self.as_unsafe_file();
        let file = File::from_unsafe_file(unsafe_file);
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }

    /// Utility for returning a value which dereferences to a `&PipeReader` or
    /// `&mut PipeReader`.
    ///
    /// Note that `AsUnsafeFile` may be implemented for types which are not
    /// pipes, and which don't support all the methods on `PipeReader`.
    #[cfg(feature = "os_pipe")]
    #[inline]
    fn as_pipe_reader_view(&self) -> View<'_, PipeReader> {
        let unsafe_file = self.as_unsafe_file();
        let file = unsafe { PipeReader::from_unsafe_file(unsafe_file) };
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_file_view`, but returns a value which is not explicitly tied
    /// to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[cfg(feature = "os_pipe")]
    #[inline]
    unsafe fn as_unscoped_pipe_reader_view(&self) -> View<'static, PipeReader> {
        let unsafe_file = self.as_unsafe_file();
        let file = PipeReader::from_unsafe_file(unsafe_file);
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }

    /// Utility for returning a value which dereferences to a `&PipeWriter` or
    /// `&mut PipeWriter`.
    ///
    /// Note that `AsUnsafeFile` may be implemented for types which are not
    /// pipes, and which don't support all the methods on `PipeWriter`.
    #[cfg(feature = "os_pipe")]
    #[inline]
    fn as_pipe_writer_view(&self) -> View<'_, PipeWriter> {
        let unsafe_file = self.as_unsafe_file();
        let file = unsafe { PipeWriter::from_unsafe_file(unsafe_file) };
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_file_view`, but returns a value which is not explicitly tied
    /// to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[cfg(feature = "os_pipe")]
    #[inline]
    unsafe fn as_unscoped_pipe_writer_view(&self) -> View<'static, PipeWriter> {
        let unsafe_file = self.as_unsafe_file();
        let file = PipeWriter::from_unsafe_file(unsafe_file);
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }

    /// Test whether `self.as_unsafe_file().as_unsafe_handle()` is equal to
    /// `other.as_unsafe_file().as_unsafe_handle()`.
    ///
    /// That this depends on the guarantee that types that implement
    /// `AsUnsafeFile` own their resources, so we won't erroneously compare
    /// dangling handles.
    #[inline]
    fn eq_file<Filelike: AsUnsafeFile>(&self, other: &Filelike) -> bool {
        unsafe { self.as_unsafe_handle().eq(other.as_unsafe_handle()) }
    }
}

/// A trait for types which can be converted into unsafe files.
///
/// A type implementing `IntoUnsafeFile` guarantees that the return value
/// from `into_unsafe_file` on an instance of the type is a copy of a handle
/// which is owned by that instance.
///
/// # Safety
///
/// This trait is `unsafe` because types implementing it must guarantee they
/// own their handle.
pub unsafe trait IntoUnsafeFile: IntoUnsafeHandle {
    /// Convert `self` into an unsafe file.
    fn into_unsafe_file(self) -> UnsafeFile;
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

    /// Convert from a type which implements `IntoUnsafeFile` into a type that
    /// implements `FromUnsafeFile`.
    #[inline]
    fn from_filelike<Filelike: IntoUnsafeFile>(filelike: Filelike) -> Self
    where
        Self: Sized,
    {
        let unsafe_file = filelike.into_unsafe_file();
        unsafe { Self::from_unsafe_file(unsafe_file) }
    }
}

/// A trait for types which contain an unsafe socket and can expose it.
///
/// A type implementing `AsUnsafeSocket` guarantees that the return value
/// from `as_unsafe_socket` on an instance of the type is a copy of a socket
/// which is owned.
///
/// # Safety
///
/// This trait is `unsafe` because types implementing it must guarantee they
/// own their socket.
pub unsafe trait AsUnsafeSocket: AsUnsafeHandle {
    /// Return the contained unsafe socket.
    fn as_unsafe_socket(&self) -> UnsafeSocket;

    /// Utility for returning a value which dereferences to a `&TcpStream` or
    /// `&mut TcpStream`.
    ///
    /// Note that `AsUnsafeSocket` may be implemented for types which are not
    /// TCP streams, and which don't support all the methods on `TcpStream`.
    #[inline]
    fn as_tcp_stream_view(&self) -> View<'_, TcpStream> {
        let unsafe_socket = self.as_unsafe_socket();
        let tcp_stream = unsafe { TcpStream::from_unsafe_socket(unsafe_socket) };
        View {
            target: ManuallyDrop::new(tcp_stream),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_tcp_stream_view`, but returns a value which is not explicitly
    /// tied to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[inline]
    unsafe fn as_unscoped_tcp_stream_view(&self) -> View<'static, TcpStream> {
        let unsafe_socket = self.as_unsafe_socket();
        let tcp_stream = TcpStream::from_unsafe_socket(unsafe_socket);
        View {
            target: ManuallyDrop::new(tcp_stream),
            _phantom_data: PhantomData,
        }
    }

    /// Utility for returning a value which dereferences to a `&TcpListener` or
    /// `&mut TcpListener`.
    ///
    /// Note that `AsUnsafeSocket` may be implemented for types which are not
    /// TCP listeners, and which don't support all the methods on
    /// `TcpListener`.
    #[inline]
    fn as_tcp_listener_view(&self) -> View<'_, TcpListener> {
        let unsafe_socket = self.as_unsafe_socket();
        let tcp_listener = unsafe { TcpListener::from_unsafe_socket(unsafe_socket) };
        View {
            target: ManuallyDrop::new(tcp_listener),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_tcp_listener_view`, but returns a value which is not
    /// explicitly tied to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[inline]
    unsafe fn as_unscoped_tcp_listener_view(&self) -> View<'static, TcpListener> {
        let unsafe_socket = self.as_unsafe_socket();
        let tcp_listener = TcpListener::from_unsafe_socket(unsafe_socket);
        View {
            target: ManuallyDrop::new(tcp_listener),
            _phantom_data: PhantomData,
        }
    }

    /// Utility for returning a value which dereferences to a `&UdpSocket` or
    /// `&mut UdpSocket`.
    ///
    /// Note that `AsUnsafeSocket` may be implemented for types which are not
    /// UDP sockets, and which don't support all the methods on `UdpSocket`.
    #[inline]
    fn as_udp_socket_view(&self) -> View<'_, UdpSocket> {
        let unsafe_socket = self.as_unsafe_socket();
        let udp_socket = unsafe { UdpSocket::from_unsafe_socket(unsafe_socket) };
        View {
            target: ManuallyDrop::new(udp_socket),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_udp_socket_view`, but returns a value which is not explicitly
    /// tied to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[inline]
    unsafe fn as_unscoped_udp_socket_view(&self) -> View<'static, UdpSocket> {
        let unsafe_socket = self.as_unsafe_socket();
        let udp_socket = UdpSocket::from_unsafe_socket(unsafe_socket);
        View {
            target: ManuallyDrop::new(udp_socket),
            _phantom_data: PhantomData,
        }
    }

    /// Utility for returning a value which dereferences to a `&UnixStream` or
    /// `&mut UnixStream`.
    ///
    /// Note that `AsUnsafeSocket` may be implemented for types which are not
    /// Unix-domain socket streams, and which don't support all the methods on
    /// `UnixStream`.
    #[cfg(unix)]
    #[inline]
    fn as_unix_stream_view(&self) -> View<'_, UnixStream> {
        let unsafe_socket = self.as_unsafe_socket();
        let unix_stream = unsafe { UnixStream::from_unsafe_socket(unsafe_socket) };
        View {
            target: ManuallyDrop::new(unix_stream),
            _phantom_data: PhantomData,
        }
    }

    /// Like `as_unix_stream_view`, but returns a value which is not explicitly
    /// tied to the lifetime of `self`.
    ///
    /// # Safety
    ///
    /// Callers must manually ensure that the view doesn't outlive `self`.
    #[cfg(unix)]
    #[inline]
    unsafe fn as_unscoped_unix_stream_view(&self) -> View<'static, UnixStream> {
        let unsafe_socket = self.as_unsafe_socket();
        let unix_stream = UnixStream::from_unsafe_socket(unsafe_socket);
        View {
            target: ManuallyDrop::new(unix_stream),
            _phantom_data: PhantomData,
        }
    }

    /// Test whether `self.as_unsafe_socket().as_unsafe_handle()` is equal to
    /// `other.as_unsafe_socket().as_unsafe_handle()`.
    ///
    /// That this depends on the guarantee that types that implement
    /// `AsUnsafeSocket` own their resources, so we won't erroneously compare
    /// dangling handles.
    #[inline]
    fn eq_socket<Socketlike: AsUnsafeSocket>(&self, other: &Socketlike) -> bool {
        unsafe { self.as_unsafe_handle().eq(other.as_unsafe_handle()) }
    }
}

/// A trait for types which can be converted into unsafe sockets.
///
/// A type implementing `IntoUnsafeSocket` guarantees that the return value
/// from `into_unsafe_socket` on an instance of the type is a copy of a handle
/// which is owned by that instance.
///
/// # Safety
///
/// This trait is `unsafe` because types implementing it must guarantee they
/// own their socket.
pub unsafe trait IntoUnsafeSocket: IntoUnsafeHandle {
    /// Convert `self` into an unsafe socket.
    fn into_unsafe_socket(self) -> UnsafeSocket;
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

    /// Convert from a type which implements `IntoUnsafeSocket` into a type
    /// that implements `FromUnsafeSocket`.
    #[inline]
    fn from_socketlike<Socketlike: IntoUnsafeSocket>(socketlike: Socketlike) -> Self
    where
        Self: Sized,
    {
        let unsafe_socket = socketlike.into_unsafe_socket();
        unsafe { Self::from_unsafe_socket(unsafe_socket) }
    }
}

/// A non-owning unsafe I/O handle.
///
/// On Posix-ish platforms this is just a [`RawFd`]. On Windows it is a minimal
/// abstraction over [`RawHandle`] and [`RawSocket`]. Similar to Rust raw
/// pointers, it is considered safe to construct these, but unsafe to do any
/// I/O with them (effectively dereferencing them).
///
/// [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
/// [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
/// [`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeHandle(InnerFileOrSocket);

/// A non-owning unsafe I/O handle which on Windows is limited to handling what
/// Windows considers to be [`RawHandle`]s—mainly files and pipes.
///
/// [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeFile(InnerFile);

/// A non-owning unsafe I/O handle which on Windows is limited to handling what
/// Windows considers to be [`RawSocket`]s—mainly TCP streams and listeners
/// and UDP sockets.
///
/// [`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeSocket(InnerSocket);

/// A non-owning unsafe I/O handle that implements [`Read`]. `Read` functions
/// are considered safe, so this type requires `unsafe` to construct.
///
/// Like [`UnsafeHandle`], this doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this reads from the handle as if it were a
/// [`File`]. On Windows, this reads from a file-like handle as if it were a
/// [`File`], and from a socket-like handle as if it were a [`TcpStream`].
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeReadable(InnerFileOrSocket);

/// A non-owning unsafe I/O handle that implements [`Write`]. `Write` functions
/// considered are safe, so this type requires `unsafe` to construct.
///
/// Like [`UnsafeHandle`], this doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this writes to the handle as if it were a
/// [`File`]. On Windows, this writes to a file-like handle as if it were a
/// [`File`], and to a socket-like handle as if it were a [`TcpStream`].
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UnsafeWriteable(InnerFileOrSocket);

/// Posix-ish platforms use a single "file descriptor" type for all the kinds
/// of resources we're abstracting over, so we can just use that for
/// everything.
#[cfg(not(windows))]
type InnerFileOrSocket = RawFd;

/// See the comments for [`InnerFileOrSocket`].
#[cfg(not(windows))]
type InnerFile = RawFd;

/// See the comments for [`InnerFileOrSocket`].
#[cfg(not(windows))]
type InnerSocket = RawFd;

/// Windows uses separate types for file-like and socket-like resources, so we
/// use each type for its purpose.
#[cfg(windows)]
type InnerFileOrSocket = RawHandleOrSocket;

/// See the comments for [`InnerFileOrSocket`].
#[cfg(windows)]
type InnerFile = RawHandle;

/// See the comments for [`InnerFileOrSocket`].
#[cfg(windows)]
type InnerSocket = RawSocket;

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`.
pub struct View<'resource, Target: AsUnsafeHandle> {
    /// The value to dereference to. It's wrapped in `ManuallyDrop` because
    /// this is a non-owning view over the underlying resource.
    target: ManuallyDrop<Target>,

    /// This field exists because we don't otherwise explicitly use
    /// `'resource`.
    _phantom_data: PhantomData<&'resource ()>,
}

impl<Target: AsUnsafeHandle> Deref for View<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<Target: AsUnsafeHandle> DerefMut for View<'_, Target> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}

impl<Target: AsUnsafeHandle + fmt::Debug> fmt::Debug for View<'_, Target> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("View").field("target", &*self).finish()
    }
}

impl UnsafeHandle {
    /// Obtain `self` with a type that implements [`Read`].
    ///
    /// # Safety
    ///
    /// The resulting [`UnsafeReadable`] value must not outlive the underlying
    /// resource.
    ///
    /// Also, callers should avoid mixing uses of `UnsafeReadable` with the
    /// type's `Read` or `Write` implementations if the type uses buffering, as
    /// for example [`async_std::fs::File`] does.
    ///
    /// [`async_std::fs::File`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html
    #[inline]
    #[must_use]
    pub const unsafe fn as_readable(self) -> UnsafeReadable {
        UnsafeReadable(self.0)
    }

    /// Obtain `self` with a type that implements [`Write`].
    ///
    /// # Safety
    ///
    /// The resulting [`UnsafeWriteable`] value must not outlive the underlying
    /// resource.
    ///
    /// Also, callers should avoid mixing uses of `UnsafeWriteable` with the
    /// type's `Read` or `Write` implementation if the type uses buffering, as
    /// for example [`async_std::fs::File`] does.
    ///
    /// [`async_std::fs::File`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html
    #[inline]
    #[must_use]
    pub const unsafe fn as_writeable(self) -> UnsafeWriteable {
        UnsafeWriteable(self.0)
    }

    /// `UnsafeHandle` doesn't implement `Eq` or `PartialEq` because comparison
    /// is undefined for dangling handles.
    ///
    /// # Safety
    ///
    /// Both `self` and `other` must outlive their underlying resources.
    #[inline]
    #[must_use]
    pub unsafe fn eq(self, other: Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }

    /// Like [`FromRawFd::from_raw_fd`], but isn't unsafe because it doesn't
    /// imply a dereference.
    #[cfg(not(windows))]
    #[inline]
    #[must_use]
    pub const fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }

    /// Like [`FromRawHandle::from_raw_handle`], but isn't unsafe because it
    /// doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    pub const fn from_raw_handle(raw_handle: RawHandle) -> Self {
        Self(RawHandleOrSocket::from_raw_handle(raw_handle))
    }

    /// Like [`FromRawSocket::from_raw_socket`], but isn't unsafe because it
    /// doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    #[must_use]
    pub const fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(RawHandleOrSocket::from_raw_socket(raw_socket))
    }

    /// Like [`FromRawHandle::from_raw_handle`] and
    /// [`FromRawSocket::from_raw_socket`] combined, but isn't unsafe because
    /// it doesn't imply a dereference.
    #[cfg(windows)]
    #[inline]
    #[must_use]
    pub const fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

#[cfg(not(windows))]
impl UnsafeFile {
    /// Like [`FromRawFd::from_raw_fd`], but isn't unsafe because it doesn't
    /// imply a dereference.
    #[inline]
    #[must_use]
    pub const fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

#[cfg(windows)]
impl UnsafeFile {
    /// Like [`FromRawHandle::from_raw_handle`], but isn't unsafe because it
    /// doesn't imply a dereference.
    #[inline]
    pub const fn from_raw_handle(raw_handle: RawHandle) -> Self {
        Self(raw_handle)
    }
}

#[cfg(not(windows))]
impl UnsafeSocket {
    /// Like [`FromRawFd::from_raw_fd`], but isn't unsafe because it doesn't
    /// imply a dereference.
    #[inline]
    #[must_use]
    pub const fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

#[cfg(windows)]
impl UnsafeSocket {
    /// Like [`FromRawSocket::from_raw_socket`], but isn't unsafe because it
    /// doesn't imply a dereference.
    #[inline]
    #[must_use]
    pub const fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(raw_socket)
    }
}

/// `UnsafeHandle` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
impl FromUnsafeHandle for UnsafeHandle {
    #[inline]
    unsafe fn from_unsafe_handle(unsafe_handle: UnsafeHandle) -> Self {
        unsafe_handle
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
impl FromUnsafeHandle for UnsafeReadable {
    #[inline]
    unsafe fn from_unsafe_handle(unsafe_handle: UnsafeHandle) -> Self {
        Self(unsafe_handle.0)
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
impl FromUnsafeHandle for UnsafeWriteable {
    #[inline]
    unsafe fn from_unsafe_handle(unsafe_handle: UnsafeHandle) -> Self {
        Self(unsafe_handle.0)
    }
}

// Posix-ish implementations.

// Safety: By requiring `T: OwnsRaw`, we can assume the `AsRawFd` owns its fd.
#[cfg(not(windows))]
unsafe impl<T: AsRawFd + OwnsRaw> AsUnsafeHandle for T {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        UnsafeHandle(self.as_raw_fd())
    }
}

#[cfg(not(windows))]
unsafe impl<T: AsRawFd + OwnsRaw> AsUnsafeFile for T {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(self.as_raw_fd())
    }
}

#[cfg(not(windows))]
unsafe impl<T: AsRawFd + OwnsRaw> AsUnsafeSocket for T {
    #[inline]
    fn as_unsafe_socket(&self) -> UnsafeSocket {
        UnsafeSocket(self.as_raw_fd())
    }
}

#[cfg(not(windows))]
unsafe impl<T: IntoRawFd + OwnsRaw> IntoUnsafeHandle for T {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle(self.into_raw_fd())
    }
}

#[cfg(not(windows))]
unsafe impl<T: IntoRawFd + OwnsRaw> IntoUnsafeFile for T {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(self.into_raw_fd())
    }
}

#[cfg(not(windows))]
unsafe impl<T: IntoRawFd + OwnsRaw> IntoUnsafeSocket for T {
    #[inline]
    fn into_unsafe_socket(self) -> UnsafeSocket {
        UnsafeSocket(self.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl<T: FromRawFd + OwnsRaw> FromUnsafeFile for T {
    #[inline]
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self {
        Self::from_raw_fd(unsafe_file.0)
    }
}

#[cfg(not(windows))]
impl<T: FromRawFd + OwnsRaw> FromUnsafeSocket for T {
    #[inline]
    unsafe fn from_unsafe_socket(unsafe_socket: UnsafeSocket) -> Self {
        Self::from_raw_fd(unsafe_socket.0)
    }
}

/// `UnsafeHandle` doesn't own its handle, but `AsRawFd` doesn't require any
/// guarantees about lifetimes, so it's safe to implement. This is similar to
/// how `RawFd` implements `AsRawFd` (see the `raw_fd_reflexive_traits` trait
/// implementations in `std`).
#[cfg(not(windows))]
impl AsRawFd for UnsafeHandle {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeHandle` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeHandle {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeHandle` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl FromRawFd for UnsafeHandle {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl AsRawFd for UnsafeReadable {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeReadable {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl FromRawFd for UnsafeReadable {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

#[cfg(not(windows))]
impl UnsafeReadable {
    /// Like `AsUnsafeFile::as_file_view`, but `unsafe` because
    /// `UnsafeReadable` doesn't own its file descriptor.
    ///
    /// # Safety
    ///
    /// The contained file descriptor must be valid.
    #[inline]
    unsafe fn as_file_view(&self) -> View<'_, File> {
        let raw_fd = self.as_raw_fd();
        let file = File::from_raw_fd(raw_fd);
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl AsRawFd for UnsafeWriteable {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeWriteable {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl FromRawFd for UnsafeWriteable {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

#[cfg(not(windows))]
impl UnsafeWriteable {
    /// Like `AsUnsafeFile::as_file_view`, but `unsafe` because
    /// `UnsafeWriteable` doesn't own its file descriptor.
    ///
    /// # Safety
    ///
    /// The contained file descriptor must be valid.
    #[inline]
    unsafe fn as_file_view(&self) -> View<'_, File> {
        let raw_fd = self.as_raw_fd();
        let file = File::from_raw_fd(raw_fd);
        View {
            target: ManuallyDrop::new(file),
            _phantom_data: PhantomData,
        }
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl AsRawFd for UnsafeFile {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeFile {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl FromRawFd for UnsafeFile {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl AsRawFd for UnsafeSocket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl IntoRawFd for UnsafeSocket {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(not(windows))]
impl FromRawFd for UnsafeSocket {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(raw_fd)
    }
}

// Windows implementations.

// Safety: By requiring `T: OwnsRaw`, we can assume the `AsRawHandleOrSocket`
// owns its handle or socket.
#[cfg(windows)]
unsafe impl<T: AsRawHandleOrSocket + OwnsRaw> AsUnsafeHandle for T {
    #[inline]
    fn as_unsafe_handle(&self) -> UnsafeHandle {
        UnsafeHandle(self.as_raw_handle_or_socket())
    }
}

#[cfg(windows)]
unsafe impl<T: AsRawHandle + AsUnsafeHandle + OwnsRaw> AsUnsafeFile for T {
    #[inline]
    fn as_unsafe_file(&self) -> UnsafeFile {
        UnsafeFile(AsRawHandle::as_raw_handle(self))
    }
}

#[cfg(windows)]
unsafe impl<T: AsRawSocket + AsUnsafeHandle + OwnsRaw> AsUnsafeSocket for T {
    #[inline]
    fn as_unsafe_socket(&self) -> UnsafeSocket {
        UnsafeSocket(AsRawSocket::as_raw_socket(self))
    }
}

#[cfg(windows)]
unsafe impl<T: IntoRawHandle + IntoUnsafeHandle + OwnsRaw> IntoUnsafeFile for T {
    #[inline]
    fn into_unsafe_file(self) -> UnsafeFile {
        UnsafeFile(Self::into_raw_handle(self))
    }
}

#[cfg(windows)]
unsafe impl<T: IntoRawSocket + IntoUnsafeSocket + OwnsRaw> IntoUnsafeSocket for T {
    #[inline]
    fn into_unsafe_socket(self) -> UnsafeSocket {
        UnsafeSocket(IntoRawSocket::into_raw_socket(self))
    }
}

#[cfg(windows)]
impl<T: FromRawHandle + OwnsRaw> FromUnsafeFile for T {
    #[inline]
    unsafe fn from_unsafe_file(unsafe_file: UnsafeFile) -> Self {
        Self::from_raw_handle(unsafe_file.0)
    }
}

#[cfg(windows)]
impl<T: FromRawSocket + OwnsRaw> FromUnsafeSocket for T {
    #[inline]
    unsafe fn from_unsafe_socket(unsafe_socket: UnsafeSocket) -> Self {
        Self::from_raw_socket(unsafe_socket.0)
    }
}

// Safety: `File` owns its handle.
#[cfg(windows)]
unsafe impl IntoUnsafeHandle for File {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

// Safety: `ChildStdin` owns its handle.
#[cfg(windows)]
unsafe impl IntoUnsafeHandle for ChildStdin {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

// Safety: `ChildStdout` owns its handle.
#[cfg(windows)]
unsafe impl IntoUnsafeHandle for ChildStdout {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

// Safety: `ChildStderr` owns its handle.
#[cfg(windows)]
unsafe impl IntoUnsafeHandle for ChildStderr {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

// Safety: `TcpStream` owns its handle.
#[cfg(windows)]
unsafe impl IntoUnsafeHandle for TcpStream {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_socket(Self::into_raw_socket(self))
    }
}

// Safety: `PipeReader` owns its handle.
#[cfg(all(windows, feature = "os_pipe"))]
unsafe impl IntoUnsafeHandle for PipeReader {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

// Safety: `PipeWriter` owns its handle.
#[cfg(all(windows, feature = "os_pipe"))]
unsafe impl IntoUnsafeHandle for PipeWriter {
    #[inline]
    fn into_unsafe_handle(self) -> UnsafeHandle {
        UnsafeHandle::from_raw_handle(Self::into_raw_handle(self))
    }
}

/// `UnsafeHandle` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeHandle {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeHandle` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeHandle {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeHandle` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl FromRawHandleOrSocket for UnsafeHandle {
    #[inline]
    unsafe fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeReadable {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeReadable {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeReadable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl FromRawHandleOrSocket for UnsafeReadable {
    #[inline]
    unsafe fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.0
    }
}

/// `UnsafeWriteable` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl FromRawHandleOrSocket for UnsafeWriteable {
    #[inline]
    unsafe fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self {
        Self(raw_handle_or_socket)
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeFile {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(self.0)
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeFile {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(self.0)
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawHandle for UnsafeFile {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.0
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawHandle for UnsafeFile {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.0
    }
}

/// `UnsafeFile` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl FromRawHandle for UnsafeFile {
    #[inline]
    unsafe fn from_raw_handle(raw_handle: RawHandle) -> Self {
        Self(raw_handle)
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawHandleOrSocket for UnsafeSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(self.0)
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawHandleOrSocket for UnsafeSocket {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(self.0)
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl AsRawSocket for UnsafeSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.0
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl IntoRawSocket for UnsafeSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.0
    }
}

/// `UnsafeSocket` doesn't own its handle; see the comments for
/// `impl AsRawFd for UnsafeHandle`.
#[cfg(windows)]
impl FromRawSocket for UnsafeSocket {
    #[inline]
    unsafe fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(raw_socket)
    }
}

#[cfg(not(windows))]
impl Read for UnsafeReadable {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Safety: The caller of `as_readable()`, which is unsafe, is expected
        // to ensure that the underlying resource outlives this
        // `UnsafeReadable`.
        unsafe { self.as_file_view() }.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsafe { self.as_file_view() }.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        unsafe { self.as_file_view() }.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unsafe { self.as_file_view() }.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        unsafe { self.as_file_view() }.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        unsafe { self.as_file_view() }.read_exact(buf)
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
        unsafe { self.as_file_view() }.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        unsafe { self.as_file_view() }.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsafe { self.as_file_view() }.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        unsafe { self.as_file_view() }.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        unsafe { self.as_file_view() }.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        unsafe { self.as_file_view() }.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        unsafe { self.as_file_view() }.write_fmt(fmt)
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
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.flush(),
            RawEnum::Socket(raw_socket) => unsafe { as_tcp_stream_view(self, raw_socket) }.flush(),
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
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.write_all(buf),
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write_all(buf)
            }
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
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        match self.0 .0 {
            RawEnum::Handle(raw_handle) => unsafe { as_file_view(self, raw_handle) }.write_fmt(fmt),
            RawEnum::Socket(raw_socket) => {
                unsafe { as_tcp_stream_view(self, raw_socket) }.write_fmt(fmt)
            }
        }
    }
}

/// Like `AsUnsafeFile::as_file_view`, but with the method for obtaining a
/// [`RawHandle`] from an [`UnsafeReadable`] or [`UnsafeWriteable`] factored
/// out.
///
/// # Safety
///
/// `raw_handle` must be valid.
#[cfg(windows)]
#[inline]
unsafe fn as_file_view<T>(_t: &T, raw_handle: RawHandle) -> View<'_, File> {
    View {
        target: ManuallyDrop::new(File::from_raw_handle(raw_handle)),
        _phantom_data: PhantomData,
    }
}

/// Like `AsUnsafeSocket::as_tcp_stream_view`, but with the method for
/// obtaining a [`RawSocket`] from an [`UnsafeReadable`] or [`UnsafeWriteable`]
/// factored out.
///
/// # Safety
///
/// `raw_socket` must be valid.
#[cfg(windows)]
#[inline]
unsafe fn as_tcp_stream_view<T>(_t: &T, raw_socket: RawSocket) -> View<'_, TcpStream> {
    View {
        target: ManuallyDrop::new(TcpStream::from_raw_socket(raw_socket)),
        _phantom_data: PhantomData,
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeHandle {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the fd number; don't try to print the path or any
        // information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        f.debug_struct("UnsafeHandle")
            .field("raw_fd", &self.as_raw_fd())
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeHandle {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket; don't try to print the path or
        // any information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        f.debug_struct("UnsafeHandle")
            .field("raw_handle_or_socket", &self.as_raw_handle_or_socket())
            .finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeFile {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw fd number.
        f.debug_struct("UnsafeFile")
            .field("raw_fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeFile {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw handle.
        f.debug_struct("UnsafeFile")
            .field("raw_handle", &self.0)
            .finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeSocket {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw fd number.
        f.debug_struct("UnsafeSocket")
            .field("raw_fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeSocket {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw socket.
        f.debug_struct("UnsafeSocket")
            .field("raw_socket", &self.0)
            .finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeReadable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw fd number.
        f.debug_struct("UnsafeReadable")
            .field("raw_fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeReadable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw handle or socket.
        f.debug_struct("UnsafeReadable")
            .field("raw_handle_or_socket", &self.0)
            .finish()
    }
}

#[cfg(not(windows))]
impl fmt::Debug for UnsafeWriteable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw fd number.
        f.debug_struct("UnsafeWriteable")
            .field("raw_fd", &self.0)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for UnsafeWriteable {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // As with `UnsafeHandle`, just print the raw handle or socket.
        f.debug_struct("UnsafeWriteable")
            .field("raw_handle_or_socket", &self.0)
            .finish()
    }
}
