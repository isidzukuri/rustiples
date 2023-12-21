#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;
    use bevy::prelude::*;
    use rustilples::fps::FpsPlugin;

    #[test]
    fn test_fps_display() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(InputPlugin::default());
        app.add_plugins(FpsPlugin);
        app.update();

        let mut query = app.world.query::<&Text>();
        let texts = query.iter(&app.world);

        assert_eq!(texts.len(), 1);

        let fps_text = query.iter(&app.world).next().unwrap();

        assert_eq!(fps_text.sections[0].value, "FPS: ".to_string());
        assert_eq!(fps_text.sections[1].value, " N/A".to_string());
    }
}
