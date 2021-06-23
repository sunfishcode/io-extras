//! "Grip" is an abstraction over "Fd" and "HandleOrSocket". "Handle"
//! would be the obvious term, but that has a more specific meaning on
//! Windows.

#[cfg(windows)]
use crate::os::windows::{
    AsHandleOrSocket, AsRawHandleOrSocket, AsRawReadWriteHandleOrSocket, AsReadWriteHandleOrSocket,
    BorrowedHandleOrSocket, FromHandleOrSocket, FromRawHandleOrSocket, IntoHandleOrSocket,
    IntoRawHandleOrSocket, OwnedHandleOrSocket, RawHandleOrSocket,
};
#[cfg(not(windows))]
use {
    crate::os::posish::{AsRawFd, AsRawReadWriteFd, AsReadWriteFd, FromRawFd, IntoRawFd, RawFd},
    io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd},
};

/// Portability abstraction over `BorrowedFd` and `BorrowedHandleOrSocket`.
#[cfg(not(windows))]
pub type BorrowedGrip<'a> = BorrowedFd<'a>;
/// Portability abstraction over `OwnedFd` and `OwnedHandleOrSocket`.
#[cfg(not(windows))]
pub type OwnedGrip = OwnedFd;

/// Portability abstraction over `BorrowedFd` and `BorrowedHandleOrSocket`.
#[cfg(windows)]
pub type BorrowedGrip<'a> = BorrowedHandleOrSocket<'a>;
/// Portability abstraction over `OwnedFd` and `OwnedHandleOrSocket`.
#[cfg(windows)]
pub type OwnedGrip = OwnedHandleOrSocket;

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsGrip<'a>: AsFd<'a> {
    /// Extracts the grip.
    fn as_grip(self) -> BorrowedGrip<'a>;
}

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(windows)]
pub trait AsGrip<'a>: AsHandleOrSocket<'a> {
    /// Extracts the grip.
    fn as_grip(self) -> BorrowedGrip<'a>;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsReadWriteGrip<'a>: AsReadWriteFd<'a> {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// reading grip.
    fn as_read_grip(self) -> BorrowedGrip<'a>;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// writing grip.
    fn as_write_grip(self) -> BorrowedGrip<'a>;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(windows)]
pub trait AsReadWriteGrip<'a>: AsReadWriteHandleOrSocket<'a> {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// reading grip.
    fn as_read_grip(self) -> BorrowedGrip<'a>;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// writing grip.
    fn as_write_grip(self) -> BorrowedGrip<'a>;
}

/// Portability abstraction over `IntoFd` and
/// `IntoHandleOrSocket`.
#[cfg(not(windows))]
pub trait IntoGrip: IntoFd {
    /// Consume `self` and convert into an `OwnedGrip`.
    fn into_grip(self) -> OwnedGrip;
}

/// Portability abstraction over `IntoFd` and
/// `IntoHandleOrSocket`.
#[cfg(windows)]
pub trait IntoGrip: IntoHandleOrSocket {
    /// Consume `self` and convert into an `OwnedGrip`.
    fn into_grip(self) -> OwnedGrip;
}

/// Portability abstraction over `FromFd` and
/// `FromHandleOrSocket`.
#[cfg(not(windows))]
pub trait FromGrip: FromFd {
    /// Consume an `OwnedGrip` and convert into a `Self`.
    fn from_grip(owned_grip: OwnedGrip) -> Self;
}

/// Portability abstraction over `FromFd` and
/// `FromHandleOrSocket`.
#[cfg(windows)]
pub trait FromGrip: FromHandleOrSocket {
    /// Consume an `OwnedGrip` and convert into a `Self`.
    fn from_grip(owned_grip: OwnedGrip) -> Self;
}

#[cfg(not(windows))]
impl<'a, T: AsFd<'a>> AsGrip<'a> for T {
    #[inline]
    fn as_grip(self) -> BorrowedGrip<'a> {
        self.as_fd()
    }
}

#[cfg(windows)]
impl<'a, T: AsHandleOrSocket<'a>> AsGrip<'a> for T {
    #[inline]
    fn as_grip(self) -> BorrowedGrip<'a> {
        self.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<'a, T: AsReadWriteFd<'a>> AsReadWriteGrip<'a> for T {
    #[inline]
    fn as_read_grip(self) -> BorrowedGrip<'a> {
        self.as_read_fd()
    }

    #[inline]
    fn as_write_grip(self) -> BorrowedGrip<'a> {
        self.as_write_fd()
    }
}

#[cfg(windows)]
impl<'a, T: AsReadWriteHandleOrSocket<'a>> AsReadWriteGrip<'a> for T {
    #[inline]
    fn as_read_grip(self) -> BorrowedGrip<'a> {
        self.as_read_handle_or_socket()
    }

    #[inline]
    fn as_write_grip(self) -> BorrowedGrip<'a> {
        self.as_write_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: IntoFd> IntoGrip for T {
    #[inline]
    fn into_grip(self) -> OwnedGrip {
        self.into_fd()
    }
}

