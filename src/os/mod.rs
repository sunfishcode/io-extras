//! OS-specific functionality.

#[cfg(not(windows))]
pub mod posish;
#[cfg(windows)]
pub mod windows;
