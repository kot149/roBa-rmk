#![allow(dead_code)]

use defmt::{info, unwrap};
use embassy_nrf::gpio::{Level, Output};
use embassy_time::Timer;
use rmk::ble::BleState;
use rmk::channel::{ControllerSub, CONTROLLER_CHANNEL};
use rmk::controller::Controller;
use rmk::event::ControllerEvent;

pub struct BleConnectionLed {
    led_blue: Output<'static>,
    led_red: Output<'static>,
    sub: ControllerSub,
    connected: bool,
}

impl BleConnectionLed {
    pub fn new(led_blue: Output<'static>, led_red: Output<'static>) -> Self {
        Self {
            led_blue,
            led_red,
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            connected: false,
        }
    }
}

impl Controller for BleConnectionLed {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::BleState(_profile, state) => match state {
                BleState::Connected => {
                    if !self.connected {
                        self.connected = true;
                        self.led_blue.set_level(Level::Low);
                        info!("BLE connected, Blue LED ON");
                        Timer::after_millis(500).await;
                        self.led_blue.set_level(Level::High);
                        info!("Blue LED OFF after 500ms");
                    }
                }
                BleState::None | BleState::Advertising => {
                    if self.connected {
                        self.connected = false;
                        self.led_red.set_level(Level::Low);
                        info!("BLE disconnected, Red LED ON");
                        Timer::after_millis(500).await;
                        self.led_red.set_level(Level::High);
                        info!("Red LED OFF after 500ms");
                    }
                }
            },
            _ => {}
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

pub struct SplitConnectionLed {
    led_blue: Output<'static>,
    led_red: Output<'static>,
    sub: ControllerSub,
}

impl SplitConnectionLed {
    pub fn new(led_blue: Output<'static>, led_red: Output<'static>) -> Self {
        Self {
            led_blue,
            led_red,
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
        }
    }
}

impl Controller for SplitConnectionLed {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::SplitCentral(connected) => {
                if connected {
                    self.led_blue.set_level(Level::Low);
                    info!("Split connected, Blue LED ON");
                    Timer::after_millis(500).await;
                    self.led_blue.set_level(Level::High);
                    info!("Blue LED OFF after 500ms");
                } else if !connected {
                    self.led_red.set_level(Level::Low);
                    info!("Split disconnected, Red LED ON");
                    Timer::after_millis(500).await;
                    self.led_red.set_level(Level::High);
                    info!("Red LED OFF after 500ms");
                }
            }
            _ => {}
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}
