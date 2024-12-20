use bevy::prelude::*;
use hexx::OffsetHexMode;

pub const HEX_SIZE: Vec2 = Vec2::splat(26.0);
pub const OFFSET_HEX_MODE: OffsetHexMode = OffsetHexMode::EvenColumns;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);