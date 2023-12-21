use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::Cursor;

pub fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/cursor.png"),
            ..default()
        },
        Cursor {},
    ));
}

pub fn move_cursor(
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if let Ok(mut transform) = cursor_query.get_single_mut() {
        if windows.is_empty() {
            return;
        };

        let window = windows.single();
        let (camera, camera_transform) = camera.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            transform.translation = Vec3::new(world_position.x + 7.5, world_position.y - 7.5, 0.0)
        }
    }
}
