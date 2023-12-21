#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use rustilples::cursor::CursorPlugin;

    #[test]
    fn test_spawn_cursor() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(ImagePlugin::default());
        app.add_plugins(CursorPlugin);
        app.update();

        let mut query = app.world.query::<&Transform>();

        assert_eq!(query.get_single(&app.world).is_ok(), true);
        assert_eq!(
            query.get_single(&app.world).unwrap().translation,
            Vec3::default()
        );
    }
}
