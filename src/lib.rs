#![no_std]

pub mod led;

pub use led::{BleConnectionLed, SplitConnectionLed};

use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::peripherals::P0_14;

/// XIAO BLE nRF52840: Enable battery voltage reading by setting P0.14 (VBAT_ENABLE) to LOW.
/// This pin controls the voltage divider circuit for battery measurement on P0.31.
pub fn xiao_ble_enable_vbat_reading() {
    let pin = unsafe { P0_14::steal() };
    let output = Output::new(pin, Level::Low, OutputDrive::Standard);
    core::mem::forget(output);
}
