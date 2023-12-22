use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResolution;

use rustilples::cursor::CursorPlugin;
use rustilples::fps::FpsPlugin;
use rustilples::world_info::WorldInfoPlugin;
pub use rustilples::world_info::print_world_info;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1000., 800.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsPlugin)
        .add_plugins(WorldInfoPlugin)
        .add_systems(Startup, spawn_camera)
        .add_plugins(CursorPlugin)
        .add_systems(Startup, generate_grid)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}



use rand::Rng;

pub const GRID_CELL_WIDTH : f32 = 50.0 as f32;

pub fn generate_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let width_in_cells = (window.width() / GRID_CELL_WIDTH) as u32;
    let height_in_cells = (window.height() / GRID_CELL_WIDTH) as u32;

    let mut col_index = 0u32;
    let mut row_index = 0u32;
    loop {
        println!("{}, {}", col_index, row_index);
        if (row_index == height_in_cells && col_index == width_in_cells) { break; }

        let x = GRID_CELL_WIDTH * col_index as f32;       
        let y = GRID_CELL_WIDTH * row_index as f32;

        let r = rand::thread_rng().gen_range(0.0..0.5);
        let g = rand::thread_rng().gen_range(0.4..0.5);
        let b = rand::thread_rng().gen_range(0.0..0.5);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(r, g, b),
                custom_size: Some(Vec2::new(GRID_CELL_WIDTH, GRID_CELL_WIDTH)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, -1.)),
            ..default()
        });
    
        col_index += 1;
        if col_index > width_in_cells { 
            col_index = 0;
            row_index += 1;
        };
    }
}

// use bevy::sprite::MaterialMesh2dBundle;

// pub const HERO_SPEED: f32 = 500.0;
// pub const HERO_SIZE: f32 = 50.0;

//     .add_systems(Startup, spawn_camera)
//     .add_systems(Startup, spawn_castle)
//     .add_systems(Startup, spawn_hero)
//     .add_systems(Update, hero_movement)
//     .add_systems(Update, confine_hero_movement)
// // .add_startup_system(spawn_hero)
// .run();
//     .add_systems(Startup, setup)
//     .add_systems(Update, mouse_button_iter)
//     .run();

// #[derive(Component)]
// pub struct Hero;

// #[derive(Component, Debug)]
// pub struct Castle {
//     width: f32,
//     height: f32
// }

// #[derive(Component)]
// struct Name(String);

// pub fn spawn_castle(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();
//     let transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
//                                          .with_scale(Vec3 { x: 100, y200 0.4, z: 0.4 });

//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load("sprites/castle.png"),
//             ..default()
//         },
//         Castle {
//             width: 354.0 * 0.4,
//             height: 296.0 * 0.4
//         }
//     ));

//     let transform = Transform::from_xyz( 100.0, window.height() / 3.0, 0.0)
//                                          .with_scale(Vec3 { x: 0.4, y: 0.4, z: 0.4 });

//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load("sprites/castle2.png"),
//             ..default()
//         },
//         Castle {
//             width: 448.0 * 0.4,
//             height: 340.0 * 0.4
//         }
//     ));

//     let transform = Transform::from_xyz( window.width() - 200.0, window.height() / 3.0, 0.0)
//                                          .with_scale(Vec3 { x: 0.4, y: 0.4, z: 0.4 });

//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load("sprites/castle3.png"),
//             ..default()
//         },
//         Castle {
//             width: 353.0 * 0.4,
//             height: 353.0 * 0.4
//         }
//     ));
// }

// pub fn spawn_hero(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();

//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_xyz(30.0, 30.0, 0.0),
//             texture: asset_server.load("sprites/hero.png"),
//             ..default()
//         },
//         Hero {},
//     ));
// }

// pub fn hero_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut hero_query: Query<&mut Transform, With<Hero>>,
//     castles_query: Query<(&Castle, &Transform), Without<Hero>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     time: Res<Time>,
// ) {
//     if let Ok(mut hero_transform) = hero_query.get_single_mut() {

//         // if let Ok((hero_entity, mut hero_transform)) = hero_query.get_single_mut() {

//         // }

//         let mut direction = Vec3::ZERO;

//         if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
//             direction += Vec3::new(-1.0, 0.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
//             direction += Vec3::new(1.0, 0.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
//             direction += Vec3::new(0.0, 1.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
//             direction += Vec3::new(0.0, -1.0, 0.0);
//         }

//         if direction.length() > 0.0 {
//             direction = direction.normalize();
//         }

//         let result_direction = direction * HERO_SPEED * time.delta_seconds();
//         let window = window_query.get_single().unwrap();

//             let half_hero_size = HERO_SIZE / 2.0; // 32.0
//             let x_min = 0.0 + half_hero_size;
//             let x_max = window.width() - half_hero_size;
//             let y_min = 0.0 + half_hero_size;
//             let y_max = window.height() - half_hero_size;

//             let mut translation = hero_transform.translation;

//             // Bound the hero x position
//             if translation.x < x_min {
//                 return
//             } else if translation.x > x_max {
//                 return
//             }
//             // Bound the heros y position.
//             if translation.y < y_min {
//                 return
//             } else if translation.y > y_max {
//                 return
//             }

//             let h_max_x = hero_transform.translation.x + half_hero_size + result_direction.x;
//             let h_min_x = hero_transform.translation.x - half_hero_size - result_direction.x;
//             let h_max_y = hero_transform.translation.y + half_hero_size - result_direction.y;
//             let h_min_y = hero_transform.translation.y - half_hero_size + result_direction.y;

