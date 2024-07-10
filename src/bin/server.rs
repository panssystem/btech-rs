use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(_commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
}
