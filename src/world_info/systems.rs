use bevy::prelude::*;

use super::{WorldInfoItem, WorldInfoRoot, WorldInfoText};

pub fn print_window_size(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();

    print_world_info(commands, format!("{}x{}", window.width(), window.height()));
}

pub fn print_world_info(mut commands: Commands, text: String){
    commands.spawn(WorldInfoItem {
        val: text
    });
}

pub fn setup_world_info(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            WorldInfoRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    left: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    right: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text = commands
        .spawn((
            WorldInfoText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "World Info:\n".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: "".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text]);
}

pub fn world_info_text_update_system(
    info_items: Query<&WorldInfoItem>,
    mut query: Query<&mut Text, With<WorldInfoText>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = info_items
        .iter()
        .map(|item| item.val.clone())
        .collect::<Vec<_>>()
        .join("\n");
}

/// Toggle the FPS counter when pressing F12
pub fn world_info_showhide(
    mut q: Query<&mut Visibility, With<WorldInfoRoot>>,
    kbd: Res<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
