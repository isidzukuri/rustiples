use crate::buttons::*;
use bevy::prelude::*;

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

pub fn button_pressed_event_listener(mut listener: EventReader<ButtonPressedEvent>) {
    for event in listener.read() {
        if event.event_type == "export_grid".to_string() {
            println!("Grid entity exported to");
        }
    }
}
