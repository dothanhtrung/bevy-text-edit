use bevy::prelude::*;

use bevy_text_edit::{TextEditable, TextEditPlugin};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
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
