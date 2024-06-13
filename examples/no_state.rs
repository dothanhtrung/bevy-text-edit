use bevy::prelude::*;

use bevy_text_edit::{TextEditable, TextEditFocus, TextEditPluginNoState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(TextEditPluginNoState)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextEditable,      // Mark text is editable
                Interaction::None, // Mark entity is interactable
                TextBundle::from_section(
                    "Input Text 1",
                    TextStyle {
                        font_size: 30.,
                        ..default()
                    },
                ),
            ));

            parent.spawn((
                TextEditable,
                TextEditFocus, // Focus edit text on this entity
                Interaction::None,
                TextBundle::from_section(
                    "Input Text 2",
                    TextStyle {
                        font_size: 30.,
                        ..default()
                    },
                ),
            ));
        });
}
