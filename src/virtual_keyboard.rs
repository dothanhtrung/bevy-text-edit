// Copyright 2024,2025 Trung Do <dothanhtrung@pm.me>

use crate::{TextEditConfig, TextFocusChanged};
use bevy::app::{App, Plugin, Startup};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::{
    in_state, on_event, AlignContent, AlignSelf, BorderColor, ChildOf, Color, Commands, Component, Deref, DerefMut,
    Entity, Event, EventReader, EventWriter, Gamepad, GamepadButton, Handle, Image, ImageNode,
    Interaction, IntoScheduleConfigs, JustifyItems, KeyCode, Luminance, Node, Pointer, Pressed, Query, Released, Res, ResMut, Resource,
    Single, States, Text, TextColor, TextFont, Timer, TimerMode, Trigger, Update, Visibility, Window, With, ZIndex,
};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, FocusPolicy, JustifyContent, JustifySelf, UiRect, Val};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_auto_timer::{ActionOnFinish, AutoTimer, AutoTimerFinished, AutoTimerPlugin};
use bevy_support_misc::ui::button::{ButtonColorEffect, ButtonTransformEffect};
use bevy_support_misc::ui::UiSupportPlugin;
use std::cmp::max;
use std::time::Duration;

macro_rules! vk_plugin_systems {
    ( ) => {
        (
            show_keyboard.run_if(on_event::<TextFocusChanged>),
            spawn_virtual_keyboard.run_if(on_event::<VirtualKeyboardChanged>),
            gamepad_system,
        )
    };
}

const KEY_1U: f32 = 5.5; // Percent
const KEY_MARGIN: f32 = 0.3; // Percent
const ROW_MARGIN: f32 = 0.4; // Percent
const WIDTH: f32 = 90.; // Percent
const HEIGHT: f32 = 30.; // Percent

pub(crate) struct VirtualKeyboardPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in.
    pub states: Vec<T>,
}

impl<T> VirtualKeyboardPlugin<T>
where
    T: States,
{
    pub(crate) fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

impl<T> Plugin for VirtualKeyboardPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugins((UiSupportPlugin, AutoTimerPlugin::new(self.states.clone())))
            .insert_resource(VirtualKeyboardTheme::new())
            .insert_resource(VirtualKeysList::default())
            .insert_resource(VirtualKeyEntities::default())
            .insert_resource(SelectingKey::default())
            .add_event::<VirtualKeyboardChanged>()
            .add_systems(Startup, spawn_virtual_keyboard);

        if self.states.is_empty() {
            app.add_systems(Update, vk_plugin_systems!());
        } else {
            for state in &self.states {
                app.add_systems(Update, vk_plugin_systems!().run_if(in_state(state.clone())));
            }
        }
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
    pub width: Val,
    pub height: Val,
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
            width: Val::Percent(WIDTH),
            height: Val::Percent(HEIGHT),
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

/// List of keys to display on the virtual keyboard.
/// This key list can be overridden.
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

#[derive(Default, Clone, Copy)]
pub enum VirtualKeyboardPos {
    #[default]
    Bottom,
    Top,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct VirtualKeyEntities(Vec<Vec<Entity>>);

#[derive(Resource, Default)]
struct SelectingKey {
    row: usize,
    col: usize,
}

#[derive(Event)]
struct KeySelected;

#[derive(Event)]
struct KeyUnselected;

#[derive(Event)]
struct KeyPressed;

fn spawn_virtual_keyboard(
    mut commands: Commands,
    theme: Res<VirtualKeyboardTheme>,
    keys: Res<VirtualKeysList>,
    query: Query<Entity, With<VirtualKeyboard>>,
    mut virtual_key_entities: ResMut<VirtualKeyEntities>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    virtual_key_entities.clear();

    let mut cmd = if let Some(bg_image) = theme.bg_image.clone() {
        commands.spawn(ImageNode {
            image: bg_image,
            ..default()
        })
    } else {
        commands.spawn_empty()
    };

    cmd.insert((
        VirtualKeyboard::default(),
        FocusPolicy::Block,
        Node {
            flex_direction: FlexDirection::Column,
            width: theme.width,
            height: theme.height,
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
                    height: Val::Percent(100. / keys.keys.len() as f32),
                    margin: UiRect::vertical(theme.row_margin),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|builder| {
                    let mut row_entities = Vec::new();
                    for (label, key, key_size) in row {
                        let e = spawn_key(builder, label, key.key_code, key.logical_key.clone(), *key_size, &theme);
                        row_entities.push(e);
                    }
                    virtual_key_entities.push(row_entities);
                });
        }
    });
}

fn show_keyboard(
    mut events: EventReader<TextFocusChanged>,
    mut query: Query<(&mut Visibility, &mut Node), With<VirtualKeyboard>>,
    mut repeated_timer: Query<&mut AutoTimer, With<VirtualKey>>,
    config: Res<TextEditConfig>,
    windows: Query<&Window>,
) {
    for event in events.read() {
        match *event {
            TextFocusChanged::Show(global_y) => {
                if config.enable_virtual_keyboard {
                    for (mut visibility, mut node) in query.iter_mut() {
                        *visibility = Visibility::Visible;

                        if let Some(pos) = config.virtual_keyboard_pos {
                            match pos {
                                VirtualKeyboardPos::Bottom => {
                                    node.align_self = AlignSelf::End;
                                }
                                VirtualKeyboardPos::Top => {
                                    node.align_self = AlignSelf::Start;
                                }
                            }
                        } else if let Ok(window) = windows.single() {
                            if global_y >= window.resolution.height() / 2. {
                                node.align_self = AlignSelf::Start;
                            } else {
                                node.align_self = AlignSelf::End;
                            }
                        }
                    }
                }
            }
            TextFocusChanged::Hide => {
                for (mut visibility, _) in query.iter_mut() {
                    *visibility = Visibility::Hidden;
                    for mut timer in repeated_timer.iter_mut() {
                        timer.timer.pause();
                    }
                }
            }
        }
    }
}

fn spawn_key(
    builder: &mut RelatedSpawnerCommands<ChildOf>,
    label: &VirtualKeyLabel,
    key_code: KeyCode,
    logical_key: (Key, Key),
    key_size: f32, // 1u, 1.5u, 2u, ...
    theme: &VirtualKeyboardTheme,
) -> Entity {
    let mut timer = Timer::default();
    timer.pause();

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
            AutoTimer {
                timer,
                action_on_finish: ActionOnFinish::Nothing,
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                label.clone(),
                Text::new(label.main.clone()),
                theme.text_font.clone(),
                TextColor::from(theme.text_color),
            ));
        })
        .observe(on_pointer_press)
        .observe(on_key_press)
        .observe(on_release)
        .observe(on_repeat)
        .observe(on_selected)
        .observe(on_unselected)
        .id()
}

