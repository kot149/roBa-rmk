#![no_main]
#![no_std]

use rmk::macros::rmk_peripheral;
use roba_rmk::{SplitConnectionLed};
use embassy_nrf::gpio::{Output, Level, OutputDrive};

#[rmk_peripheral(id = 0)]
mod keyboard_peripheral {
    #[controller(event)]
    fn split_connection_led() -> SplitConnectionLed {
        let led_blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);
        let led_red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
        SplitConnectionLed::new(led_blue, led_red)
    }
}
