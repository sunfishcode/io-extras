//! "Grip" is an abstraction over "Fd" and "HandleOrSocket". "Handle"
//! would be the obvious term, but that has a more specific meaning on
//! Windows.

#[cfg(windows)]
use crate::os::windows::{
    AsHandleOrSocket, AsReadWriteHandleOrSocket, BorrowedHandleOrSocket, FromHandleOrSocket,
    IntoHandleOrSocket, OwnedHandleOrSocket,
};
#[cfg(not(windows))]
use {
    crate::os::posish::AsReadWriteFd,
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
