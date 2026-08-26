#![no_main]
#![no_std]

use embassy_nrf::gpio::{Level, Output, OutputDrive};
use rmk::macros::rmk_peripheral;
use mona2_rmk::SplitConnectionLed;

#[rmk_peripheral(id = 0)]
mod keyboard_peripheral {
    #[register_processor(poll)]
    fn split_connection_led() -> SplitConnectionLed {
        SplitConnectionLed::new(
            Output::new(p.P0_26, Level::High, OutputDrive::Standard),
            Output::new(p.P0_30, Level::High, OutputDrive::Standard),
            Output::new(p.P0_06, Level::High, OutputDrive::Standard),
        )
    }
}
