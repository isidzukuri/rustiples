use super::{ButtonPressedEvent, ClickableButton, Menu};
use bevy::prelude::*;

pub fn spawn_menu(mut commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Menu {},
            NodeBundle {
                // background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    top: Val::Percent(5.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(2.0)),
                    // width: Val::Px(150.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id()
}

pub fn spawn_button(
    label: String,
    event_type: String,
    mut commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    mut query: &mut Query<Entity, With<Menu>>,
) {
    let mut menu = match query.get_single_mut() {
        Ok(entity) => entity,
        _error => spawn_menu(&mut commands),
    };

    let button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            ClickableButton {
                label: label.clone(),
                event_type: event_type,
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ));
        })
        .id();

    commands.entity(menu).push_children(&[button]);
}

pub fn event_publisher(
    mut interaction_query: Query<
        (&Interaction, &ClickableButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<ButtonPressedEvent>,
) {
    for (interaction, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                event_writer.send(ButtonPressedEvent {
                    event_type: button.event_type.clone(),
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
