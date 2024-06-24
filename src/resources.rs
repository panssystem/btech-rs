use bevy::{asset::Handle, prelude::{Entity, Resource}, sprite::ColorMaterial, utils::HashMap};
use hexx::{Hex, HexLayout};

use crate::units::Unit;

#[derive(Debug, Default, Resource)]
pub struct Highlighted {
    pub hovered: Hex,
    pub unit: Option<Unit>,
}

#[derive(Debug, Resource)]
pub struct Map {
    pub layout: HexLayout,
    pub entities: HashMap<Hex, Entity>,
    pub bare_material: Handle<ColorMaterial>,
    pub highlighted_material: Handle<ColorMaterial>,
}