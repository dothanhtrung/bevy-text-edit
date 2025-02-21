use bevy::color::palettes::tailwind::ZINC_800;
use bevy::prelude::*;

use bevy_text_edit::{TextEditFocus, TextEditPlugin, TextEditable};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Since bevy 0.14, default plugin need to be added before init_state
        .init_state::<GameState>()
        // Add the plugin
        .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
        .add_systems(OnEnter(GameState::Menu), setup)
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
                Text::new("Section 1"),
                Node {
                    height: Val::Px(64.),
                    width: Val::Percent(80.),
                    ..default()
                },
                BackgroundColor::from(ZINC_800),
            ));

            parent.spawn((
                TextEditable {
                    filter_in: vec!["[0-9]".into(), " ".into()], // Only allow number and space
                    blink: true,
                    placeholder: String::from("Section 2"),
                    max_length: 255,
                    ..default()
                },
                Text::new(""),
                Node {
                    height: Val::Px(64.),
                    width: Val::Percent(80.),
                    ..default()
                },
                BackgroundColor::from(ZINC_800),
            ));
        });
}
