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
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, FocusPolicy, JustifyContent, JustifySelf, UiRect, Val};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_support_misc::ui::button::{ButtonColorEffect, ButtonTransformEffect};
use bevy_support_misc::ui::UiSupportPlugin;

const KEY_1U: f32 = 5.5;
const KEY_MARGIN: f32 = 0.5;
const ROW_MARGIN: f32 = 0.5;

pub(crate) struct VirtualKeyboardPlugin;

// TODO: Support gamepad
impl Plugin for VirtualKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiSupportPlugin)
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
    pub key_margin: Val,
    pub row_margin: Val,
}

impl VirtualKeyboardTheme {
    fn new() -> Self {
        Self {
            bg_color: Color::NONE,
            button_color: Color::NONE,
            text_color: Color::WHITE,
            key_size_1u: Val::Percent(KEY_1U),
            key_margin: Val::Percent(KEY_MARGIN),
            row_margin: Val::Percent(ROW_MARGIN),
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

/// List of keys to display on virtual keyboard
#[derive(Resource)]
pub struct VirtualKeysList {
    /// List of keys by row
    pub keys: Vec<Vec<(VirtualKeyLabel, VirtualKey, f32)>>,
}

impl From<Vec<Vec<((&str, &str), KeyCode, Option<(Key, Key)>, f32)>>> for VirtualKeysList {
    /// If logical_key is None, it will be set as Key::Character from label
    fn from(keys: Vec<Vec<((&str, &str), KeyCode, Option<(Key, Key)>, f32)>>) -> Self {
        let mut ret = Self { keys: Vec::new() };
        for row in keys {
            let mut ret_row = Vec::new();
            for (label, key_code, logical_key, size) in row {
                let logical_key =
                    logical_key.unwrap_or((Key::Character(label.0.into()), Key::Character(label.1.into())));
                let vkey = VirtualKey::new(key_code, logical_key);
                let label = VirtualKeyLabel::from(label);
                ret_row.push((label, vkey, size));
            }
            ret.keys.push(ret_row);
        }

        ret
    }
}

impl Default for VirtualKeysList {
    fn default() -> Self {
        Self::from(vec![
            vec![
                (("`", "~"), KeyCode::Backquote, None, 1.),
                (("1", "!"), KeyCode::Digit1, None, 1.),
                (("2", "@"), KeyCode::Digit2, None, 1.),
                (("3", "#"), KeyCode::Digit3, None, 1.),
                (("4", "$"), KeyCode::Digit4, None, 1.),
                (("5", "%"), KeyCode::Digit5, None, 1.),
                (("6", "^"), KeyCode::Digit6, None, 1.),
                (("7", "&"), KeyCode::Digit7, None, 1.),
                (("8", "*"), KeyCode::Digit8, None, 1.),
                (("9", "("), KeyCode::Digit9, None, 1.),
                (("0", ")"), KeyCode::Digit0, None, 1.),
                (("-", "_"), KeyCode::Minus, None, 1.),
                (("=", "+"), KeyCode::Equal, None, 1.),
                (
                    ("Backspace", "BACKSPACE"),
                    KeyCode::Backspace,
                    Some((Key::Backspace, Key::Backspace)),
                    2.,
                ),
            ],
            vec![
                (("q", "Q"), KeyCode::KeyQ, None, 1.),
                (("w", "W"), KeyCode::KeyW, None, 1.),
                (("e", "E"), KeyCode::KeyE, None, 1.),
                (("r", "R"), KeyCode::KeyR, None, 1.),
                (("t", "T"), KeyCode::KeyT, None, 1.),
                (("y", "Y"), KeyCode::KeyY, None, 1.),
                (("u", "U"), KeyCode::KeyU, None, 1.),
                (("i", "I"), KeyCode::KeyI, None, 1.),
                (("o", "O"), KeyCode::KeyO, None, 1.),
                (("p", "P"), KeyCode::KeyP, None, 1.),
                (("[", "{"), KeyCode::BracketLeft, None, 1.),
                (("]", "}"), KeyCode::BracketRight, None, 1.),
                (("\\", "|"), KeyCode::Backslash, None, 1.),
                (("Del", "DEL"), KeyCode::Delete, Some((Key::Delete, Key::Delete)), 1.),
            ],
            vec![
                (
                    ("Shift", "SHIFT"),
                    KeyCode::ShiftLeft,
                    Some((Key::Shift, Key::Shift)),
                    1.5,
                ),
                (("a", "A"), KeyCode::KeyA, None, 1.),
                (("s", "S"), KeyCode::KeyS, None, 1.),
                (("d", "D"), KeyCode::KeyD, None, 1.),
                (("f", "F"), KeyCode::KeyF, None, 1.),
                (("g", "G"), KeyCode::KeyG, None, 1.),
                (("h", "H"), KeyCode::KeyH, None, 1.),
                (("j", "J"), KeyCode::KeyJ, None, 1.),
                (("k", "K"), KeyCode::KeyK, None, 1.),
                (("l", "L"), KeyCode::KeyL, None, 1.),
                ((";", ":"), KeyCode::Semicolon, None, 1.),
                (("'", "\""), KeyCode::Quote, None, 1.),
                (("Enter", "ENTER"), KeyCode::Enter, Some((Key::Enter, Key::Enter)), 1.5),
            ],
            vec![
                (("z", "Z"), KeyCode::KeyZ, None, 1.),
                (("x", "X"), KeyCode::KeyX, None, 1.),
                (("c", "C"), KeyCode::KeyC, None, 1.),
                (("v", "V"), KeyCode::KeyV, None, 1.),
                (("Space", "SPACE"), KeyCode::Space, Some((Key::Space, Key::Space)), 2.5),
                (("b", "B"), KeyCode::KeyB, None, 1.),
                (("n", "N"), KeyCode::KeyN, None, 1.),
                (("m", "M"), KeyCode::KeyM, None, 1.),
                ((",", "<"), KeyCode::Comma, None, 1.),
                ((".", ">"), KeyCode::Period, None, 1.),
                (("/", "?"), KeyCode::Slash, None, 1.),
                (
                    ("<=", "<="),
                    KeyCode::ArrowLeft,
                    Some((Key::ArrowLeft, Key::ArrowLeft)),
                    1.,
                ),
                (
                    ("=>", "=>"),
                    KeyCode::ArrowRight,
                    Some((Key::ArrowRight, Key::ArrowRight)),
                    1.,
                ),
            ],
        ])
    }
}

#[derive(Component)]
#[require(Interaction)]
pub struct VirtualKey {
    pub key_code: KeyCode,
    pub logical_key: (Key, Key),
}

impl VirtualKey {
    pub fn new(key_code: KeyCode, logical_key: (Key, Key)) -> Self {
        Self { key_code, logical_key }
    }
}

#[derive(Component, Clone)]
#[require(Text)]
pub struct VirtualKeyLabel {
    pub main: String,
    pub alt: String,
}

impl VirtualKeyLabel {
    pub fn new(main: &str, alt: &str) -> Self {
        Self {
            main: main.to_string(),
            alt: alt.to_string(),
        }
    }
}

impl From<(&str, &str)> for VirtualKeyLabel {
    fn from(value: (&str, &str)) -> Self {
        Self::new(value.0, value.1)
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
        FocusPolicy::Block,
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(98.),
            height: Val::Percent(40.),
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::End,
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
                    margin: UiRect::vertical(theme.row_margin),
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
                margin: UiRect::horizontal(theme.key_margin),
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
                    **text = if virtual_keyboard.show_alt { label.alt.clone() } else { label.main.clone() };
                }
            } else {
                let logical_key =
                    if virtual_keyboard.show_alt { key.logical_key.1.clone() } else { key.logical_key.0.clone() };
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
