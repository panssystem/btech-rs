use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
    sprite::ColorMaterial,
};
use hexx::{Hex, HexLayout};
use std::collections::HashMap;

use crate::{
    import::read_map,
    {
        movement::{map::MapBoard, MapHex},
        units::Unit,
    },
};

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
    pub map_hexes: HashMap<Hex, MapHex>,
    // pub tileset: HashMap<&str, Tile>
}

impl Map {
    pub fn get_map_hex(&self, hex: Hex) -> Option<&MapHex> {
        self.map_hexes.get(&hex)
    }
    // pub fn move_cost(&self, unit: Unit, hex: Hex) -> Option<u32> {
    //     match hexx::algorithms::a_star(hex, hex, |start, end| {
    // map_start = self.get_map_hex(start);
    // map_end = self.get_map_hex(end);
    // movement_cost(unit, map_start,map_end)
    //         Some(1)
    //     }) {
    //         Some(route) => {
    //             let mut cost = 0u32;

    //             for i in 1..route.len() {
    //                 cost += movement_cost(unit, self.get_map_hex(route[i - 1]), self.get_map_hex(route[i]));
    //             }
    //             Some(cost)
    //         }
    //         None => None,
    //     }
    // }
    // fn get_map_hex(&self, hex: Hex) -> MapHex {
    // self.map_hexes.get(hex)
    // }
}

#[derive(Debug, Resource)]
pub struct MenuData {
    pub start_game_button: Entity,
    // pub view_unit_button: Entity,
    // pub view_map_button: Entity,
    // pub campaign_button: Entity,
}
// struct Tile {}
pub struct ResourceLoader {
    root: String,
}
impl ResourceLoader {
    pub fn load_map(map_name: String) -> Option<MapBoard> {
        read_map(map_name)
    }
}