fn on_pointer_press(
    trigger: Trigger<Pointer<Pressed>>,
    mut keys: Query<(&VirtualKey, &mut AutoTimer)>,
    mut event: EventWriter<KeyboardInput>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut virtual_keyboard: Single<&mut VirtualKeyboard>,
    mut text: Query<(&mut Text, &VirtualKeyLabel)>,
    config: Res<TextEditConfig>,
) {
    on_press(
        trigger.target(),
        &mut keys,
        &mut event,
        windows,
        &mut virtual_keyboard,
        &mut text,
        config,
    );
}

fn on_key_press(
    trigger: Trigger<KeyPressed>,
    mut keys: Query<(&VirtualKey, &mut AutoTimer)>,
    mut event: EventWriter<KeyboardInput>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut virtual_keyboard: Single<&mut VirtualKeyboard>,
    mut text: Query<(&mut Text, &VirtualKeyLabel)>,
    config: Res<TextEditConfig>,
) {
    on_press(
        trigger.target(),
        &mut keys,
        &mut event,
        windows,
        &mut virtual_keyboard,
        &mut text,
        config,
    );
}

fn on_press(
    target: Entity,
    keys: &mut Query<(&VirtualKey, &mut AutoTimer)>,
    event: &mut EventWriter<KeyboardInput>,
    windows: Query<Entity, With<PrimaryWindow>>,
    virtual_keyboard: &mut Single<&mut VirtualKeyboard>,
    text: &mut Query<(&mut Text, &VirtualKeyLabel)>,
    config: Res<TextEditConfig>,
) {
    if let Ok(window) = windows.single() {
        if let Ok((key, mut timer)) = keys.get_mut(target) {
            if key.logical_key.0 == Key::Shift {
                virtual_keyboard.show_alt = !virtual_keyboard.show_alt;

                for (mut text, label) in text.iter_mut() {
                    **text = if virtual_keyboard.show_alt { label.alt.clone() } else { label.main.clone() };
                }
            } else {
                timer
                    .timer
                    .set_duration(Duration::from_secs_f32(config.repeated_key_init_timeout));
                timer.timer.set_mode(TimerMode::Once);
                timer.timer.reset();
                timer.timer.unpause();

                let logical_key =
                    if virtual_keyboard.show_alt { key.logical_key.1.clone() } else { key.logical_key.0.clone() };
                event.write(KeyboardInput {
                    key_code: key.key_code,
                    logical_key,
                    state: ButtonState::Pressed,
                    repeat: false,
                    window,
                    text: None, // FIXME: Do plugin need to send the key text
                });
            }
        }
    }
}

