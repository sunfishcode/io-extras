//! This example isn't faster, smaller, simpler, more efficient, more portable,
//! or more desireable than regular hello world in any practical way. It just
//! demonstrates the API of this crate.

use std::io::{stderr, stdout};
use unsafe_io::{AsUnsafeFile, AsUnsafeHandle};

#[test]
fn eq() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let stderr = stderr();
    let stderr = stderr.lock();

    // Trivially assert that stdout and stderr has the same handles as
    // themselves and different handles from each other.
    assert!(stdout.eq_handle(&stdout));
    assert!(stderr.eq_handle(&stderr));
    assert!(!stdout.eq_handle(&stderr));
    assert!(!stderr.eq_handle(&stdout));

    // The same is true of file-like views of their handles.
    assert!(stdout.eq_file(&stdout));
    assert!(stderr.eq_file(&stderr));
    assert!(!stdout.eq_file(&stderr));
    assert!(!stderr.eq_file(&stdout));
}
