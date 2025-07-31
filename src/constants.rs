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
    use std::{collections::{hash_map, HashMap}, sync::LazyLock};

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

    pub struct LookupTable {
        pub armor: &'static str,
        pub has_rear_armor: bool,
        pub rear_armor: &'static str,
    }
    pub static LOCATION_LOOKUPS: LazyLock<HashMap<&str, LookupTable>> = LazyLock::new(|| load_location_lookups());
    fn load_location_lookups() ->  HashMap<&'static str, LookupTable>{
        HashMap::from([
        (
            "Head",
            LookupTable {
                armor: "HD armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Center Torso",
            LookupTable {
                armor: "CT armor",
                has_rear_armor: true,
                rear_armor: "RTC armor",
            },
        ),
        (
            "Left Torso",
            LookupTable {
                armor: "LT armor",
                has_rear_armor: true,
                rear_armor: "RTL armor",
            },
        ),
        (
            "Right Torso",
            LookupTable {
                armor: "RT armor",
                has_rear_armor: true,
                rear_armor: "RRT armor",
            },
        ),
        (
            "Left Arm",
            LookupTable {
                armor: "LA armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Right Arm",
            LookupTable {
                armor: "RA armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Left Leg",
            LookupTable {
                armor: "LL armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Front Left Leg",
            LookupTable {
                armor: "FLL armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Rear Left Leg",
            LookupTable {
                armor: "RLL armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Right Leg",
            LookupTable {
                armor: "RL armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Front Right Leg",
            LookupTable {
                armor: "FRL armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
        (
            "Rear Right Leg",
            LookupTable {
                armor: "RRL armor",
                has_rear_armor: false,
                rear_armor: "",
            },
        ),
    ])}
}

