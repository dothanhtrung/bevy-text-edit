use bevy::app::{App, Plugin, Startup};
use bevy::hierarchy::ChildBuild;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::{
    on_event, AlignContent, AlignSelf, BuildChildren, ChildBuilder, Click, Color, Commands, Component, Entity, Event,
    EventReader, EventWriter, Interaction, IntoSystemConfigs, JustifyItems, KeyCode, Node, Pointer, Query, Res,
    Resource, Text, TextColor, TextFont, Trigger, Update, Visibility, With, ZIndex,
};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, JustifySelf, UiRect, Val};
use bevy::utils::default;
use bevy::window::PrimaryWindow;

const KEY_1U: f32 = 5.5;
const KEY_MARGIN: Val = Val::Percent(1.);

pub struct VirtualKeyboardPlugin;

impl Plugin for VirtualKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VirtualKeyboardTheme::default())
            .add_event::<ShowVirtualKeyboard>()
            .add_systems(Startup, setup_virtual_keyboard)
            .add_systems(Update, show_keyboard.run_if(on_event::<ShowVirtualKeyboard>));
    }
}

#[derive(Resource)]
pub struct VirtualKeyboardTheme {
    pub bg_color: Color,
    pub button_color: Color,
    pub text_color: Color,
    pub text_font: TextFont,
}

impl Default for VirtualKeyboardTheme {
    fn default() -> Self {
        Self {
            bg_color: Color::NONE,
            button_color: Color::BLACK,
            text_color: Color::WHITE,
            text_font: TextFont::default(),
        }
    }
}

#[derive(Component, Default)]
#[require(Node, Interaction)]
pub struct VirtualKeyboard;

#[derive(Component)]
#[require(Interaction)]
pub struct VirtualKey {
    key_code: KeyCode,
    logical_key: Key,
}

/// Show virtual keyboard event:
/// * true: show
/// * false: hide
#[derive(Event)]
pub struct ShowVirtualKeyboard(pub bool);

