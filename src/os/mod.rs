//! OS-specific functionality.

#[cfg(not(windows))]
pub mod posish;
#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub mod windows_stdio;
