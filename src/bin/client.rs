use std::time::Duration;

use bevy::{
    color,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    utils::hashbrown::HashMap,
    winit::WinitSettings,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use btech_rs::{
    constants::*,
    movement::{HexType, Level, MapHex},
    resources::*,
    states::Mode,
    systems::*,
};
use hexx::{Hex, HexLayout, PlaneMeshBuilder};

fn main() {
    let mut binding = App::new();
    let app = binding
        .init_resource::<Highlighted>()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(50)),
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(
                1000,
            )),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1000.0, 1000.0).into(),
                // cuicui_layout_bevy_ui::Plugin,
                ..default()
            }),
            ..default()
        }))
        .init_state::<Mode>()
        .add_systems(Startup, (setup_camera, draw_grid.after(setup_grid)))
        .add_systems(OnEnter(Mode::Menu), setup_menu)
        .add_systems(OnExit(Mode::Menu), teardown_menu)
        .add_systems(OnEnter(Mode::Map), setup_grid)
        .add_systems(
            Update,
            (
                handle_hover.run_if(in_state(Mode::Map)),
                button_system.run_if(in_state(Mode::Menu)),
            ),
        );

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
    commands.spawn((Transform::from_xyz(470.0, 450.0, 0.0), Camera2d {}));
}

fn build_offset_rectangle([left, right, top, bottom]: [i32; 4]) -> impl Iterator<Item = Hex> {
    (left..=right).flat_map(move |x| {
        ((top)..=(bottom))
            .map(move |y| Hex::from_offset_coordinates([x, y], OFFSET_HEX_MODE, HEX_ORIENTATION))
    })
}
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let layout = HexLayout {
        scale: HEX_SIZE,
        ..default()
    };
    // let (camera,cam_transform) = cameras.single();
    let bare_material = materials.add(Color::Srgba(color::palettes::css::BEIGE));
    let highlighted_material = materials.add(Color::Srgba(color::palettes::css::RED));
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);
    let top_left = Hex::from_offset_coordinates([1, 1], OFFSET_HEX_MODE, HEX_ORIENTATION);
    let bottom_right = Hex::from_offset_coordinates([16, 17], OFFSET_HEX_MODE, HEX_ORIENTATION);
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
                    // ColorMesh2dBundle {
                    //     transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    //     mesh: mesh_handle.clone().into(),
                    //     material: bare_material.clone(),
                    //     ..default()
                    // },
                    Mesh2d(mesh_handle.clone().into()),
                    MeshMaterial2d(bare_material.clone()),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                    HexType::Water(None, 1),
                    Level::Height(hex.x - 6),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text2d::new(format!(
                            "{},{}",
                            hex.to_offset_coordinates(OFFSET_HEX_MODE, HEX_ORIENTATION)[0],
                            hex.to_offset_coordinates(OFFSET_HEX_MODE, HEX_ORIENTATION)[1],
                        )),
                        TextFont {
                            font_size: 9.0,
                            ..default()
                        },
                        TextColor(Color::BLACK.into()),
                        Transform::from_xyz(0.0, 18.0, 10.0),
                    ));
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
        map_hexes: HashMap::<Hex, MapHex>::new(),
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
                    b.spawn((
                        Transform::from_xyz(0.0, -10.0, 10.0),
                        Text2d::new(format!("Height {}", h,)),
                        TextFont {
                            font_size: 9.0,
                            ..Default::default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
            }
            _ => (),
        }
        if let HexType::Water(_, h) = hex_type {
            commands.entity(e).with_children(|b| {
                b.spawn((
                    Transform::from_xyz(0.0, -16.0, 10.0),
                    Text2d::new(format!("Depth {}", h,)),
                    TextFont {
                        font_size: 9.0,
                        ..Default::default()
                    },
                    TextColor(Color::BLACK),
                ));
            });
        }
    }
}
