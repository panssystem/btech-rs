use bevy::prelude::Component;

#[derive(Component)]
pub struct Water {
    pub depth: u32,
}

#[derive(Component)]
pub struct Height(pub i32);
