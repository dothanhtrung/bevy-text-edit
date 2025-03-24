use bevy::app::{App, Plugin, Startup};
use bevy::hierarchy::ChildBuild;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::{
    on_event, AlignContent, AlignSelf, BorderColor, BuildChildren, ChildBuilder, Click, Color, Commands, Component,
    DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, Handle, Image, ImageNode, Interaction,
    IntoSystemConfigs, JustifyItems, KeyCode, Node, Pointer, Query, Res, Resource, Single, Text, TextColor, TextFont,
    Trigger, Update, Visibility, With, ZIndex,
};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, JustifySelf, UiRect, Val};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_support_misc::ui::button::{ButtonColorEffect, ButtonTransformEffect, GameButtonPlugin};

const KEY_1U: f32 = 6.5;
const KEY_MARGIN: Val = Val::Percent(0.5);

pub struct VirtualKeyboardPlugin;

impl Plugin for VirtualKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameButtonPlugin)
            .insert_resource(VirtualKeyboardTheme::new())
            .insert_resource(VirtualKeysList::default())
            .add_event::<VirtualKeyboardChanged>()
            .add_event::<ShowVirtualKeyboard>()
            .add_systems(Startup, spawn_virtual_keyboard)
            .add_systems(
                Update,
                (
                    show_keyboard.run_if(on_event::<ShowVirtualKeyboard>),
                    spawn_virtual_keyboard.run_if(on_event::<VirtualKeyboardChanged>),
                ),
            );
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
    pub key_size_1u: Val,
}

impl VirtualKeyboardTheme {
    fn new() -> Self {
        Self {
            bg_color: Color::NONE,
            button_color: Color::NONE,
            text_color: Color::WHITE,
            key_size_1u: Val::Percent(KEY_1U),
            ..Self::default()
        }
    }
}

#[derive(Event)]
pub struct VirtualKeyboardChanged;

#[derive(Component, Default)]
#[require(Node, Interaction)]
pub struct VirtualKeyboard {
    show_alt: bool,
}

#[derive(Resource)]
pub struct VirtualKeysList {
    pub keys: Vec<Vec<(VirtualKeyLabel, VirtualKey, f32)>>,
}

