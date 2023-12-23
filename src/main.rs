use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResolution;

use rustilples::cursor::CursorPlugin;
use rustilples::fps::FpsPlugin;
use rustilples::game_grid::GameGridPlugin;
pub use rustilples::world_info::print_world_info;
use rustilples::world_info::WorldInfoPlugin;

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
        .add_plugins(GameGridPlugin)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
