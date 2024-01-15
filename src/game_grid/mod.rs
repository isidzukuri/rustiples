use bevy::prelude::*;
use bevy::utils::hashbrown::Equivalent;
use bevy::window::PrimaryWindow;

mod components;
mod systems;

use crate::app_state::AppState;
pub use components::*;
use systems::*;

pub struct GameGridPlugin;

impl Plugin for GameGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::NewGame), start_game)
            .add_systems(OnEnter(AppState::LoadGame), start_game)
            .add_systems(OnEnter(AppState::InGrid), (controls::spawn_control_buttons))
            .add_systems(
                Update,
                (grid_click, controls::button_pressed_event_listener)
                    .run_if(in_state(AppState::InGrid)),
            );
    }
}

pub fn start_game(
    mut app_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if current_state.get() == &AppState::LoadGame {
        load_grid(&mut commands, &window_query, &asset_server);
    } else {
        generate_grid(&mut commands, &window_query, &asset_server);
    }
    app_state.set(AppState::InGrid);
}
