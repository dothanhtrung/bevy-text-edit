use bevy::{color::palettes::css::DARK_GRAY, prelude::*};

use bevy_text_edit::{listen_changing_focus, TextEditFocus, TextEditPlugin, TextEditable};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

#[derive(Component)]
struct Result;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Since bevy 0.14, plugin need to be added before init_state
        .init_state::<GameState>()
        // Add the plugin
        .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
        .add_systems(Startup, setup)
        // Get text after focus change to ensure text cursor is removed
        .add_systems(Update, get_result.after(listen_changing_focus))
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
            let text_style = TextStyle {
                font_size: 30.,
                ..default()
            };

            parent.spawn((
                TextEditable,      // Mark text is editable
                TextEditFocus,     // Mark text is focused
                Interaction::None, // Mark entity is interactable
                TextBundle::from_section("Input Text 1", text_style.clone()),
            ));

            parent.spawn((
                TextEditable,
                Interaction::None,
                TextBundle::from_section("Input Text 2", text_style.clone()),
            ));

            parent
                .spawn(ButtonBundle {
                    background_color: DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Result".to_string(), text_style.clone()));
                });

            parent.spawn((Result, TextBundle::from_section("", text_style.clone())));
        });
}

fn get_result(
    texts: Query<&Text, With<TextEditable>>,
    interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut result_box: Query<&mut Text, (With<Result>, Without<TextEditable>)>,
) {
    if let Ok(interaction) = interaction.get_single() {
        if *interaction == Interaction::Pressed {
            let mut result = String::new();

            for text in texts.iter() {
                result = format!("{} {}", result, text.sections[0].value);
            }

            if let Ok(mut result_box) = result_box.get_single_mut() {
                result_box.sections[0].value = result;
            }
        }
    }
}