fn on_release(trigger: Trigger<Pointer<Released>>, mut repeated_timer: Query<&mut AutoTimer, With<VirtualKey>>) {
    if let Ok(mut timer) = repeated_timer.get_mut(trigger.target()) {
        timer.timer.pause();
    }
}

fn on_repeat(
    trigger: Trigger<AutoTimerFinished>,
    mut keys: Query<(&VirtualKey, &mut AutoTimer)>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut event: EventWriter<KeyboardInput>,
    virtual_keyboard: Single<&VirtualKeyboard>,
    config: Res<TextEditConfig>,
) {
    if let Ok(window) = windows.single() {
        if let Ok((key, mut timer)) = keys.get_mut(trigger.target()) {
            let logical_key =
                if virtual_keyboard.show_alt { key.logical_key.1.clone() } else { key.logical_key.0.clone() };
            event.write(KeyboardInput {
                key_code: key.key_code,
                logical_key,
                state: ButtonState::Pressed,
                repeat: false,
                window,
                text: None, // FIXME: Do plugin need to send the key text
            });

            let repeat_duration = Duration::from_secs_f32(config.repeated_key_timeout);
            if timer.timer.duration() != repeat_duration {
                timer.timer.set_duration(repeat_duration);
            }
            if timer.timer.mode() != TimerMode::Repeating {
                timer.timer.set_mode(TimerMode::Repeating);
            }
            timer.timer.unpause();
        }
    }
}

fn on_selected(trigger: Trigger<KeySelected>, bg_keys: Query<(Entity, &mut BackgroundColor), With<VirtualKey>>) {
    for (e, mut bg) in bg_keys {
        if e == trigger.target() {
            bg.0 = bg.0.lighter(0.3);
            return;
        }
    }
}

fn on_unselected(trigger: Trigger<KeySelected>, bg_keys: Query<(Entity, &mut BackgroundColor), With<VirtualKey>>) {
    for (e, mut bg) in bg_keys {
        if e == trigger.target() {
            bg.0 = bg.0.darker(0.3);
            return;
        }
    }
}

fn gamepad_system(
    mut commands: Commands,
    gamepads: Query<&Gamepad>,
    mut selecting_key: ResMut<SelectingKey>,
    keys: Res<VirtualKeysList>,
    key_entities: Res<VirtualKeyEntities>,
) {
    if keys.keys.is_empty() || selecting_key.row >= keys.keys.len() {
        return;
    }

    let mut select_changed = false;
    let row_length = keys.keys.len();
    let col_length = keys.keys[selecting_key.row].len();
    let old_select = (selecting_key.row, selecting_key.col);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            selecting_key.row = max(selecting_key.row - 1, 0);
            select_changed = true;
        } else if gamepad.just_pressed(GamepadButton::DPadDown) {
            selecting_key.row = (selecting_key.row + 1) % row_length;
            select_changed = true;
        } else if gamepad.just_pressed(GamepadButton::DPadLeft) {
            selecting_key.col = max(selecting_key.col - 1, 0);
            select_changed = true;
        } else if gamepad.just_pressed(GamepadButton::DPadRight) {
            selecting_key.col = selecting_key.col + 1;
            if selecting_key.col >= col_length {
                selecting_key.col = 0;
                selecting_key.row = (selecting_key.row + 1) % row_length;
            }
            select_changed = true;
        } else if gamepad.just_pressed(GamepadButton::South) {
            if selecting_key.row < key_entities.len() && selecting_key.col < key_entities[selecting_key.row].len() {
                let e = key_entities[selecting_key.row][selecting_key.col];
                commands.trigger_targets(KeyPressed, e);
            }
        }
    }

    if select_changed {
        if selecting_key.row < key_entities.len() && selecting_key.col < key_entities[selecting_key.row].len() {
            let new_select = key_entities[selecting_key.row][selecting_key.col];
            commands.trigger_targets(KeySelected, new_select);
        }

        if old_select.0 < key_entities.len() && old_select.1 < key_entities[old_select.0].len() {
            let old_select = key_entities[old_select.0][old_select.1];
            commands.trigger_targets(KeyUnselected, old_select);
        }
    }
}
