use bevy::prelude::Component;

use crate::units::{Infantry, Unit, Vehicle};

pub enum MoveMode {
    Walk,
    Run,
    Jump,
    Cruise,
    Flank,
}

pub enum MoveType {
    Mech,
    Wheeled,
    Tracked,
    Hover,
    VTOL,
    WiGE,
    UMU, // todo: special feature?
    Naval,
    Submarine,
    // Hydrofoil,
    Infantry,
}

pub enum MoveAction {
    Move,
    Stand,
    Drop,
    Lateral,
    Jump,
    Ascend,
    Descend,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Facing {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Clone, Component)]
pub enum HexType {
    Clear,
    Paved(Option<Vec<Facing>>), // Which directions does the road leave. For tracking if something is on it or not.
    Rough(Option<Vec<Facing>>),
    LightWoods(Option<Vec<Facing>>),
    HeavyWoods(Option<Vec<Facing>>),
    Water(Option<Vec<Facing>>, i8), //Depth
    Rubble(Option<Vec<Facing>>),
    LightBuilding(Option<Vec<Facing>>),
    MediumBuilding(Option<Vec<Facing>>),
    HeavyBuilding(Option<Vec<Facing>>),
    HardenedBuilding(Option<Vec<Facing>>),
}

pub enum MoveCost {
    Allowed(i8),
    AllowedWithCheck(i8),
    Prohibited,
}

pub struct Coord(i32, i32);

pub struct Hex {
    hex_type: HexType,
    level: i8,
    x: i32,
    y: i32,
}
impl Hex {
    fn try_move_into(&self, unit: Unit) -> MoveCost {
        let veh_move_type = unit.get_move_type();
        move_cost_to_hex_type(self.hex_type.clone(), veh_move_type, unit.get_facing())
    }
}

fn move_cost_to_hex_type(hex_type: HexType, veh_move_type: MoveType, facing: &Facing) -> MoveCost {
    match hex_type {
        HexType::Clear => move_clear_or_paved(veh_move_type),
        HexType::Paved(facings) => move_clear_or_paved(veh_move_type),
        HexType::Rough(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => move_rough_or_rubble(veh_move_type),
        },
        HexType::LightWoods(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => match veh_move_type {
                MoveType::Naval => MoveCost::Prohibited,
                MoveType::Submarine => MoveCost::Prohibited,
                MoveType::Wheeled => MoveCost::Prohibited, // todo: unless it's got bicycle or monocycle
                MoveType::Hover => MoveCost::Prohibited, // todo: unless it's got bicycle or monocycle
                MoveType::VTOL => MoveCost::Prohibited,  // todo: unless it's higher than woods
                MoveType::WiGE => MoveCost::Prohibited,  // todo: unless it's higher than woods
                MoveType::Infantry => MoveCost::Allowed(1), // todo: unless it's mechanized, in which case it's 2.
                _ => MoveCost::Allowed(2),
            },
        },
        HexType::HeavyWoods(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => match veh_move_type {
                MoveType::Naval => MoveCost::Prohibited,
                MoveType::Submarine => MoveCost::Prohibited,
                MoveType::Wheeled => MoveCost::Prohibited,
                MoveType::Hover => MoveCost::Prohibited,
                MoveType::VTOL => MoveCost::Prohibited, // todo: unless it's higher than woods
                MoveType::WiGE => MoveCost::Prohibited, // todo: unless it's higher than woods
                MoveType::Infantry => MoveCost::Allowed(2), // todo: unless it's mechanized, in which case it's 2.
                _ => MoveCost::Allowed(3),
            },
        },
        HexType::Water(road_facings, depth) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => todo!(),
        },
        HexType::Rubble(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => move_rough_or_rubble(veh_move_type),
        },
        HexType::LightBuilding(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => todo!(),
        },
        HexType::MediumBuilding(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => todo!(),
        },
        HexType::HeavyBuilding(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => todo!(),
        },
        HexType::HardenedBuilding(road_facings) => match road_facings {
            Some(facings) if facings.contains(facing) => move_clear_or_paved(veh_move_type),
            _ => todo!(),
        },
    }
}

fn move_rough_or_rubble(veh_move_type: MoveType) -> MoveCost {
    match veh_move_type {
        MoveType::Naval => MoveCost::Prohibited,
        MoveType::Submarine => MoveCost::Prohibited,
        MoveType::Wheeled => MoveCost::Prohibited,
        _ => MoveCost::Allowed(2),
    }
}

fn move_clear_or_paved(veh_move_type: MoveType) -> MoveCost {
    match veh_move_type {
        MoveType::Naval => MoveCost::Prohibited,
        MoveType::Submarine => MoveCost::Prohibited,
        MoveType::Wheeled => {
            // todo: if wheeled SupportVehicle, and features don't include Off-Road Chassis, add 1. Otherwise...
            MoveCost::AllowedWithCheck(1)
        }
        _ => MoveCost::AllowedWithCheck(1),
    }
}

pub mod map {
    use std::collections::HashMap;

    use super::{Coord, Hex};
    pub struct Map {
        hexes: HashMap<Coord, Hex>,
    }
    impl Map {
        pub fn new(size: Coord) -> Self {
            Map {
                hexes: HashMap::with_capacity((size.0 * size.1).try_into().unwrap_or_else(|_| 0)),
            }
        }
    }
}
