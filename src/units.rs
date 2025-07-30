use hexx::Hex;

use crate::movement::{Facing, MoveType};

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
    #[cfg(feature = "infantry")]
    BattleArmor(BattleArmor),
    #[cfg(feature = "infantry")]
    Infantry(Infantry),
    #[cfg(feature = "vehicle")]
    Vehicle(Vehicle),
    #[cfg(feature = "aerospace")]
    Fighter(Fighter),
    #[cfg(feature = "dropship")]
    DropShip(DropShip),
    #[cfg(feature = "protomech")]
    ProtoMech(ProtoMech),
    #[cfg(feature = "vehicle")]
    SmallCraft(SmallCraft),
    #[cfg(feature = "aerospace")]
    JumpShip(JumpShip),
    #[cfg(feature = "aerospace")]
    WarShip(WarShip),
    #[cfg(feature = "aerospace")]
    SpaceStation(SpaceStation),
    Invalid,
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
pub struct Location {
    pub name: String,
    pub slots: i8,
    pub structure: i8,
    pub armor: i8,
    pub rear_armor: Option<i8>,
    pub components: Vec<Component>,
}

#[derive(Debug)]
pub struct Component {
    name: String,
    weight: f32,
    slots: i8,
}

#[derive(Debug)]
pub enum MechConfig {
    Biped,
    BipedOmni,
    Quad,
    QuadOmni,
    QuadVee,
    QuadVeeOmni,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
pub enum TechBase {
    Clan,
    InnerSphere,
}

#[derive(Debug)]
pub enum ChassisTech {
    Single(TechBase),
    Mixed(TechBase),
}
#[derive(Debug)]
pub enum Structure {
    Standard(TechBase),
    EndoSteel(TechBase),
}

#[derive(Debug)]
pub enum Armor {
    Standard(TechBase),
    FerroFibrous(TechBase),
    HeavyFerroFibrous(TechBase),
    Stealth(TechBase),
    FerroLamellor(TechBase),
    Reactive(TechBase),
}
#[derive(Debug)]
pub struct BattleMech {
    pub chassis: String,
    pub model: String,
    pub mass: i8,
    pub structure: Structure,
    pub myomer: String,
    pub armor: Armor,
    pub locations: Vec<Location>,
    pub config: MechConfig,
}

#[cfg(feature = "protomech")]
#[derive(Debug)]
pub struct ProtoMech {
    // name: String,
}

#[cfg(feature = "vehicle")]
#[derive(Debug)]
pub struct Vehicle {
    pub name: String,
    pub vehicle_type: VehicleType,
    pub movement_mode: MoveType,
}

#[cfg(feature = "infantry")]
#[derive(Debug)]
pub struct Infantry {
    // name: String,
}

#[derive(Debug)]
#[cfg(feature = "infantry")]
pub struct BattleArmor {
    // name: String,
}

#[cfg(feature = "aerospace")]
#[derive(Debug)]
pub struct Fighter {
    // name: String,
}

#[derive(Debug)]
pub struct SmallCraft {
    // name: String,
}

#[cfg(feature = "aerospace")]
#[derive(Debug)]
pub struct DropShip {
    // name: String,
}

#[cfg(feature = "aerospace")]
#[derive(Debug)]
pub struct JumpShip {}

#[cfg(feature = "aerospace")]
#[derive(Debug)]
pub struct WarShip {}

#[cfg(feature = "aerospace")]
#[derive(Debug)]
pub struct SpaceStation {}
