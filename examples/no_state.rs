use bevy::color::palettes::tailwind::ZINC_800;
use bevy::prelude::*;

use bevy_text_edit::{TextEditFocus, TextEditPluginNoState, TextEditable};

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
                Text::new("Input Text 1"),
                Node {
                    height: Val::Px(64.),
                    width: Val::Percent(80.),
                    margin: UiRect::bottom(Val::Px(10.)),
                    ..default()
                },
                BackgroundColor::from(ZINC_800),
            ));

            parent.spawn((
                TextEditable::default(),
                Text::new("Input Text 2"),
                Node {
                    height: Val::Px(64.),
                    width: Val::Percent(80.),
                    ..default()
                },
                BackgroundColor::from(ZINC_800),
            ));
        });
}
