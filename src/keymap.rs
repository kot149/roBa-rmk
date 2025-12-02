use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, lt};

pub(crate) const COL: usize = 11;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 1;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0 - Default
        [
            [k!(Q),     k!(W),    k!(E),    k!(R),         lt!(7, T),  a!(No),                     lt!(7, Y),  k!(U),        k!(I),     k!(O),   k!(P)],
            [k!(A),     k!(S),    k!(D),    k!(F),         k!(G),      a!(No),                     k!(H),      k!(J),        k!(K),     k!(L),   k!(Minus)],
            [k!(Z),     k!(X),    k!(C),    k!(V),         k!(B),      a!(No),                     k!(N),      k!(M),        k!(Comma), k!(Dot), k!(Slash)],
            [k!(LCtrl), k!(LGui), k!(LAlt), k!(Language2), k!(Space),  k!(Tab),  a!(No),  a!(No),  k!(Backspace), k!(Enter),                     k!(Escape)],
        ],
        // Layer 1
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // Layer 2
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // Layer 3
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // Layer 4
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // Layer 5
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // Layer 6
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // Layer 7 - Configuration
        [
            [k!(Bootloader), k!(Reboot), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(User8), k!(Reboot), k!(Bootloader)],
            [a!(No),         a!(No),     a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),    a!(No),     a!(No)],
            [a!(No),         a!(No),     a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),    a!(No),     k!(User7)],
            [a!(No),         a!(No),     a!(No), a!(No), a!(No), a!(No), k!(User6), k!(User5), k!(User0), k!(User1), a!(No)],
        ],
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
    ]
}
