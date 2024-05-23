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

pub enum VehicleType {
    Combat,
    Support,
}
pub enum Unit {
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

pub struct BattleMech {
    // name: String,
}

pub struct ProtoMech {
    // name: String,
}

pub struct Vehicle {
    name: String,
    vehicle_type: VehicleType,
    movement_mode: MoveType,
}

pub struct Infantry {
    // name: String,
}

pub struct BattleArmor {
    // name: String,
}

pub struct Fighter {
    // name: String,
}

pub struct SmallCraft {
    // name: String,
}

pub struct DropShip {
    // name: String,
}

pub struct JumpShip {}
pub struct WarShip {}
pub struct SpaceStation {}
