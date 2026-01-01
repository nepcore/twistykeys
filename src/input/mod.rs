#[cfg(unix)]
mod uinput;
#[cfg(unix)]
pub use uinput::*;

#[cfg(not(unix))]
mod einput;
#[cfg(not(unix))]
pub use einput::*;