#[cfg(windows)]
impl<T: IntoHandleOrSocket> IntoGrip for T {
    #[inline]
    fn into_grip(self) -> OwnedGrip {
        self.into_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: FromFd> FromGrip for T {
    #[inline]
    fn from_grip(owned_grip: OwnedGrip) -> Self {
        Self::from_fd(owned_grip)
    }
}

#[cfg(windows)]
impl<T: FromHandleOrSocket> FromGrip for T {
    #[inline]
    fn from_grip(owned_grip: OwnedGrip) -> Self {
        Self::from_handle_or_socket(owned_grip)
    }
}

/// Portability abstraction over `RawFd` and `RawHandleOrSocket`.
#[cfg(not(windows))]
pub type RawGrip = RawFd;

/// Portability abstraction over `RawFd` and `RawHandleOrSocket`.
#[cfg(windows)]
pub type RawGrip = RawHandleOrSocket;

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsRawGrip: AsRawFd {
    /// Extracts the raw grip.
    fn as_raw_grip(self) -> RawGrip;
}

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(windows)]
pub trait AsRawGrip: AsRawHandleOrSocket {
    /// Extracts the raw grip.
    fn as_raw_grip(self) -> RawGrip;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsRawReadWriteGrip: AsRawReadWriteFd {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw reading grip.
    fn as_raw_read_grip(self) -> RawGrip;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw writing grip.
    fn as_raw_write_grip(self) -> RawGrip;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(windows)]
pub trait AsRawReadWriteGrip: AsRawReadWriteHandleOrSocket {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw reading grip.
    fn as_raw_read_grip(self) -> RawGrip;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw writing grip.
    fn as_raw_write_grip(self) -> RawGrip;
}

/// Portability abstraction over `IntoFd` and
/// `IntoHandleOrSocket`.
#[cfg(not(windows))]
pub trait IntoRawGrip: IntoRawFd {
    /// Consume `self` and convert into an `RawGrip`.
    fn into_raw_grip(self) -> RawGrip;
}

/// Portability abstraction over `IntoFd` and
/// `IntoHandleOrSocket`.
#[cfg(windows)]
pub trait IntoRawGrip: IntoRawHandleOrSocket {
    /// Consume `self` and convert into an `RawGrip`.
    fn into_raw_grip(self) -> RawGrip;
}

/// Portability abstraction over `FromFd` and
/// `FromHandleOrSocket`.
#[cfg(not(windows))]
pub trait FromRawGrip: FromRawFd {
    /// Consume an `RawGrip` and convert into a `Self`.
    unsafe fn from_raw_grip(owned_grip: RawGrip) -> Self;
}

/// Portability abstraction over `FromFd` and
/// `FromHandleOrSocket`.
#[cfg(windows)]
pub trait FromRawGrip: FromRawHandleOrSocket {
    /// Consume an `RawGrip` and convert into a `Self`.
    unsafe fn from_raw_grip(owned_grip: RawGrip) -> Self;
}

#[cfg(not(windows))]
impl<T: AsRawFd> AsRawGrip for T {
    #[inline]
    fn as_raw_grip(self) -> RawGrip {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawHandleOrSocket> AsRawGrip for T {
    #[inline]
    fn as_raw_grip(self) -> RawGrip {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: AsRawReadWriteFd> AsRawReadWriteGrip for T {
    #[inline]
    fn as_raw_read_grip(self) -> RawGrip {
        self.as_raw_read_fd()
    }

    #[inline]
    fn as_raw_write_grip(self) -> RawGrip {
        self.as_raw_write_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawReadWriteHandleOrSocket> AsRawReadWriteGrip for T {
    #[inline]
    fn as_raw_read_grip(self) -> RawGrip {
        self.as_raw_read_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_grip(self) -> RawGrip {
        self.as_raw_write_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: IntoRawFd> IntoRawGrip for T {
    #[inline]
    fn into_raw_grip(self) -> RawGrip {
        self.into_raw_fd()
    }
}

#[cfg(windows)]
impl<T: IntoRawHandleOrSocket> IntoRawGrip for T {
    #[inline]
    fn into_raw_grip(self) -> RawGrip {
        self.into_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: FromRawFd> FromRawGrip for T {
    #[inline]
    unsafe fn from_raw_grip(owned_grip: RawGrip) -> Self {
        Self::from_raw_fd(owned_grip)
    }
}

#[cfg(windows)]
impl<T: FromRawHandleOrSocket> FromRawGrip for T {
    #[inline]
    unsafe fn from_raw_grip(owned_grip: RawGrip) -> Self {
        Self::from_raw_handle_or_socket(owned_grip)
    }
}
