use bevy::prelude::*;

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

            parent.spawn(Button).with_children(|parent| {
                parent.spawn(Text::new("Result"));
            });

            parent.spawn((Result, Text::new("")));
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
                result = format!("{} {}", result, **text);
            }

            if let Ok(mut result_box) = result_box.get_single_mut() {
                **result_box = result;
            }
        }
    }
}
