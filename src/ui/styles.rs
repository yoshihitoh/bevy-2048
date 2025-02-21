use bevy::prelude::{Color, Srgba};

fn srgba_hex(hex: &str) -> Srgba {
    Srgba::hex(hex).unwrap()
}

pub mod window {
    use super::*;

    pub fn background_color() -> Color {
        srgba_hex("#faf8f0").into()
    }
}

pub mod board {
    use super::*;

    pub fn border_color() -> Color {
        srgba_hex("#756452").into()
    }

    pub fn background_color() -> Color {
        srgba_hex("#756452").into()
    }

    pub const BORDER_SIZE: f32 = 12.0;
    pub const BORDER_RADII: f32 = 12.0;
}

pub mod slot {
    use crate::ui::styles::srgba_hex;
    use bevy::color::Color;

    pub const SLOT_MARGIN: f32 = 12.0;
    pub const SLOT_SIZE: f32 = 100.0;
    pub const SPACING: f32 = SLOT_SIZE + SLOT_MARGIN;
    pub const BORDER_RADII: f32 = 12.0;

    pub fn background_color() -> Color {
        srgba_hex("#baad9a").into()
    }
}

pub mod cell {
    use crate::ui::styles::srgba_hex;
    use bevy::color::Color;

    pub const CELL_SIZE: f32 = 100.0;
    pub const BORDER_RADII: f32 = 12.0;

    pub fn background_color_for(_value: u32) -> Color {
        srgba_hex("#baad9a").into()
    }
}
