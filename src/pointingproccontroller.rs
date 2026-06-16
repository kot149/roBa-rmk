use rmk::{
    event::{LayerChangeEvent, PointingProcessorEvent, publish_event},
    input_device::pointing::{PointingMode, CursorConfig, SniperConfig, ScrollConfig, CaretConfig},
    types::keycode::HidKeyCode,
};
use rmk::macros::processor;

#[processor(subscribe = [LayerChangeEvent])]
#[derive(Default)]
pub struct PointingProcessorController;

impl PointingProcessorController {
    pub fn new() -> Self {
        Self
    }

    async fn on_layer_change_event(&mut self, event: LayerChangeEvent) {
        match event.0 {
            0 => {
                publish_event(PointingProcessorEvent {
                    device_id: 0,
                    mode: PointingMode::Cursor(CursorConfig::default()),
                });
            }
            1 => {
                publish_event(PointingProcessorEvent {
                    device_id: 0,
                    mode: PointingMode::Caret(CaretConfig {
                        disable_x: false,
                        disable_y: false,
                        invert_x: false,
                        invert_y: false,
                        threshold: 100,
                        keycode_up: HidKeyCode::Up,
                        keycode_down: HidKeyCode::Down,
                        keycode_left: HidKeyCode::Left,
                        keycode_right: HidKeyCode::Right,
                    }),
                });
            }
            2 => {
                publish_event(PointingProcessorEvent {
                    device_id: 0,
                    mode: PointingMode::Sniper(SniperConfig {
                        multiplier: 1,
                        divisor: 8,
                        invert_x: false,
                        invert_y: false,
                    }),
                });
            }
            6 => {
                publish_event(PointingProcessorEvent {
                    device_id: 0,
                    mode: PointingMode::Scroll(ScrollConfig {
                        multiplier_x: 1,
                        divisor_x: 16,
                        multiplier_y: 1,
                        divisor_y: 16,
                        invert_x: false,
                        invert_y: false,
                    }),
                });
            }
            _ => {}
        }
    }
}
