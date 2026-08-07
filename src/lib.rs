#![cfg_attr(not(test), no_std)]

#[cfg(all(feature = "firmware", target_arch = "arm", target_os = "none"))]
mod led;
mod led_state;

#[cfg(all(feature = "firmware", target_arch = "arm", target_os = "none"))]
pub use led::{BleConnectionLed, SplitConnectionLed};
