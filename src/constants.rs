use bevy::prelude::*;
use hexx::{HexOrientation, OffsetHexMode};

pub const HEX_SIZE: Vec2 = Vec2::splat(26.0);
pub const OFFSET_HEX_MODE: OffsetHexMode = OffsetHexMode::Even;
pub const HEX_ORIENTATION: HexOrientation = HexOrientation::Flat;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// For imports:
pub mod imports {
    pub const UNIT_TYPE: &str = "UnitType";
    pub const TANK_TYPE: &str = "Tank";
    pub const UNIT_NAME: &str = "Name";
    pub const MOTION_TYPE: &str = "motion_type";
    pub const MECH_CONFIG: &str = "Config";
    pub const MUL_ID: &str = "mul id";
    pub const MECH_CHASSIS: &str = "chassis";
    pub const MECH_MODEL: &str = "model";
    pub const MECH_MYOMER: &str = "myomer";
    pub const MECH_STRUCTURE: &str = "structure";
    pub const TECH_BASE: &str = "techbase";
}
