use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;
use systems::*;

pub struct GameGridPlugin;

impl Plugin for GameGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_grid, spawn_control_buttons))
            .add_systems(Update, (grid_click, button_pressed_event_listener));
    }
}




use crate::buttons::Menu;
use crate::buttons::spawn_button;
use crate::buttons::ButtonPressedEvent;

pub fn spawn_control_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<Entity, With<Menu>>,
) {
    spawn_button(
        "Save map".to_string(),
        "export_grid".to_string(),
        commands,
        asset_server,
        query,
    );
}


use std::fs::File;
use std::fs::create_dir_all;
use std::io::prelude::*;
use crate::game_grid::grid::Grid;

    
pub fn button_pressed_event_listener(mut listener: EventReader<ButtonPressedEvent>, grid: Res<Grid>,) {
    for event in listener.read() {
        if event.event_type == "export_grid".to_string() {
            export_grid_to_file(grid.as_ref());
        }
    }
}

pub fn export_grid_to_file(grid: &Grid){
    println!("Grid export is starter");

    match serde_json::to_string(grid) {
        Err(error) => println!("Export grid has failed: {:?}", error),
        Ok(serialized) => {
            match create_dir_all("grids/") {
                Err(error) => panic!("Storage folder cannot be created: {:?}", error),
                Ok(_) => {
                    let mut file = File::create("grids/save.json").unwrap();
                    file.write_all(serialized.as_bytes());
                    println!("Grid export is compleated");
                },
            }
        }
    }
}