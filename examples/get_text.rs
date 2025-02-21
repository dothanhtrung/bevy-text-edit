use bevy::prelude::*;

use bevy_text_edit::{TextEditFocus, TextEditPlugin, TextEditable, TextEdited};

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
            parent
                .spawn((
                    TextEditable::default(), // Mark text is editable
                    TextEditFocus,           // Mark text is focused
                    Interaction::None,       // Mark entity is interactable
                    Text::new("Input Text 1"),
                    Node {
                        height: Val::Px(64.),
                        width: Val::Percent(80.),
                        ..default()
                    },
                ))
                .observe(get_result);

            parent
                .spawn((
                    TextEditable{
                        placeholder: String::from("Input your text here"),
                        blink: true,
                        ..default()
                    },
                    Interaction::None,
                    Text::new("Input Text 2"),
                    Node {
                        height: Val::Px(64.),
                        width: Val::Percent(80.),
                        ..default()
                    },
                ))
                .observe(get_result);

            parent.spawn((Result, Text::new("")));
        });
}

fn get_result(
    trigger: Trigger<TextEdited>,
    mut result_box: Query<&mut Text, (With<Result>, Without<TextEditable>)>,
    // mut event: EventReader<TextEdited>,
) {
    let e = trigger.entity();
    let text = trigger.text.as_str();
    if let Ok(mut result_box) = result_box.get_single_mut() {
        **result_box = format!("Entity {}: {}", e, text);
    }
}
