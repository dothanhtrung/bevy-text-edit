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
    commands.spawn(Camera2d::default());

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextEditable::default(), // Mark text is editable
                TextEditFocus,           // Mark text is focused
                Interaction::None,       // Mark entity is interactable
                Text::new("Input Text 1"),
            ));

            parent.spawn((TextEditable::default(), Interaction::None, Text::new("Input Text 2")));
        });
}
