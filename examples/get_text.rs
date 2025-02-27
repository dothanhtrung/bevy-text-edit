use bevy::color::palettes::tailwind::ZINC_800;
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
                    TextEditable {
                        // Mark text is editable
                        placeholder: String::from("Input your text here"),
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
                        blink: true,
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
    let e = trigger.entity();
    let text = trigger.text.as_str();
    if let Ok(mut result_box) = result_box.get_single_mut() {
        **result_box = format!("Entity {}: {}", e, text);
    }
}
