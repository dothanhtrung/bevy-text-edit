use bevy::color::palettes::basic::{GREEN, LIME, RED, YELLOW};
use bevy::prelude::*;

use bevy_text_edit::{TextEditable, TextEditFocus, TextEditPlugin};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Since bevy 0.14, plugin need to be added before init_state
        .init_state::<GameState>()
        // Add the plugin
        .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
        .add_systems(OnEnter(GameState::Menu), setup)
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
            let section1 = TextSection {
                value: "Section1".into(),
                style: TextStyle {
                    color: GREEN.into(),
                    ..default()
                },
            };
            let section2 = TextSection {
                value: "Section2".into(),
                style: TextStyle {
                    color: RED.into(),
                    ..default()
                },
            };
            let section3 = TextSection {
                value: "1234".into(),
                style: TextStyle {
                    color: LIME.into(),
                    ..default()
                },
            };
            let section4 = TextSection {
                value: "5678".into(),
                style: TextStyle {
                    color: YELLOW.into(),
                    ..default()
                },
            };

            parent.spawn((
                TextEditable::default(), // Mark text is editable
                TextEditFocus,           // Mark text is focused
                Interaction::None,       // Mark entity is interactable
                TextBundle::from_sections(vec![section1, section2]),
            ));

            parent.spawn((
                TextEditable {
                    filter_in: vec!["[0-9]".into(), " ".into()], // Only allow number and space
                    blink: true,
                    ..default()
                },
                Interaction::None,
                TextBundle::from_sections(vec![section3, section4]),
            ));
        });
}
