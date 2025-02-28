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

pub mod tile {
    use crate::game::Value;
    use crate::ui::styles::srgba_hex;
    use bevy::color::Color;
    use bevy::utils::HashMap;
    use std::cell::{LazyCell, OnceCell};
    use std::sync::LazyLock;

    #[derive(Clone)]
    struct TileColor {
        background: &'static str,
        foreground: &'static str,
    }

    const DEFAULT_TILE_COLOR: TileColor = TileColor {
        background: "#000000",
        foreground: "#ffffff",
    };

    const TILE_COLORS: &[(u32, TileColor)] = &[
        (
            2,
            TileColor {
                background: "#ede5db",
                foreground: "#726554",
            },
        ),
        (
            4,
            TileColor {
                background: "#e8d9ba",
                foreground: "#726554",
            },
        ),
        (
            8,
            TileColor {
                background: "#e7b37f",
                foreground: "#feffff",
            },
        ),
    ];

    static TILE_COLORS_BY_VALUE: LazyLock<HashMap<u32, TileColor>> =
        LazyLock::new(|| TILE_COLORS.iter().map(|(v, c)| (*v, c.clone())).collect());

    pub const TILE_SIZE: f32 = 100.0;
    pub const BORDER_RADII: f32 = 12.0;

    fn tile_color_for(value: Value) -> &'static TileColor {
        TILE_COLORS_BY_VALUE
            .get(&value.as_u32())
            .unwrap_or(&&DEFAULT_TILE_COLOR)
    }

    pub fn background_color_for(value: Value) -> Color {
        srgba_hex(tile_color_for(value).background).into()
    }

    pub fn foreground_color_for(value: Value) -> Color {
        srgba_hex(tile_color_for(value).foreground).into()
    }
}