fn setup_virtual_keyboard(
    mut commands: Commands,
    theme: Res<VirtualKeyboardTheme>,
    query: Query<Entity, With<VirtualKeyboard>>,
) {
    if query.is_empty() {
        commands
            .spawn((
                VirtualKeyboard,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(98.),
                    height: Val::Percent(40.),
                    align_self: AlignSelf::End,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                BackgroundColor(theme.bg_color),
                ZIndex(i32::MAX),
                Visibility::Hidden,
            ))
            .with_children(|builder| {
                let keys = vec![
                    vec![
                        ("`", KeyCode::Backquote, Key::Character("`".into()), 1.),
                        ("1", KeyCode::Digit1, Key::Character("1".into()), 1.),
                        ("2", KeyCode::Digit2, Key::Character("2".into()), 1.),
                        ("3", KeyCode::Digit3, Key::Character("3".into()), 1.),
                        ("4", KeyCode::Digit4, Key::Character("4".into()), 1.),
                        ("5", KeyCode::Digit5, Key::Character("5".into()), 1.),
                        ("6", KeyCode::Digit6, Key::Character("6".into()), 1.),
                        ("7", KeyCode::Digit7, Key::Character("7".into()), 1.),
                        ("8", KeyCode::Digit8, Key::Character("8".into()), 1.),
                        ("9", KeyCode::Digit9, Key::Character("9".into()), 1.),
                        ("0", KeyCode::Digit0, Key::Character("0".into()), 1.),
                        ("-", KeyCode::Minus, Key::Character("-".into()), 1.),
                        ("=", KeyCode::Equal, Key::Character("=".into()), 1.),
                        ("Backspace", KeyCode::Backspace, Key::Backspace, 2.),
                    ],
                    vec![
                        ("q", KeyCode::KeyQ, Key::Character("q".into()), 1.),
                        ("w", KeyCode::KeyW, Key::Character("w".into()), 1.),
                        ("e", KeyCode::KeyE, Key::Character("e".into()), 1.),
                        ("r", KeyCode::KeyR, Key::Character("r".into()), 1.),
                        ("t", KeyCode::KeyT, Key::Character("t".into()), 1.),
                        ("y", KeyCode::KeyY, Key::Character("y".into()), 1.),
                        ("u", KeyCode::KeyU, Key::Character("u".into()), 1.),
                        ("i", KeyCode::KeyI, Key::Character("i".into()), 1.),
                        ("o", KeyCode::KeyO, Key::Character("o".into()), 1.),
                        ("p", KeyCode::KeyP, Key::Character("p".into()), 1.),
                        ("[", KeyCode::BracketLeft, Key::Character("[".into()), 1.),
                        ("]", KeyCode::BracketRight, Key::Character("]".into()), 1.),
                        ("\\", KeyCode::Backslash, Key::Character("\\".into()), 1.25),
                    ],
                    vec![
                        ("a", KeyCode::KeyA, Key::Character("a".into()), 1.),
                        ("s", KeyCode::KeyS, Key::Character("s".into()), 1.),
                        ("d", KeyCode::KeyD, Key::Character("d".into()), 1.),
                        ("f", KeyCode::KeyF, Key::Character("f".into()), 1.),
                        ("g", KeyCode::KeyG, Key::Character("g".into()), 1.),
                        ("h", KeyCode::KeyH, Key::Character("h".into()), 1.),
                        ("j", KeyCode::KeyJ, Key::Character("j".into()), 1.),
                        ("k", KeyCode::KeyK, Key::Character("k".into()), 1.),
                        ("l", KeyCode::KeyL, Key::Character("l".into()), 1.),
                        (";", KeyCode::Semicolon, Key::Character(";".into()), 1.),
                        ("'", KeyCode::Quote, Key::Character("'".into()), 1.),
                        // ("Enter", KeyCode::Enter, Key::Enter, 2.5),
                    ],
                    vec![
                        ("Shift", KeyCode::ShiftLeft, Key::Shift, 1.5),
                        ("z", KeyCode::KeyZ, Key::Character("z".into()), 1.),
                        ("x", KeyCode::KeyX, Key::Character("x".into()), 1.),
                        ("c", KeyCode::KeyC, Key::Character("c".into()), 1.),
                        ("v", KeyCode::KeyV, Key::Character("v".into()), 1.),
                        ("Space", KeyCode::Space, Key::Space, 2.5),
                        ("b", KeyCode::KeyB, Key::Character("b".into()), 1.),
                        ("n", KeyCode::KeyN, Key::Character("n".into()), 1.),
                        ("m", KeyCode::KeyM, Key::Character("m".into()), 1.),
                        (",", KeyCode::Comma, Key::Character(",".into()), 1.),
                        (".", KeyCode::Period, Key::Character(".".into()), 1.),
                        ("/", KeyCode::Slash, Key::Character("/".into()), 1.),
                        ("Del", KeyCode::Delete, Key::Delete, 1.5),
                    ],
                ];
                for row in keys {
                    builder
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            height: Val::Percent(15.),
                            margin: UiRect::top(Val::Percent(2.)),
                            align_content: AlignContent::Center,
                            ..default()
                        })
                        .with_children(|builder| {
                            for (key_str, keycode, logical_key, key_size) in row {
                                spawn_key(
                                    builder,
                                    key_str,
                                    keycode,
                                    logical_key,
                                    key_size * KEY_1U,
                                    theme.bg_color,
                                    &theme.text_font,
                                    theme.text_color,
                                );
                            }
                        });
                }
            });
    }
}

fn show_keyboard(
    mut events: EventReader<ShowVirtualKeyboard>,
    mut query: Query<&mut Visibility, With<VirtualKeyboard>>,
) {
    for event in events.read() {
        if event.0 {
            for mut visibility in query.iter_mut() {
                *visibility = Visibility::Visible
            }
        } else {
            for mut visibility in query.iter_mut() {
                *visibility = Visibility::Hidden
            }
        }
    }
}

fn spawn_key(
    builder: &mut ChildBuilder,
    label: &str,
    key_code: KeyCode,
    logical_key: Key,
    width_percent: f32,
    button_color: Color,
    font: &TextFont,
    text_color: Color,
) {
    builder
        .spawn((
            VirtualKey { key_code, logical_key },
            Node {
                width: Val::Percent(width_percent),
                margin: UiRect::horizontal(KEY_MARGIN),
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            BackgroundColor::from(button_color),
        ))
        .with_children(|builder| {
            builder.spawn((Text::new(label), font.clone(), TextColor::from(text_color)));
        })
        .observe(send_key);
}

fn send_key(
    trigger: Trigger<Pointer<Click>>,
    query: Query<&VirtualKey>,
    mut event: EventWriter<KeyboardInput>,
    windows: Query<Entity, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.get_single() {
        if let Ok(key) = query.get(trigger.entity()) {
            event.send(KeyboardInput {
                key_code: key.key_code,
                logical_key: key.logical_key.clone(),
                state: ButtonState::Pressed,
                repeat: false,
                window,
            });
        }
    }
}
