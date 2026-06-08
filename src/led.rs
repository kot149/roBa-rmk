#![allow(dead_code)]

use defmt::info;
use embassy_nrf::gpio::{Level, Output};
use embassy_time::Timer;
use rmk::types::ble::BleState;

pub struct BleConnectionLed {
    led_blue: Output<'static>,
    led_red: Output<'static>,
    connected: bool,
}

impl BleConnectionLed {
    pub fn new(led_blue: Output<'static>, led_red: Output<'static>) -> Self {
        Self {
            led_blue,
            led_red,
            connected: false,
        }
    }

    pub async fn run(&mut self) -> ! {
        use rmk::event::SubscribableEvent as _;
        use rmk::event::EventSubscriber as _;
        use rmk::event::ConnectionStatusChangeEvent;
        let mut sub = ConnectionStatusChangeEvent::subscriber();
        loop {
            let event = sub.next_event().await;
            let status = event.0; // ConnectionStatus
            let state = status.ble.state; // BleState
            match state {
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
                BleState::Inactive | BleState::Advertising => {
                    if self.connected {
                        self.connected = false;
                        self.led_red.set_level(Level::Low);
                        info!("BLE disconnected, Red LED ON");
                        Timer::after_millis(500).await;
                        self.led_red.set_level(Level::High);
                        info!("Red LED OFF after 500ms");
                    }
                }
            }
        }
    }
}

pub struct SplitConnectionLed {
    led_blue: Output<'static>,
    led_red: Output<'static>,
}

impl SplitConnectionLed {
    pub fn new(led_blue: Output<'static>, led_red: Output<'static>) -> Self {
        Self {
            led_blue,
            led_red,
        }
    }

    pub async fn run(&mut self) -> ! {
        use rmk::event::SubscribableEvent as _;
        use rmk::event::EventSubscriber as _;
        use rmk::event::CentralConnectedEvent;
        let mut sub = CentralConnectedEvent::subscriber();
        loop {
            let event = sub.next_event().await;
            if event.connected {
                self.led_blue.set_level(Level::Low);
                info!("Split connected, Blue LED ON");
                Timer::after_millis(500).await;
                self.led_blue.set_level(Level::High);
                info!("Blue LED OFF after 500ms");
            } else {
                self.led_red.set_level(Level::Low);
                info!("Split disconnected, Red LED ON");
                Timer::after_millis(500).await;
                self.led_red.set_level(Level::High);
                info!("Red LED OFF after 500ms");
            }
        }
    }
}
