use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Hex(pub iced::Color);

impl TryFrom<String> for Hex {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
            .map(Hex)
            .map_err(|e: <iced::Color as std::str::FromStr>::Err| e.to_string())
    }
}

impl From<Hex> for String {
    fn from(h: Hex) -> Self {
        h.0.to_string()
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ColourConfig {
    pub background: Hex,
    pub button_active_background: Hex,
    pub button_active_text: Hex,
    pub button_inactive_background: Hex,
    pub button_inactive_text: Hex,
}

impl Default for ColourConfig {
    fn default() -> Self {
        ColourConfig {
            background: Hex(iced::color!(0x0f0f0f)),
            button_active_background: Hex(iced::color!(0x2a2a2a)),
            button_active_text: Hex(iced::color!(0x89b4fa)),
            button_inactive_background: Hex(iced::color!(0x0f0f0f)),
            button_inactive_text: Hex(iced::color!(0xa6adc8)),
        }
    }
}
