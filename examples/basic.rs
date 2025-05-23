use bevy::color::palettes::tailwind::ZINC_800;
use bevy::prelude::*;
use bevy_text_edit::{TextEditConfig, TextEditFocus, TextEditPlugin, TextEditable, TextEdited};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

#[derive(Component)]
struct Result;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Since bevy 0.14, plugin needs to be added before init_state
        .init_state::<GameState>()
        // Add the plugin with state or `TextEditPluginAnyState::any()`
        .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut config: ResMut<TextEditConfig>) {
    commands.spawn(Camera2d::default());

    // There is a built-in virtual keyboard. You can enable it if needed.
    config.enable_virtual_keyboard = true;

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Percent(10.)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    // Mark text is editable
                    TextEditable {
                        placeholder: String::from("Input your text here"),
                        max_length: 255,
                        ..default()
                    },
                    TextEditFocus, // Mark text is focused
                    Node {
                        height: Val::Px(64.),
                        width: Val::Percent(80.),
                        margin: UiRect::bottom(Val::Px(10.)),
                        ..default()
                    },
                    BackgroundColor::from(ZINC_800),
                ))
                .observe(get_result);

            parent
                .spawn((
                    TextEditable {
                        placeholder: String::from("Input your text here"),
                        filter_in: vec!["[0-9]".to_string()], // Only accept number
                        ..default()
                    },
                    Node {
                        height: Val::Px(64.),
                        width: Val::Percent(80.),
                        ..default()
                    },
                    BackgroundColor::from(ZINC_800),
                ))
                .observe(get_result);

            parent.spawn((Result, Text::new("")));
        });
}

fn get_result(trigger: Trigger<TextEdited>, mut result_box: Query<&mut Text, (With<Result>, Without<TextEditable>)>) {
    let text = trigger.text.as_str();
    if let Ok(mut result_box) = result_box.single_mut() {
        **result_box = format!("Just input: {}", text);
    }
}
