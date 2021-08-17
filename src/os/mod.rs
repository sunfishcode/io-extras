//! OS-specific functionality.

#[cfg(not(windows))]
pub mod rsix;
#[cfg(windows)]
pub mod windows;
