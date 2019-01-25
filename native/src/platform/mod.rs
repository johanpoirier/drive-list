pub mod common;
pub use self::common::*;

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use self::windows::PlatformImpl;

#[cfg(any(target_os = "linux"))]
pub mod linux;
#[cfg(any(target_os = "linux"))]
pub use self::linux::PlatformImpl;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::PlatformImpl;
