use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
    sprite::ColorMaterial,
    utils::HashMap,
};
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
impl Map {
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
