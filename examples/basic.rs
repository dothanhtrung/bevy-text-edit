// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::prelude::*;

use bevy_text_edit::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(EditTextPlugin)
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
                // Add the component
                TextEditable,
                Interaction::None,
                TextBundle::from_section(
                    "Input Text 1",
                    TextStyle {
                        font_size: 20.,
                        ..default()
                    },
                ),
            ));

            parent.spawn((
                TextEditable,
                TextEditFocus,
                Interaction::None,
                TextBundle::from_section(
                    "Input Text 2",
                    TextStyle {
                        font_size: 20.,
                        ..default()
                    },
                ),
            ));
        });
}
