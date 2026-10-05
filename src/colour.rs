use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ColourConfig {
    pub background: u32,
    pub button_active_background: u32,
    pub button_active_text: u32,
    pub button_inactive_background: u32,
    pub button_inactive_text: u32,
}

impl Default for ColourConfig {
    fn default() -> Self {
        ColourConfig {
            background: 0x0f0f0f,
            button_active_background: 0x2a2a2a,
            button_active_text: 0x89b4fa,
            button_inactive_background: 0x0f0f0f,
            button_inactive_text: 0xa6adc8,
        }
    }
}

pub fn color_from_u32(value: u32) -> iced::Color {
    let r = ((value >> 16) & 0xff) as u8;
    let g = ((value >> 8) & 0xff) as u8;
    let b = (value & 0xff) as u8;
    iced::Color::from_rgb8(r, g, b)
}
