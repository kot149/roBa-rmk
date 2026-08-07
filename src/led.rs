use embassy_nrf::gpio::{Level, Output};
use embassy_time::{Duration, Instant};
use rmk::event::{CentralConnectedEvent, ConnectionStatusChangeEvent};
use rmk::macros::processor;
use rmk::types::ble::BleState;

use crate::led_state::{BleConnectionState, BleLedState, LedColor, SplitLedState};

const DISPLAY_TIME: Duration = Duration::from_millis(750);
const DISPLAY_GAP: Duration = Duration::from_millis(750);

struct RgbLed {
    red: Output<'static>,
    green: Output<'static>,
    blue: Output<'static>,
}

impl RgbLed {
    fn new(red: Output<'static>, green: Output<'static>, blue: Output<'static>) -> Self {
        Self { red, green, blue }
    }

    fn off(&mut self) {
        self.red.set_level(Level::High);
        self.green.set_level(Level::High);
        self.blue.set_level(Level::High);
    }

    fn set_color(&mut self, color: LedColor) {
        self.off();

        match color {
            LedColor::Blue => self.blue.set_level(Level::Low),
            LedColor::Red => self.red.set_level(Level::Low),
            LedColor::Yellow => {
                self.red.set_level(Level::Low);
                self.green.set_level(Level::Low);
            }
        }
    }
}

struct LedPulse {
    led: RgbLed,
    active_until: Option<Instant>,
    next_show_at: Option<Instant>,
    pending_color: Option<LedColor>,
}

impl LedPulse {
    fn new(led: RgbLed) -> Self {
        Self {
            led,
            active_until: None,
            next_show_at: None,
            pending_color: None,
        }
    }

    fn show(&mut self, color: LedColor) {
        let now = Instant::now();
        if self.active_until.is_some()
            || self
                .next_show_at
                .is_some_and(|next_show_at| now < next_show_at)
        {
            self.pending_color = Some(color);
            return;
        }

        self.start(color, now);
    }

    fn clear(&mut self) {
        self.led.off();
        self.active_until = None;
        self.next_show_at = None;
        self.pending_color = None;
    }

    fn start(&mut self, color: LedColor, now: Instant) {
        self.led.set_color(color);
        self.active_until = Some(now + DISPLAY_TIME);
    }

    fn poll(&mut self) {
        let now = Instant::now();
        if self.active_until.is_some_and(|deadline| now >= deadline) {
            self.led.off();
            self.active_until = None;
            self.next_show_at = Some(now + DISPLAY_GAP);
        }

        if self
            .next_show_at
            .is_some_and(|next_show_at| now >= next_show_at)
        {
            self.next_show_at = None;
            if let Some(color) = self.pending_color.take() {
                self.start(color, now);
            }
        }
    }
}

#[processor(subscribe = [ConnectionStatusChangeEvent], poll_interval = 50)]
pub struct BleConnectionLed<F>
where
    F: Fn() -> bool,
{
    pulse: LedPulse,
    has_bond: F,
    state: BleLedState,
}

impl<F> BleConnectionLed<F>
where
    F: Fn() -> bool,
{
    pub fn new(
        red: Output<'static>,
        green: Output<'static>,
        blue: Output<'static>,
        has_bond: F,
    ) -> Self {
        Self {
            pulse: LedPulse::new(RgbLed::new(red, green, blue)),
            has_bond,
            state: BleLedState::new(),
        }
    }

    async fn on_connection_status_change_event(&mut self, event: ConnectionStatusChangeEvent) {
        let ble = event.0.ble;
        let has_bond = match ble.state {
            BleState::Advertising => (self.has_bond)(),
            _ => false,
        };
        let state = match ble.state {
            BleState::Advertising => BleConnectionState::Advertising,
            BleState::Connected => BleConnectionState::Connected,
            BleState::Inactive => BleConnectionState::Inactive,
        };
        let update = self.state.update(ble.profile, state, has_bond);

        if update.clear {
            self.pulse.clear();
        }
        if let Some(color) = update.color {
            self.pulse.show(color);
        }
    }

    async fn poll(&mut self) {
        if let Some(color) = self.state.initial_color() {
            self.pulse.show(color);
        }
        self.pulse.poll();
    }
}

#[processor(subscribe = [CentralConnectedEvent], poll_interval = 50)]
pub struct SplitConnectionLed {
    pulse: LedPulse,
    state: SplitLedState,
}

impl SplitConnectionLed {
    pub fn new(red: Output<'static>, green: Output<'static>, blue: Output<'static>) -> Self {
        Self {
            pulse: LedPulse::new(RgbLed::new(red, green, blue)),
            state: SplitLedState::new(),
        }
    }

    async fn on_central_connected_event(&mut self, event: CentralConnectedEvent) {
        self.pulse.show(self.state.update(event.connected));
    }

    async fn poll(&mut self) {
        if let Some(color) = self.state.initial_color() {
            self.pulse.show(color);
        }
        self.pulse.poll();
    }
}
