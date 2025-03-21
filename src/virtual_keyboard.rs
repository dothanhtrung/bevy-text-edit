use bevy::app::{App, Plugin, Startup};
use bevy::hierarchy::ChildBuild;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::{
    on_event, AlignContent, AlignSelf, BorderColor, BuildChildren, ChildBuilder, Click, Color, Commands, Component,
    Entity, Event, EventReader, EventWriter, Handle, Image, ImageNode, Interaction, IntoSystemConfigs, JustifyItems,
    KeyCode, Node, Pointer, Query, Res, Resource, Single, Text, TextColor, TextFont, Trigger, Update, Visibility, With,
    ZIndex,
};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, JustifySelf, UiRect, Val};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_support_misc::ui::button::{ButtonColorEffect, ButtonTransformEffect, GameButtonPlugin};

const KEY_1U: f32 = 6.;
const KEY_MARGIN: Val = Val::Percent(0.5);

pub struct VirtualKeyboardPlugin;

impl Plugin for VirtualKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameButtonPlugin)
            .insert_resource(VirtualKeyboardTheme::new())
            .add_event::<ShowVirtualKeyboard>()
            .add_systems(Startup, setup_virtual_keyboard)
            .add_systems(Update, show_keyboard.run_if(on_event::<ShowVirtualKeyboard>));
    }
}

#[derive(Resource, Default)]
pub struct VirtualKeyboardTheme {
    pub bg_color: Color,
    pub bg_image: Option<Handle<Image>>,
    pub button_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub text_font: TextFont,
}

impl VirtualKeyboardTheme {
    fn new() -> Self {
        Self {
            bg_color: Color::NONE,
            button_color: Color::NONE,
            text_color: Color::WHITE,
            ..Self::default()
        }
    }
}

#[derive(Component, Default)]
#[require(Node, Interaction)]
pub struct VirtualKeyboard {
    show_alt: bool,
}

#[derive(Component)]
#[require(Interaction)]
pub struct VirtualKey {
    label: (String, String),
    key_code: KeyCode,
    logical_key: (Key, Key),
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
        let mut cmd = if let Some(image) = theme.bg_image.clone() {
            commands.spawn(ImageNode { image, ..default() })
        } else {
            commands.spawn_empty()
        };

