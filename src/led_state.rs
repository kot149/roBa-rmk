#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LedColor {
    Blue,
    Red,
    Yellow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BleConnectionState {
    Advertising,
    Connected,
    Inactive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LedUpdate {
    pub(crate) clear: bool,
    pub(crate) color: Option<LedColor>,
}

pub(crate) struct BleLedState {
    status: LedColor,
    profile: u8,
    profile_switching: bool,
    initialized: bool,
}

impl BleLedState {
    pub(crate) const fn new() -> Self {
        Self {
            status: LedColor::Red,
            profile: 0,
            profile_switching: false,
            initialized: false,
        }
    }

    pub(crate) fn initial_color(&mut self) -> Option<LedColor> {
        if self.initialized || self.profile_switching {
            return None;
        }

        self.initialized = true;
        Some(self.status)
    }

    pub(crate) fn update(
        &mut self,
        profile: u8,
        state: BleConnectionState,
        has_bond: bool,
    ) -> LedUpdate {
        let profile_changed = self.profile != profile;
        self.profile = profile;

        if profile_changed {
            self.profile_switching = true;
        }
        if self.profile_switching && state == BleConnectionState::Inactive {
            return LedUpdate {
                clear: profile_changed,
                color: None,
            };
        }

        let status = color_for_ble_state(state, has_bond);
        let changed = self.profile_switching || !self.initialized || self.status != status;

        self.profile_switching = false;
        self.initialized = true;
        self.status = status;

        LedUpdate {
            clear: profile_changed,
            color: changed.then_some(status),
        }
    }
}

fn color_for_ble_state(state: BleConnectionState, has_bond: bool) -> LedColor {
    match state {
        BleConnectionState::Connected => LedColor::Blue,
        BleConnectionState::Advertising if has_bond => LedColor::Red,
        BleConnectionState::Advertising => LedColor::Yellow,
        BleConnectionState::Inactive => LedColor::Red,
    }
}

pub(crate) struct SplitLedState {
    initialized: bool,
}

impl SplitLedState {
    pub(crate) const fn new() -> Self {
        Self { initialized: false }
    }

    pub(crate) fn initial_color(&mut self) -> Option<LedColor> {
        if self.initialized {
            return None;
        }

        self.initialized = true;
        Some(LedColor::Red)
    }

    pub(crate) fn update(&mut self, connected: bool) -> LedColor {
        self.initialized = true;
        if connected {
            LedColor::Blue
        } else {
            LedColor::Red
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BleConnectionState, BleLedState, LedColor, LedUpdate, SplitLedState};

    #[test]
    fn central_starts_red_once() {
        let mut state = BleLedState::new();

        assert_eq!(state.initial_color(), Some(LedColor::Red));
        assert_eq!(state.initial_color(), None);
    }

    #[test]
    fn advertising_is_yellow_only_without_a_bond() {
        let mut unpaired = BleLedState::new();
        let mut paired = BleLedState::new();

        assert_eq!(
            unpaired.update(0, BleConnectionState::Advertising, false),
            LedUpdate {
                clear: false,
                color: Some(LedColor::Yellow),
            }
        );
        assert_eq!(
            paired.update(0, BleConnectionState::Advertising, true),
            LedUpdate {
                clear: false,
                color: Some(LedColor::Red),
            }
        );
    }

    #[test]
    fn connected_is_blue() {
        let mut state = BleLedState::new();

        assert_eq!(
            state.update(0, BleConnectionState::Connected, true).color,
            Some(LedColor::Blue)
        );
    }

    #[test]
    fn switching_from_connected_to_unpaired_shows_only_yellow() {
        let mut state = BleLedState::new();
        state.update(0, BleConnectionState::Connected, true);

        assert_eq!(
            state.update(1, BleConnectionState::Inactive, false),
            LedUpdate {
                clear: true,
                color: None,
            }
        );
        assert_eq!(
            state.update(1, BleConnectionState::Advertising, false),
            LedUpdate {
                clear: false,
                color: Some(LedColor::Yellow),
            }
        );
    }

    #[test]
    fn profile_switch_before_initial_poll_waits_for_target_state() {
        let mut state = BleLedState::new();

        assert_eq!(
            state.update(1, BleConnectionState::Inactive, false),
            LedUpdate {
                clear: true,
                color: None,
            }
        );
        assert_eq!(state.initial_color(), None);
        assert_eq!(
            state
                .update(1, BleConnectionState::Advertising, false)
                .color,
            Some(LedColor::Yellow)
        );
    }

    #[test]
    fn switching_from_unpaired_to_paired_shows_red_then_blue() {
        let mut state = BleLedState::new();
        state.update(0, BleConnectionState::Advertising, false);

        assert_eq!(
            state.update(1, BleConnectionState::Inactive, false),
            LedUpdate {
                clear: true,
                color: None,
            }
        );
        assert_eq!(
            state.update(1, BleConnectionState::Advertising, true).color,
            Some(LedColor::Red)
        );
        assert_eq!(
            state.update(1, BleConnectionState::Connected, true).color,
            Some(LedColor::Blue)
        );
    }

    #[test]
    fn repeated_advertising_state_does_not_flash_twice() {
        let mut state = BleLedState::new();
        state.update(0, BleConnectionState::Advertising, false);

        assert_eq!(
            state.update(0, BleConnectionState::Advertising, false),
            LedUpdate {
                clear: false,
                color: None,
            }
        );
    }

    #[test]
    fn peripheral_uses_red_for_disconnected_and_blue_for_connected() {
        let mut state = SplitLedState::new();

        assert_eq!(state.initial_color(), Some(LedColor::Red));
        assert_eq!(state.update(false), LedColor::Red);
        assert_eq!(state.update(true), LedColor::Blue);
    }
}
