//! This example isn't faster, smaller, simpler, more efficient, more portable,
//! or more desireable than regular hello world in any practical way. It just
//! demonstrates the API of this crate.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![allow(unstable_name_collisions)]

use io_extras::grip::{AsGrip, AsRawGrip};
use io_extras::read_write::{ReadHalf, WriteHalf};
#[cfg(not(windows))]
use {
    io_extras::os::rustix::{AsRawFd, AsReadWriteFd},
    io_lifetimes::BorrowedFd,
};
#[cfg(windows)]
use {
    io_extras::os::windows::AsRawHandleOrSocket,
    io_extras::os::windows::{AsReadWriteHandleOrSocket, BorrowedHandleOrSocket},
};

struct Stdio {}

#[cfg(not(windows))]
impl AsReadWriteFd for Stdio {
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(std::io::stdin().as_raw_fd()) }
    }

    fn as_write_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(std::io::stdout().as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsReadWriteHandleOrSocket for Stdio {
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        unsafe {
            BorrowedHandleOrSocket::borrow_raw_handle_or_socket(
                std::io::stdin().as_raw_handle_or_socket(),
            )
        }
    }

    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        unsafe {
            BorrowedHandleOrSocket::borrow_raw_handle_or_socket(
                std::io::stdout().as_raw_handle_or_socket(),
            )
        }
    }
}

#[test]
fn read_write() {
    let stdio = Stdio {};
    assert!(
        ReadHalf::new(&stdio).as_grip().as_raw_grip() == std::io::stdin().as_grip().as_raw_grip()
    );
    assert!(
        WriteHalf::new(&stdio).as_grip().as_raw_grip() == std::io::stdout().as_grip().as_raw_grip()
    );
}

struct Stream {}
impl Stream {
    fn use_socket<Socketlike: io_lifetimes::AsSocketlike>(_socketlike: &mut Socketlike) {}

    fn use_file<Filelike: io_lifetimes::AsFilelike>(_filelike: &mut Filelike) {}

    fn use_grip<Grip: io_extras::grip::AsGrip>(grip: &mut Grip) {
        #[cfg(windows)]
        assert_ne!(
            grip.as_handle_or_socket().as_handle().is_some(),
            grip.as_handle_or_socket().as_socket().is_some()
        );
        #[cfg(not(windows))]
        let _ = grip.as_fd();
    }

    fn from_socket<Socketlike: io_lifetimes::IntoSocketlike>(_socketlike: Socketlike) {}

    fn from_file<Filelike: io_lifetimes::IntoFilelike>(_filelike: Filelike) {}

    fn from_grip<Grip: io_extras::grip::IntoGrip>(_grip: Grip) {}
}

#[test]
fn likes() {
    let _ = Stream::use_socket(&mut std::net::TcpListener::bind("127.0.0.1:0").unwrap());
    let _ = Stream::use_file(&mut std::fs::File::open("Cargo.toml").unwrap());
    let _ = Stream::use_grip(&mut std::net::TcpListener::bind("127.0.0.1:0").unwrap());
    let _ = Stream::use_grip(&mut std::fs::File::open("Cargo.toml").unwrap());

    let _ = Stream::from_socket(std::net::TcpListener::bind("127.0.0.1:0").unwrap());
    let _ = Stream::from_file(std::fs::File::open("Cargo.toml").unwrap());
    let _ = Stream::from_grip(std::net::TcpListener::bind("127.0.0.1:0").unwrap());
    let _ = Stream::from_grip(std::fs::File::open("Cargo.toml").unwrap());
}
