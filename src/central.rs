#![no_main]
#![no_std]

use rmk::macros::rmk_central;
use roBa_rmk::{xiao_ble_enable_vbat_reading, BleConnectionLed};
use embassy_nrf::gpio::{Output, Level, OutputDrive};

#[rmk_central]
mod keyboard_central {
    #[controller(event)]
    fn ble_connection_led() -> BleConnectionLed {
        xiao_ble_enable_vbat_reading();

        let led_blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);
        let led_red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
        BleConnectionLed::new(led_blue, led_red)
    }
}
