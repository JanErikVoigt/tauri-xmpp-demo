#[cfg(target_os = "android")]
mod android_keyring;
#[cfg(target_os = "android")]
pub use android_keyring::*;

#[cfg(not(target_os = "android"))]
mod keyring;
#[cfg(not(target_os = "android"))]
pub use keyring::*;
