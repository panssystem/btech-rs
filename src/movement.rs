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

#[derive(Clone, Copy)]
pub enum HexType<'a> {
    Clear,
    Paved,
    Road(&'a Vec<Facing>, &'a HexType<'a>), // Which directions does the road leave. For tracking if something is on it or not.
    Rough,
    LightWoods,
    HeavyWoods,
    Water(i8), //Depth
    Rubble,
    LightBuilding,
    MediumBuilding,
    HeavyBuilding,
    HardenedBuilding,
}

pub enum MoveCost {
    Allowed(i8),
    AllowedWithCheck(i8),
    Prohibited,
}

pub struct Hex<'a> {
    hex_type: HexType<'a>,
    level: i8,
    x: i32,
    y: i32,
}
impl<'a> Hex<'a> {
    fn try_move_into(&'a self, unit: Unit, from: Facing) -> MoveCost {
        let veh_move_type = unit.get_move_type();
        move_cost_to_hex_type(self.hex_type, veh_move_type, unit.get_facing())
    }
}

fn move_cost_to_hex_type(hex_type: HexType, veh_move_type: MoveType, facing: &Facing) -> MoveCost {
    match hex_type {
        HexType::Clear => move_clear_or_paved(veh_move_type),
        HexType::Paved => move_clear_or_paved(veh_move_type),
        HexType::Road(facings, underlying) => {
            if facings.contains(facing) {
                move_clear_or_paved(veh_move_type)
            } else {
                move_cost_to_hex_type(*underlying, veh_move_type, facing)
            }
        }
        HexType::Rough => move_rough_or_rubble(veh_move_type),
        HexType::LightWoods => {
            match veh_move_type {
                MoveType::Naval => MoveCost::Prohibited,
                MoveType::Submarine => MoveCost::Prohibited,
                MoveType::Wheeled => MoveCost::Prohibited, // todo: unless it's got bicycle or monocycle
                MoveType::Hover => MoveCost::Prohibited, // todo: unless it's got bicycle or monocycle
                MoveType::VTOL => MoveCost::Prohibited,  // todo: unless it's higher than woods
                MoveType::WiGE => MoveCost::Prohibited,  // todo: unless it's higher than woods
                MoveType::Infantry => MoveCost::Allowed(1), // todo: unless it's mechanized, in which case it's 2.
                _ => MoveCost::Allowed(2),
            }
        }
        HexType::HeavyWoods => {
            match veh_move_type {
                MoveType::Naval => MoveCost::Prohibited,
                MoveType::Submarine => MoveCost::Prohibited,
                MoveType::Wheeled => MoveCost::Prohibited,
                MoveType::Hover => MoveCost::Prohibited,
                MoveType::VTOL => MoveCost::Prohibited, // todo: unless it's higher than woods
                MoveType::WiGE => MoveCost::Prohibited, // todo: unless it's higher than woods
                MoveType::Infantry => MoveCost::Allowed(2), // todo: unless it's mechanized, in which case it's 2.
                _ => MoveCost::Allowed(3),
            }
        }
        HexType::Water(depth) => todo!(),
        HexType::Rubble => move_rough_or_rubble(veh_move_type),
        HexType::LightBuilding => todo!(),
        HexType::MediumBuilding => todo!(),
        HexType::HeavyBuilding => todo!(),
        HexType::HardenedBuilding => todo!(),
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
    pub struct Map {}
}