impl Default for VirtualKeysList {
    fn default() -> Self {
        Self {
            keys: vec![
                vec![
                    (
                        VirtualKeyLabel::new("`", "~"),
                        VirtualKey::new(
                            KeyCode::Backquote,
                            (Key::Character("`".into()), Key::Character("~".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("1", "!"),
                        VirtualKey::new(
                            KeyCode::Digit1,
                            (Key::Character("1".into()), Key::Character("!".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("2", "@"),
                        VirtualKey::new(
                            KeyCode::Digit2,
                            (Key::Character("2".into()), Key::Character("@".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("3", "#"),
                        VirtualKey::new(
                            KeyCode::Digit3,
                            (Key::Character("3".into()), Key::Character("#".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("4", "$"),
                        VirtualKey::new(
                            KeyCode::Digit4,
                            (Key::Character("4".into()), Key::Character("$".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("5", "%"),
                        VirtualKey::new(
                            KeyCode::Digit5,
                            (Key::Character("5".into()), Key::Character("%".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("6", "^"),
                        VirtualKey::new(
                            KeyCode::Digit6,
                            (Key::Character("6".into()), Key::Character("^".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("7", "&"),
                        VirtualKey::new(
                            KeyCode::Digit7,
                            (Key::Character("7".into()), Key::Character("&".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("8", "*"),
                        VirtualKey::new(
                            KeyCode::Digit8,
                            (Key::Character("8".into()), Key::Character("*".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("9", "("),
                        VirtualKey::new(
                            KeyCode::Digit9,
                            (Key::Character("9".into()), Key::Character("(".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("0", ")"),
                        VirtualKey::new(
                            KeyCode::Digit0,
                            (Key::Character("0".into()), Key::Character(")".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("-", "_"),
                        VirtualKey::new(KeyCode::Minus, (Key::Character("-".into()), Key::Character("_".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("=", "+"),
                        VirtualKey::new(KeyCode::Equal, (Key::Character("=".into()), Key::Character("+".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("Backspace", "BACKSPACE"),
                        VirtualKey::new(KeyCode::Backspace, (Key::Backspace, Key::Backspace)),
                        2.,
                    ),
                ],
                vec![
                    (
                        VirtualKeyLabel::new("q", "Q"),
                        VirtualKey::new(KeyCode::KeyQ, (Key::Character("q".into()), Key::Character("Q".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("w", "W"),
                        VirtualKey::new(KeyCode::KeyW, (Key::Character("w".into()), Key::Character("W".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("e", "E"),
                        VirtualKey::new(KeyCode::KeyE, (Key::Character("e".into()), Key::Character("E".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("r", "R"),
                        VirtualKey::new(KeyCode::KeyR, (Key::Character("r".into()), Key::Character("R".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("t", "T"),
                        VirtualKey::new(KeyCode::KeyT, (Key::Character("t".into()), Key::Character("T".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("y", "Y"),
                        VirtualKey::new(KeyCode::KeyY, (Key::Character("y".into()), Key::Character("Y".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("u", "U"),
                        VirtualKey::new(KeyCode::KeyU, (Key::Character("u".into()), Key::Character("U".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("i", "I"),
                        VirtualKey::new(KeyCode::KeyI, (Key::Character("i".into()), Key::Character("I".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("o", "O"),
                        VirtualKey::new(KeyCode::KeyO, (Key::Character("o".into()), Key::Character("O".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("p", "P"),
                        VirtualKey::new(KeyCode::KeyP, (Key::Character("p".into()), Key::Character("P".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("[", "{"),
                        VirtualKey::new(
                            KeyCode::BracketLeft,
                            (Key::Character("[".into()), Key::Character("{".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("]", "}"),
                        VirtualKey::new(
                            KeyCode::BracketRight,
                            (Key::Character("]".into()), Key::Character("}".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("\\", "|"),
                        VirtualKey::new(
                            KeyCode::Backslash,
                            (Key::Character("\\".into()), Key::Character("|".into())),
                        ),
                        1.25,
                    ),
                ],
                vec![
                    (
                        VirtualKeyLabel::new("Shift", "SHIFT"),
                        VirtualKey::new(KeyCode::ShiftLeft, (Key::Shift, Key::Shift)),
                        1.5,
                    ),
                    (
                        VirtualKeyLabel::new("a", "A"),
                        VirtualKey::new(KeyCode::KeyA, (Key::Character("a".into()), Key::Character("A".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("s", "S"),
                        VirtualKey::new(KeyCode::KeyS, (Key::Character("s".into()), Key::Character("S".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("d", "D"),
                        VirtualKey::new(KeyCode::KeyD, (Key::Character("d".into()), Key::Character("D".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("f", "F"),
                        VirtualKey::new(KeyCode::KeyF, (Key::Character("f".into()), Key::Character("F".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("g", "G"),
                        VirtualKey::new(KeyCode::KeyG, (Key::Character("g".into()), Key::Character("G".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("h", "H"),
                        VirtualKey::new(KeyCode::KeyH, (Key::Character("h".into()), Key::Character("H".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("j", "J"),
                        VirtualKey::new(KeyCode::KeyJ, (Key::Character("j".into()), Key::Character("J".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("k", "K"),
                        VirtualKey::new(KeyCode::KeyK, (Key::Character("k".into()), Key::Character("K".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("l", "L"),
                        VirtualKey::new(KeyCode::KeyL, (Key::Character("l".into()), Key::Character("L".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new(";", ":"),
                        VirtualKey::new(
                            KeyCode::Semicolon,
                            (Key::Character(";".into()), Key::Character(":".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("'", "\""),
                        VirtualKey::new(
                            KeyCode::Quote,
                            (Key::Character("'".into()), Key::Character("\"".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("Del", "DEL"),
                        VirtualKey::new(KeyCode::Delete, (Key::Delete, Key::Delete)),
                        1.5,
                    ),
                ],
                vec![
                    (
                        VirtualKeyLabel::new("z", "Z"),
                        VirtualKey::new(KeyCode::KeyZ, (Key::Character("z".into()), Key::Character("Z".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("x", "X"),
                        VirtualKey::new(KeyCode::KeyX, (Key::Character("x".into()), Key::Character("X".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("c", "C"),
                        VirtualKey::new(KeyCode::KeyC, (Key::Character("c".into()), Key::Character("C".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("v", "V"),
                        VirtualKey::new(KeyCode::KeyV, (Key::Character("v".into()), Key::Character("V".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("Space", "SPACE"),
                        VirtualKey::new(KeyCode::Space, (Key::Space, Key::Space)),
                        2.5,
                    ),
                    (
                        VirtualKeyLabel::new("b", "B"),
                        VirtualKey::new(KeyCode::KeyB, (Key::Character("b".into()), Key::Character("B".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("n", "N"),
                        VirtualKey::new(KeyCode::KeyN, (Key::Character("n".into()), Key::Character("N".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("m", "M"),
                        VirtualKey::new(KeyCode::KeyM, (Key::Character("m".into()), Key::Character("M".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new(",", "<"),
                        VirtualKey::new(KeyCode::Comma, (Key::Character(",".into()), Key::Character("<".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new(".", ">"),
                        VirtualKey::new(
                            KeyCode::Period,
                            (Key::Character(".".into()), Key::Character(">".into())),
                        ),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("/", "?"),
                        VirtualKey::new(KeyCode::Slash, (Key::Character("/".into()), Key::Character("?".into()))),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("<=", "<="),
                        VirtualKey::new(KeyCode::ArrowLeft, (Key::ArrowLeft, Key::ArrowLeft)),
                        1.,
                    ),
                    (
                        VirtualKeyLabel::new("=>", "=>"),
                        VirtualKey::new(KeyCode::ArrowRight, (Key::ArrowRight, Key::ArrowRight)),
                        1.,
                    ),
                ],
            ],
        }
    }
}

#[derive(Component)]
#[require(Interaction)]
pub struct VirtualKey {
    key_code: KeyCode,
    logical_key: (Key, Key),
}

impl VirtualKey {
    pub fn new(key_code: KeyCode, logical_key: (Key, Key)) -> Self {
        Self { key_code, logical_key }
    }
}

#[derive(Component, Clone)]
#[require(Text)]
pub struct VirtualKeyLabel {
    main: String,
    alt: String,
}

impl VirtualKeyLabel {
    pub fn new(main: &str, alt: &str) -> Self {
        Self {
            main: main.to_string(),
            alt: alt.to_string(),
        }
    }
}

/// Show virtual keyboard event:
/// * true: show
/// * false: hide
#[derive(Event)]
pub struct ShowVirtualKeyboard(pub bool);

fn spawn_virtual_keyboard(
    mut commands: Commands,
    theme: Res<VirtualKeyboardTheme>,
    keys: Res<VirtualKeysList>,
    query: Query<Entity, With<VirtualKeyboard>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }

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
        for row in keys.keys.iter() {
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
                    for (label, key, key_size) in row {
                        spawn_key(builder, label, key.key_code, key.logical_key.clone(), *key_size, &theme);
                    }
                });
        }
    });
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
    label: &VirtualKeyLabel,
    key_code: KeyCode,
    logical_key: (Key, Key),
    key_size: f32, // 1u, 1.5u, 2u, ...
    theme: &VirtualKeyboardTheme,
) {
    builder
        .spawn((
            VirtualKey { key_code, logical_key },
            ButtonTransformEffect::default(),
            ButtonColorEffect::default(),
            Node {
                width: theme.key_size_1u * key_size,
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
                label.clone(),
                Text::new(label.main.clone()),
                theme.text_font.clone(),
                TextColor::from(theme.text_color),
            ));
        })
        .observe(on_click);
}

fn on_click(
    trigger: Trigger<Pointer<Click>>,
    keys: Query<&VirtualKey>,
    mut event: EventWriter<KeyboardInput>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut virtual_keyboard: Single<&mut VirtualKeyboard>,
    mut text: Query<(&mut Text, &VirtualKeyLabel)>,
) {
    if let Ok(window) = windows.get_single() {
        if let Ok(key) = keys.get(trigger.entity()) {
            if key.logical_key.0 == Key::Shift {
                virtual_keyboard.show_alt = !virtual_keyboard.show_alt;

                for (mut text, label) in text.iter_mut() {
                    **text = if virtual_keyboard.show_alt {
                        label.alt.clone()
                    } else {
                        label.main.clone()
                    };
                }
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
