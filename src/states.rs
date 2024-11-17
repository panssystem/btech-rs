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
