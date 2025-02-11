use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    #[default]
    Menu,
    GameSetup,
    Game,
    Map,
    Campaign,
    Mech,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    #[default]
    DeploymentPhase,
    InitiativePhase,
    MovementPhase(MovementSubphase),
    WeaponAttackPhase,
    PhysicalAttackPhase,
    HeatPhase,
    EndPhase,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementSubphase {
    #[default]
    GroundMovementPhase,
    AerospaceMovementPhase,
}
