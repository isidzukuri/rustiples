use bevy::prelude::*;
use bevy::window::WindowResolution;

use rustilples::app_state::AppState;
use rustilples::buttons::ButtonsPlugin;
use rustilples::camera::CameraPlugin;
use rustilples::cursor::CursorPlugin;
use rustilples::fps::FpsPlugin;
use rustilples::game_grid::GameGridPlugin;
use rustilples::main_menu::MainMenuPlugin;
pub use rustilples::world_info::print_world_info;
use rustilples::world_info::WorldInfoPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1200., 1000.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugins(CameraPlugin)
        .add_plugins(FpsPlugin)
        .add_plugins(WorldInfoPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(GameGridPlugin)
        .add_plugins(ButtonsPlugin)
        .add_plugins(MainMenuPlugin)
        .run();
}
