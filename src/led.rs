use defmt::{info, unwrap};
use rmk::controller::Controller;
use rmk::channel::{ControllerSub, CONTROLLER_CHANNEL};
use rmk::event::ControllerEvent;
use rmk::ble::BleState;
use embassy_nrf::gpio::{Output, Level};
use embassy_time::Timer;

pub struct BleConnectionLed {
    led_blue: Output<'static>,
    led_red: Output<'static>,
    sub: ControllerSub,
    last_profile: Option<u8>,
    last_connected: Option<bool>,
}

impl BleConnectionLed {
    pub fn new(led_blue: Output<'static>, led_red: Output<'static>) -> Self {
        Self {
            led_blue,
            led_red,
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            last_profile: None,
            last_connected: None,
        }
    }

    async fn indicate(&mut self, state: BleState) {
        match state {
            BleState::Connected => {
                self.led_blue.set_level(Level::Low);
                info!("BLE connected, Blue LED ON");
                Timer::after_millis(500).await;
                self.led_blue.set_level(Level::High);
                info!("Blue LED OFF after 500ms");
            }
            BleState::None | BleState::Advertising => {
                self.led_red.set_level(Level::Low);
                info!("BLE not connected, Red LED ON");
                Timer::after_millis(500).await;
                self.led_red.set_level(Level::High);
                info!("Red LED OFF after 500ms");
            }
        }
    }
}

impl Controller for BleConnectionLed {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::BleState(profile, state) => {
                let profile_id = profile as u8;
                let connected_now = matches!(state, BleState::Connected);
                let first = self.last_connected.is_none();
                let profile_changed = self.last_profile != Some(profile_id);
                let state_changed = self.last_connected != Some(connected_now);

                self.last_profile = Some(profile_id);
                self.last_connected = Some(connected_now);

                if first || profile_changed || state_changed {
                    self.indicate(state).await;
                }
            }
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
