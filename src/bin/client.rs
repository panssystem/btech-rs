use std::time::Duration;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    winit::WinitSettings,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use btech_rs::{
    constants::*,
    movement::{HexType, Level},
    resources::*,
    systems::*,
};
use hexx::{Hex, HexLayout, PlaneMeshBuilder};

fn main() {
    let mut binding = App::new();
    let app = binding
        .init_resource::<Highlighted>()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                wait: Duration::from_millis(50),
            },
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                wait: Duration::from_millis(1000),
            },
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1000.0, 1000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (setup_camera, setup_grid, draw_grid.after(setup_grid)),
        )
        .add_systems(Update, handle_hover);

    if cfg!(feature = "debug") {
        add_world_inspector(app);
    }
    app.run();
}

#[cfg(feature = "debug")]
fn add_world_inspector(app: App) -> App {
    app.add_plugins(WorldInspectorPlugin::new())
}

#[cfg(not(feature = "debug"))]
fn add_world_inspector(_app: &mut App) {}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(470.0, -450.0, 0.0),
        ..default()
    });
}

fn build_offset_rectangle([left, right, top, bottom]: [i32; 4]) -> impl Iterator<Item = Hex> {
    (left..=right).flat_map(move |x| {
        ((top)..=(bottom)).map(move |y| Hex::from_offset_coordinates([x, y], OFFSET_HEX_MODE))
    })
}
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    // let (camera,cam_transform) = cameras.single();
    let bare_material = materials.add(Color::BEIGE);
    let highlighted_material = materials.add(Color::RED);
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);
    let top_left = Hex::from_offset_coordinates([1, 1], OFFSET_HEX_MODE);
    let bottom_right = Hex::from_offset_coordinates([16, 17], OFFSET_HEX_MODE);
    info!(
        "{:?} {:?} {:?} {:?}",
        top_left,
        bottom_right,
        layout.hex_to_world_pos(top_left),
        layout.hex_to_world_pos(bottom_right)
    );
    let entities = build_offset_rectangle([1, 17, 1, 16])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    ColorMesh2dBundle {
                        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                        mesh: mesh_handle.clone().into(),
                        material: bare_material.clone(),
                        ..default()
                    },
                    HexType::Water(None, 1),
                    Level::Height(hex.x - 6),
                ))
                .with_children(|b| {
                    b.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!(
                                "{},{}",
                                hex.to_offset_coordinates(OFFSET_HEX_MODE)[0],
                                hex.to_offset_coordinates(OFFSET_HEX_MODE)[1],
                            ),
                            TextStyle {
                                font_size: 9.0,
                                color: Color::BLACK,

                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 18.0, 10.0),
                        ..default()
                    });
                })
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(Map {
        layout,
        entities,
        bare_material,
        highlighted_material,
    });
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.98))
        // .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

fn draw_grid(mut commands: Commands, hexes: Query<(Entity, &Level, &HexType)>) {
    for (e, height, hex_type) in &hexes {
        match height {
            Level::Height(0) => (), // Don't show height if it's 0.
            Level::Height(h) => {
                commands.entity(e).with_children(|b| {
                    b.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("Height {}", h,),
                            TextStyle {
                                font_size: 9.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(0.0, -10.0, 10.0),
                        ..default()
                    });
                });
            }
            _ => (),
        }
        if let HexType::Water(_, h) = hex_type {
            commands.entity(e).with_children(|b| {
                b.spawn(Text2dBundle {
                    text: Text::from_section(
                        format!("Depth {}", h,),
                        TextStyle {
                            font_size: 9.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, -16.0, 10.0),
                    ..default()
                });
            });
        }
    }
}
