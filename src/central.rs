#![no_main]
#![no_std]

mod pointingproccontroller;

use embassy_nrf::gpio::{Level, Output, OutputDrive};
use rmk::macros::rmk_central;
use roba_rmk::BleConnectionLed;

#[rmk_central]
mod keyboard_central {
    #[register_processor(poll)]
    fn ble_connection_led() -> BleConnectionLed<impl Fn() -> bool> {
        BleConnectionLed::new(
            Output::new(p.P0_26, Level::High, OutputDrive::Standard),
            Output::new(p.P0_30, Level::High, OutputDrive::Standard),
            Output::new(p.P0_06, Level::High, OutputDrive::Standard),
            || stack.with_bond_information(|bonds| !bonds.is_empty()),
        )
    }

    #[register_processor(event)]
    fn pointing_processor_controller() -> crate::pointingproccontroller::PointingProcessorController
    {
        crate::pointingproccontroller::PointingProcessorController::new()
    }
}
