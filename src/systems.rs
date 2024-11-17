use bevy::{prelude::*, window::PrimaryWindow};
// use cuicui_dsl::{dsl, BuildChildren, EntityCommands};
// use cuicui_layout::dsl_functions::*;
// use cuicui_layout_bevy_ui::UiDsl;

use crate::{
    constants::{NORMAL_BUTTON, OFFSET_HEX_MODE},
    resources::{Highlighted, Map},
};

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


pub fn setup_menu( mut commands:Commands, serv: Res<AssetServer>) {
    let menu_items = [
        "Start Game",
        "View Unit",
        "View Map",
        "Campaign",
    ];
    // let button_bg = serv.load("button.png");

    // dsl! {
    //     <UiDsl>
    //     &mut commands.spawn_empty(),
    //     Root(screen_root row distrib_start main_margin(50.) image(&bg)) {
    //         code(let commands) {
    //             dsl! { <UiDsl> commands,
    //                 ButtonContainer(column rules(pct(100), pct(60)))
    //             };
    //             cmds.with_children(|commands| {
    //                 for text in menu_items {
    //                     helpers::button(&mut commands.spawn_empty(), &button_bg, text);
    //                 }
    //             });
    //         }
    //     }
    // };
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();
    // commands.insert_resource(MenuData { button_entity });

}
mod helpers {
    use bevy::{prelude::*, window::PrimaryWindow};
    // use cuicui_dsl::{dsl, EntityCommands};
    // use cuicui_layout_bevy_ui::UiDsl;
    // use cuicui_layout::dsl_functions::*;

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

    // fn button(cmds: &mut EntityCommands, button_bg: &Handle<Image>, button_text: &'static str) {
    //     dsl! {
    //         <UiDsl> cmds,
    //         Entity(text(button_text) named(button_text) image(button_bg) width(pct(80)))
    //     }
    // }
}
