use bevy::{prelude::*, window::PrimaryWindow};

use crate::{constants::OFFSET_HEX_MODE, resources::{Highlighted, Map}};

pub fn handle_hover(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    mut highlighted_hexes: ResMut<Highlighted>,
) {
    if let Some(pos) = helpers::get_position(windows, cameras) {
        let coord = map.layout.world_pos_to_hex(pos);
        if coord == highlighted_hexes.hovered {
            return;
        }
        info!(
            "{:?}, {:?}, {:?}",
            coord.to_offset_coordinates(OFFSET_HEX_MODE),
            coord,
            pos
        );
        // clear last hex highlighting.
        if let Some(entity) = map.entities.get(&highlighted_hexes.hovered) {
            commands
                .entity(*entity)
                .insert(map.bare_material.clone_weak());
        }
        highlighted_hexes.hovered = coord;
        if let Some(entity) = map.entities.get(&coord).copied() {
            commands
                .entity(entity)
                .insert(map.highlighted_material.clone_weak());
        }
    }
}

mod helpers {
    use bevy::{prelude::*, window::PrimaryWindow};

    pub(crate) fn get_position(
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&Camera, &GlobalTransform)>,
    ) -> Option<Vec2> {
        let window = windows.single();
        let (camera, cam_transform) = cameras.single();
        window
            .cursor_position()
            .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    }
}
