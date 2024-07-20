use hexx::Hex;

use crate::movement::{Facing, MoveMode, MoveType};

// trait Vehicle {
//     fn get_move_type(&self) -> MoveType;
//     fn get_facing(&self) -> &Facing;
//     fn get_move_points(&self) -> i8;
//     fn get_level(&self) -> i8;
//     fn get_elevation(&self) -> i8;
//     fn get_altitude(&self) -> Option<i8>;
//     fn get_unit(&self) -> &Unit;
// }
#[derive(Debug)]
pub enum VehicleType {
    Combat,
    Support,
}

#[derive(Debug)]
pub struct Unit {
    unit_type: UnitType,
    facing: Facing, 
    position: Hex,
}

#[derive(Debug)]
pub enum UnitType {
    Mech(BattleMech),
    BattleArmor(BattleArmor),
    Infantry(Infantry),
    Vehicle(Vehicle),
    Fighter(Fighter),
    DropShip(DropShip),
    ProtoMech(ProtoMech),
    SmallCraft(SmallCraft),
    JumpShip(JumpShip),
    WarShip(WarShip),
    SpaceStation(SpaceStation),
}

// impl Vehicle for Unit {
impl Unit {
    pub fn get_move_type(&self) -> MoveType {
        todo!()
    }

    pub fn get_facing(&self) -> &Facing {
        todo!()
    }

    pub fn get_move_points(&self) -> i8 {
        todo!()
    }

    pub fn get_level(&self) -> i8 {
        todo!()
    }

    pub fn get_elevation(&self) -> i8 {
        todo!()
    }

    pub fn get_altitude(&self) -> Option<i8> {
        todo!()
    }
}

#[derive(Debug)]
pub struct BattleMech {
    // name: String,
}

#[derive(Debug)]
pub struct ProtoMech {
    // name: String,
}

#[derive(Debug)]
pub struct Vehicle {
    name: String,
    vehicle_type: VehicleType,
    movement_mode: MoveType,
}

#[derive(Debug)]
pub struct Infantry {
    // name: String,
}

#[derive(Debug)]
pub struct BattleArmor {
    // name: String,
}

#[derive(Debug)]
pub struct Fighter {
    // name: String,
}

#[derive(Debug)]
pub struct SmallCraft {
    // name: String,
}

#[derive(Debug)]
pub struct DropShip {
    // name: String,
}
#[derive(Debug)]
pub struct JumpShip {}
#[derive(Debug)]
pub struct WarShip {}
#[derive(Debug)]
pub struct SpaceStation {}