//             for (castle, castle_transform) in castles_query.iter() {
//                     let c_max_x = castle_transform.translation.x + castle.width / 2.0;
//                     let c_min_x = castle_transform.translation.x - castle.width / 2.0;
//                     let c_max_y = castle_transform.translation.y + castle.height / 2.0;
//                     let c_min_y = castle_transform.translation.y - castle.height / 2.0;

//                     if h_min_y < c_max_y && h_max_y > c_min_y {
//                         if h_max_x > c_min_x && h_min_x < c_max_x  {
//                             return
//                        }
//                     }
//             }

//         hero_transform.translation += result_direction;
//     }
// }

// pub fn confine_hero_movement(
//     mut hero_query: Query<(&Hero, &mut Transform), Without<Castle>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     castles_query: Query<(&Castle, &Transform), Without<Hero>>,
// ) {
//     // if let Ok((hero_entity, mut hero_transform)) = hero_query.get_single_mut() {
//     //     let window = window_query.get_single().unwrap();

//     //     let half_hero_size = HERO_SIZE / 2.0; // 32.0
//     //     let x_min = 0.0 + half_hero_size;
//     //     let x_max = window.width() - half_hero_size;
//     //     let y_min = 0.0 + half_hero_size;
//     //     let y_max = window.height() - half_hero_size;

//     //     let mut translation = hero_transform.translation;

//     //     // Bound the hero x position
//     //     if translation.x < x_min {
//     //         return
//     //     } else if translation.x > x_max {
//     //         return
//     //     }
//     //     // Bound the heros y position.
//     //     if translation.y < y_min {
//     //         return
//     //     } else if translation.y > y_max {
//     //         return
//     //     }

//     //     let h_max_x = hero_transform.translation.x + half_hero_size;
//     //     let h_min_x = hero_transform.translation.x - half_hero_size;
//     //     let h_max_y = hero_transform.translation.y + half_hero_size;
//     //     let h_min_y = hero_transform.translation.y - half_hero_size;

//     //     for (castle, castle_transform) in castles_query.iter() {
//     //             let c_max_x = castle_transform.translation.x + castle.width / 2.0;
//     //             let c_min_x = castle_transform.translation.x - castle.width / 2.0;
//     //             let c_max_y = castle_transform.translation.y + castle.height / 2.0;
//     //             let c_min_y = castle_transform.translation.y - castle.height / 2.0;

//     //             if h_min_y < c_max_y && h_max_y > c_min_y {
//     //                 if h_max_x > c_min_x && h_min_x < c_max_x  {
//     //                     return
//     //                }
//     //             }
//     //     }

//     // }
// }

// // fn spawn_castles(mut commands: Commands) {
// //     commands.spawn((Castle, Name("Elaina".to_string())));
// //     commands.spawn((Castle, Name("Renzo".to_string())));
// //     commands.spawn((Castle, Name("Zayna".to_string())));
// // }

// // fn render_castles(query: Query<&Name, With<Castle>>) {
// //     for name in &query {
// //         println!("castle {}", name.0);
// //     }
// // }

// // fn mouse_button_iter(
// //     buttons: Res<Input<MouseButton>>,
// // ) {
// //     for button in buttons.get_pressed() {
// //         println!("{:?} is currently held down", button);
// //     }
// //     for button in buttons.get_just_pressed() {
// //         println!("{:?} was pressed", button);
// //     }
// //     for button in buttons.get_just_released() {
// //         println!("{:?} was released", button);
// //     }
// // }

// // fn cursor_position(
// //     q_windows: Query<&Window, With<PrimaryWindow>>,
// // ) {
// //     // Games typically only have one window (the primary window)
// //     if let Some(position) = q_windows.single().cursor_position() {
// //         println!("Cursor is inside the primary window, at {:?}", position);
// //     } else {
// //         println!("Cursor is not in the game window.");
// //     }
// // }

// // fn setup(
// //     mut commands: Commands,
// //     mut meshes: ResMut<Assets<Mesh>>,
// //     mut materials: ResMut<Assets<ColorMaterial>>,
// // ) {
// //     commands.spawn(Camera2dBundle::default());

// //     // Circle
// //     commands.spawn(MaterialMesh2dBundle {
// //         mesh: meshes.add(shape::Circle::new(50.).into()).into(),
// //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
// //         transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
// //         ..default()
// //     });

// //     // Rectangle
// //     commands.spawn(SpriteBundle {
// //         sprite: Sprite {
// //             color: Color::rgb(0.25, 0.25, 0.75),
// //             custom_size: Some(Vec2::new(50.0, 100.0)),
// //             ..default()
// //         },
// //         transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
// //         ..default()
// //     });

// //     // Quad
// //     commands.spawn(MaterialMesh2dBundle {
// //         mesh: meshes
// //             .add(shape::Quad::new(Vec2::new(50., 100.)).into())
// //             .into(),
// //         material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
// //         transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
// //         ..default()
// //     });

// //     // Hexagon
// //     commands.spawn(MaterialMesh2dBundle {
// //         mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
// //         material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
// //         transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
// //         ..default()
// //     });
// // }

// // pub struct HelloPlugin;

// // impl Plugin for HelloPlugin {
// //     fn build(&self, app: &mut App) {
// //         println!("hello world!");
// //     }
// // }

// // pub struct CastlesPlugin;

// // impl Plugin for CastlesPlugin {
// //     fn build(&self, app: &mut App) {
// //         app.add_systems(Startup, add_castles)
// //            .add_systems(Update, render_castles);
// //     }
// // }