        cmd.insert((
            VirtualKeyboard::default(),
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
                        (("`", "~"), KeyCode::Backquote, (Key::Character("`".into()), Key::Character("~".into())), 1.),
                        (("1", "!"), KeyCode::Digit1, (Key::Character("1".into()), Key::Character("!".into())), 1.),
                        (("2", "@"), KeyCode::Digit2, (Key::Character("2".into()), Key::Character("@".into())), 1.),
                        (("3", "#"), KeyCode::Digit3, (Key::Character("3".into()), Key::Character("#".into())), 1.),
                        (("4", "$"), KeyCode::Digit4, (Key::Character("4".into()), Key::Character("$".into())), 1.),
                        (("5", "%"), KeyCode::Digit5, (Key::Character("5".into()), Key::Character("%".into())), 1.),
                        (("6", "^"), KeyCode::Digit6, (Key::Character("6".into()), Key::Character("^".into())), 1.),
                        (("7", "&"), KeyCode::Digit7, (Key::Character("7".into()), Key::Character("&".into())), 1.),
                        (("8", "*"), KeyCode::Digit8, (Key::Character("8".into()), Key::Character("*".into())), 1.),
                        (("9", "("), KeyCode::Digit9, (Key::Character("9".into()), Key::Character("(".into())), 1.),
                        (("0", ")"), KeyCode::Digit0, (Key::Character("0".into()), Key::Character(")".into())), 1.),
                        (("-", "_"), KeyCode::Minus, (Key::Character("-".into()), Key::Character("_".into())), 1.),
                        (("=", "+"), KeyCode::Equal, (Key::Character("=".into()), Key::Character("+".into())), 1.),
                        (("Backspace", ""), KeyCode::Backspace, (Key::Backspace, Key::Backspace), 2.),
                    ],
                    vec![
                        (("q", "Q"), KeyCode::KeyQ, (Key::Character("q".into()), Key::Character("Q".into())), 1.),
                        (("w", "W"), KeyCode::KeyW, (Key::Character("w".into()), Key::Character("W".into())), 1.),
                        (("e", "E"), KeyCode::KeyE, (Key::Character("e".into()), Key::Character("E".into())), 1.),
                        (("r", "R"), KeyCode::KeyR, (Key::Character("r".into()), Key::Character("R".into())), 1.),
                        (("t", "T"), KeyCode::KeyT, (Key::Character("t".into()), Key::Character("T".into())), 1.),
                        (("y", "Y"), KeyCode::KeyY, (Key::Character("y".into()), Key::Character("Y".into())), 1.),
                        (("u", "U"), KeyCode::KeyU, (Key::Character("u".into()), Key::Character("U".into())), 1.),
                        (("i", "I"), KeyCode::KeyI, (Key::Character("i".into()), Key::Character("I".into())), 1.),
                        (("o", "O"), KeyCode::KeyO, (Key::Character("o".into()), Key::Character("O".into())), 1.),
                        (("p", "P"), KeyCode::KeyP, (Key::Character("p".into()), Key::Character("P".into())), 1.),
                        (("[", "{"), KeyCode::BracketLeft, (Key::Character("[".into()), Key::Character("{".into())), 1.),
                        (("]", "}"), KeyCode::BracketRight, (Key::Character("]".into()), Key::Character("}".into())), 1.),
                        (("\\", "|"), KeyCode::Backslash, (Key::Character("\\".into()), Key::Character("|".into())), 1.25),
                    ],
                    vec![
                        (("Shift", "SHIFT"), KeyCode::ShiftLeft, (Key::Shift, Key::Shift), 1.5),
                        (("a", "A"), KeyCode::KeyA, (Key::Character("a".into()), Key::Character("A".into())), 1.),
                        (("s", "S"), KeyCode::KeyS, (Key::Character("s".into()), Key::Character("S".into())), 1.),
                        (("d", "D"), KeyCode::KeyD, (Key::Character("d".into()), Key::Character("D".into())), 1.),
                        (("f", "F"), KeyCode::KeyF, (Key::Character("f".into()), Key::Character("F".into())), 1.),
                        (("g", "G"), KeyCode::KeyG, (Key::Character("g".into()), Key::Character("G".into())), 1.),
                        (("h", "H"), KeyCode::KeyH, (Key::Character("h".into()), Key::Character("H".into())), 1.),
                        (("j", "J"), KeyCode::KeyJ, (Key::Character("j".into()), Key::Character("J".into())), 1.),
                        (("k", "K"), KeyCode::KeyK, (Key::Character("k".into()), Key::Character("K".into())), 1.),
                        (("l", "L"), KeyCode::KeyL, (Key::Character("l".into()), Key::Character("L".into())), 1.),
                        ((";", ":"), KeyCode::Semicolon, (Key::Character(";".into()), Key::Character(":".into())), 1.),
                        (("'", "\""), KeyCode::Quote, (Key::Character("'".into()), Key::Character("\"".into())), 1.),
                        (("Del", "DEL"), KeyCode::Delete, (Key::Delete, Key::Delete), 1.5),
                    ],
                    vec![
                        (("z", "Z"), KeyCode::KeyZ, (Key::Character("z".into()), Key::Character("Z".into())), 1.),
                        (("x", "X"), KeyCode::KeyX, (Key::Character("x".into()), Key::Character("X".into())), 1.),
                        (("c", "C"), KeyCode::KeyC, (Key::Character("c".into()), Key::Character("C".into())), 1.),
                        (("v", "V"), KeyCode::KeyV, (Key::Character("v".into()), Key::Character("V".into())), 1.),
                        (("Space", "SPACE"), KeyCode::Space, (Key::Space, Key::Space), 2.5),
                        (("b", "B"), KeyCode::KeyB, (Key::Character("b".into()), Key::Character("B".into())), 1.),
                        (("n", "N"), KeyCode::KeyN, (Key::Character("n".into()), Key::Character("N".into())), 1.),
                        (("m", "M"), KeyCode::KeyM, (Key::Character("m".into()), Key::Character("M".into())), 1.),
                        ((",", "<"), KeyCode::Comma, (Key::Character(",".into()), Key::Character("<".into())), 1.),
                        ((".", ">"), KeyCode::Period, (Key::Character(".".into()), Key::Character(">".into())), 1.),
                        (("/", "?"), KeyCode::Slash, (Key::Character("/".into()), Key::Character("?".into())), 1.),
                        (("<=", "<="), KeyCode::ArrowLeft, (Key::ArrowLeft, Key::ArrowLeft), 1.),
                        (("=>", "=>"), KeyCode::ArrowRight, (Key::ArrowRight, Key::ArrowRight), 1.),
                    ],
                ];
                for row in keys {
                    builder
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            height: Val::Percent(15.),
                            margin: UiRect::top(Val::Percent(2.)),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_children(|builder| {
                            for (key_str, keycode, logical_key, key_size) in row {
                                spawn_key(builder, key_str, keycode, logical_key, key_size * KEY_1U, &theme);
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
    label: (&str, &str),
    key_code: KeyCode,
    logical_key: (Key, Key),
    width_percent: f32,
    theme: &VirtualKeyboardTheme,
) {
    let text = label.0;
    builder
        .spawn((
            VirtualKey {
                label: (label.0.to_string(), label.1.to_string()),
                key_code,
                logical_key,
            },
            ButtonTransformEffect::default(),
            ButtonColorEffect::default(),
            Node {
                width: Val::Percent(width_percent),
                margin: UiRect::horizontal(KEY_MARGIN),
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor::from(theme.border_color),
            BackgroundColor::from(theme.button_color),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                theme.text_font.clone(),
                TextColor::from(theme.text_color),
            ));
        })
        .observe(send_key);
}

fn send_key(
    trigger: Trigger<Pointer<Click>>,
    keys: Query<&VirtualKey>,
    mut event: EventWriter<KeyboardInput>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut virtual_keyboard: Single<&mut VirtualKeyboard>,
) {
    if let Ok(window) = windows.get_single() {
        if let Ok(key) = keys.get(trigger.entity()) {
            if key.logical_key.0 == Key::Shift {
                virtual_keyboard.show_alt = !virtual_keyboard.show_alt;
            } else {
                let logical_key = if virtual_keyboard.show_alt {
                    key.logical_key.1.clone()
                } else {
                    key.logical_key.0.clone()
                };
                event.send(KeyboardInput {
                    key_code: key.key_code,
                    logical_key,
                    state: ButtonState::Pressed,
                    repeat: false,
                    window,
                });
            }
        }
    }
}
