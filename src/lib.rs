// Copyright 2024,2025 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//!
//! Add plugin `TextEditPlugin` to the app and define which states it will run in:
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_text_edit::TextEditPlugin;
//!
//! #[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
//! enum GameState {
//!     #[default]
//!     Menu,
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Add the plugin
//!         .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
//!         .run();
//! }
//! ```
//!
//! If you don't care to game state and want to always run input text, use `TextEditPluginAnyState`:
//! ```rust
//! use bevy::prelude::*;
//! use bevy_text_edit::TextEditPluginAnyState;
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     // Add the plugin
//!     .add_plugins(TextEditPluginAnyState::any())
//!     .run();
//! ```
//!
//! ### Component
//!
//! Insert the component `TextEditable` into any text entity that needs to be editable:
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_text_edit::TextEditable;
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn((
//!         TextEditable::default(), // Mark text is editable
//!         Text::new("Input Text 1"),
//!     ));
//! }
//! ```
//!
//! Only text that is focused by clicking gets keyboard input.
//!
//!
//! It is also possible to limit which characters are allowed to enter through `filter_in` and `filter_out` attribute (regex is supported):
//! ```rust
//! use bevy::prelude::*;
//! use bevy_text_edit::TextEditable;
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn((
//!         TextEditable {
//!             filter_in: vec!["[0-9]".into(), " ".into()], // Only allow number and space
//!             filter_out: vec!["5".into()],                // Ignore number 5
//!             ..default()
//!         },
//!         Text::new("Input Text 1"),
//!     ));
//! }
//! ```
//!
//! ### Get text
//!
//! The edited text can be retrieved from event `TextEdited`.
//! ```rust
//! use bevy::prelude::*;
//! use bevy_text_edit::TextEdited;
//!
//! fn get_text(
//!     mut event: EventReader<TextEdited>,
//! ) {
//!     for e in event.read() {
//!         info!("Entity {}: {}", e.entity, e.text);
//!     }
//! }
//! ```

#[cfg(feature = "experimental")]
pub mod experimental;
pub mod virtual_keyboard;

use crate::virtual_keyboard::{VirtualKey, VirtualKeyboard, VirtualKeyboardPlugin, VirtualKeyboardPos};
#[cfg(feature = "clipboard")]
use arboard::Clipboard;
use bevy::app::{App, Plugin, Update};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
#[cfg(feature = "log")]
use bevy::log::error;
use bevy::prelude::{in_state, IntoScheduleConfigs, KeyCode, States};
use bevy::prelude::{
    Alpha, ButtonInput, Changed, Commands, Component, Deref, DerefMut, Entity, Event, EventReader, EventWriter,
    GlobalTransform, MouseButton, Query, Res, ResMut, Resource, Text, Time, Timer, TimerMode, Touches, With, Without,
};
use bevy::text::TextColor;
use bevy::ui::Interaction;
use regex_lite::Regex;

macro_rules! plugin_systems {
    ( ) => {
        (
            listen_changing_focus,
            focus_text_box,
            listen_keyboard_input,
            blink_cursor,
            display_placeholder,
        )
            .chain()
    };
}

/// The main plugin
#[derive(Default)]
pub struct TextEditPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in.
    pub states: Vec<T>,
}

impl<T> Plugin for TextEditPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualKeyboardPlugin::new(self.states.clone()))
            .insert_resource(TextEditConfig::new())
            .insert_resource(DisplayTextCursor(DEFAULT_CURSOR))
            .insert_resource(BlinkInterval(Timer::from_seconds(BLINK_INTERVAL, TimerMode::Repeating)))
            .add_event::<TextFocusChanged>()
            .add_event::<TextEdited>();

        #[cfg(feature = "clipboard")]
        app.insert_resource(ClipboardMng::new());

        if self.states.is_empty() {
            app.add_systems(Update, plugin_systems!());
        } else {
            for state in &self.states {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        }
    }
}

impl<T> TextEditPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states }
    }

    pub fn any() -> Self {
        Self { states: Vec::new() }
    }
}

#[derive(States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DummyState {}

/// Use this if you don't care to state and want this plugin's systems always run.
pub struct TextEditPluginAnyState;

impl TextEditPluginAnyState {
    pub fn any() -> TextEditPlugin<DummyState> {
        TextEditPlugin::new(Vec::new())
    }
}

const DEFAULT_CURSOR: char = '|';
const BLINK_INTERVAL: f32 = 0.5;

/// Current position of cursor in the text.
#[derive(Component, Default)]
pub struct CursorPosition {
    pub pos: usize,
}

/// The text that will be displayed as cursor. Default is `|`.
#[derive(Resource, Deref, DerefMut)]
pub struct DisplayTextCursor(char);

/// Text cursor blink interval in millisecond.
#[derive(Resource, Deref, DerefMut)]
pub struct BlinkInterval(Timer);

/// Event when text is focused
#[derive(Event)]
pub enum TextFocusChanged {
    Show(f32),
    Hide,
}

/// Mark a text entity is focused. Normally done by mouse click.
#[derive(Component)]
pub struct TextEditFocus;

/// Mark a text is editable.
/// You can limit which characters are allowed to enter through `filter_in` and `filter_out` attribute (regex is supported):
/// ```rust
/// use bevy::prelude::*;
/// use bevy_text_edit::TextEditable;
///
/// fn setup(mut commands: Commands) {
///     commands.spawn((
///         TextEditable {
///             filter_in: vec!["[0-9]".into(), " ".into()], // Only allow number and space
///             filter_out: vec!["5".into()],                // Ignore number 5
///             ..default()
///         },
///         Text::new("Input Text 1"),
///     ));
/// }
/// ```
#[derive(Component)]
#[require(Interaction, Text)]
pub struct TextEditable {
    /// Character in this list won't be added to the text.
    pub filter_out: Vec<String>,

    /// If not empty, only character in this list will be added to the text.
    pub filter_in: Vec<String>,

    /// Maximum text length. Default is 254. 0 means unlimited.
    pub max_length: usize,

    /// Text placeholder. Display when text box is empty.
    pub placeholder: String,
    pub is_placeholder_shown: bool,
    pub orig_text_alpha: f32,
}

impl Default for TextEditable {
    fn default() -> Self {
        Self {
            filter_out: Default::default(),
            filter_in: Default::default(),
            max_length: 254,
            placeholder: String::new(),
            is_placeholder_shown: false,
            orig_text_alpha: 1.0,
        }
    }
}

#[derive(Event, Clone)]
pub struct TextEdited {
    pub text: String,
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct TextEditConfig {
    pub enable_virtual_keyboard: bool,

    /// Fixed position of virtual keyboard
    pub virtual_keyboard_pos: Option<VirtualKeyboardPos>,

    /// Blink the text cursor.
    pub blink: bool,

    pub placeholder_alpha: f32,

    /// Time (sec) wait before start repeat. Only apply to virtual keyboard.
    /// Default: 0.5.
    pub repeated_key_init_timeout: f32,

    /// Time (sec) to repeat key. Only apply to virtual keyboard.
    /// Default: 0.05.
    pub repeated_key_timeout: f32,
}

impl TextEditConfig {
    pub fn new() -> Self {
        Self {
            placeholder_alpha: 0.2,
            repeated_key_init_timeout: 0.5,
            repeated_key_timeout: 0.05,
            ..Self::default()
        }
    }
}

#[cfg(feature = "clipboard")]
#[derive(Resource)]
struct ClipboardMng {
    clipboard: Option<Clipboard>,
}

#[cfg(feature = "clipboard")]
impl ClipboardMng {
    fn new() -> Self {
        match Clipboard::new() {
            Ok(c) => Self { clipboard: Some(c) },
            Err(_e) => {
                #[cfg(feature = "log")]
                error!("Failed to create clipboard: {}", _e);
                Self { clipboard: None }
            }
        }
    }
}

fn unfocus_text_box(
    commands: &mut Commands,
    text_focus: &mut Query<(Entity, &CursorPosition, &mut Text, &TextEditable), With<TextEditFocus>>,
    ignore_entity: Option<Entity>,
    text_edited_event: &mut EventWriter<TextEdited>,
) {
    for (e, cursor, mut text, text_editable) in text_focus.iter_mut() {
        if ignore_entity.is_none() || e != ignore_entity.unwrap() {
            commands.entity(e).remove::<TextEditFocus>();

            if text.len() > cursor.pos {
                text.remove(cursor.pos);
            }
            commands.entity(e).remove::<CursorPosition>();
            commands.entity(e).remove::<TextEditFocus>();

            let edited_text = if text_editable.is_placeholder_shown { String::new() } else { text.0.clone() };

            let text_edited = TextEdited {
                text: edited_text,
                entity: e,
            };
            text_edited_event.write(text_edited.clone());
            commands.trigger_targets(text_edited, e);
        }
    }
}

fn focus_text_box(
    mut commands: Commands,
    mut focused_texts: Query<
        (&mut Text, &mut TextColor, &mut TextEditable, Entity),
        (With<TextEditFocus>, Without<CursorPosition>),
    >,
    display_cursor: Res<DisplayTextCursor>,
) {
    for (mut text, mut text_color, mut text_editable, e) in focused_texts.iter_mut() {
        if text_editable.is_placeholder_shown {
            **text = String::new();
            text_editable.is_placeholder_shown = false;
            text_color.set_alpha(text_editable.orig_text_alpha);
        }

        let pos = text.len();
        commands.entity(e).insert(CursorPosition { pos });
        text.push(**display_cursor);
    }
}

pub fn listen_changing_focus(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut text_interactions: Query<(&Interaction, Entity, &GlobalTransform), (Changed<Interaction>, With<TextEditable>)>,
    virtual_key_interaction: Query<&Interaction, (Changed<Interaction>, With<VirtualKey>, Without<TextEditable>)>,
    virtual_keyboard_interaction: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<VirtualKeyboard>,
            Without<VirtualKey>,
            Without<TextEditable>,
        ),
    >,
    mut focusing_texts: Query<(Entity, &CursorPosition, &mut Text, &TextEditable), With<TextEditFocus>>,
    mut text_edited_event: EventWriter<TextEdited>,
    mut focus_event: EventWriter<TextFocusChanged>,
    mut events: EventReader<KeyboardInput>,
    touches: Res<Touches>,
) {
    let mut unfocus_key_pressed = false;
    for event in events.read() {
        // Only trigger changes at the first time the key is pressed.
        if event.state == ButtonState::Released {
            continue;
        }
        match &event.logical_key {
            Key::Enter => unfocus_key_pressed = true,
            Key::Escape => unfocus_key_pressed = true,
            _ => {}
        }
    }
    let clicked_elsewhere = input.just_pressed(MouseButton::Left) || touches.any_just_pressed();
    if unfocus_key_pressed
        || (text_interactions.is_empty()
            && virtual_key_interaction.is_empty()
            && virtual_keyboard_interaction.is_empty()
            && clicked_elsewhere)
    {
        unfocus_text_box(&mut commands, &mut focusing_texts, None, &mut text_edited_event);
        focus_event.write(TextFocusChanged::Hide);
        return;
    }

    for (interaction, e, global_transform) in text_interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus_event.write(TextFocusChanged::Show(global_transform.translation().y));

            let mut focusing_list = Vec::new();
            for (focusing_e, _, _, _) in focusing_texts.iter() {
                focusing_list.push(focusing_e);
            }

            // Unfocus all text box except which is currently clicked on
            unfocus_text_box(&mut commands, &mut focusing_texts, Some(e), &mut text_edited_event);

            if !focusing_list.contains(&e) {
                commands.entity(e).insert(TextEditFocus);
            }
        }
    }
}

fn listen_keyboard_input(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<(&mut Text, &mut CursorPosition, &TextEditable), With<TextEditFocus>>,
    display_cursor: Res<DisplayTextCursor>,
    #[cfg(feature = "clipboard")] mut clipboard_mng: ResMut<ClipboardMng>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let is_ctrl_pressed = keyboard_input.pressed(KeyCode::ControlRight) || keyboard_input.pressed(KeyCode::ControlLeft);

    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        for (mut text, mut cursor, texteditable) in edit_text.iter_mut() {
            let ignore_list = &texteditable.filter_out;
            let allow_list = &texteditable.filter_in;
            match &event.logical_key {
                Key::Space => {
                    if is_ignored(ignore_list, allow_list, " ".into())
                        || (texteditable.max_length > 0 && text.len() > texteditable.max_length)
                    {
                        continue;
                    }

                    text.insert(cursor.pos, ' ');
                    cursor.pos += 1;
                }
                Key::Backspace => {
                    if cursor.pos > 0 {
                        text.remove(cursor.pos - 1);
                        cursor.pos -= 1;
                    }
                }
                Key::Delete => {
                    if cursor.pos < text.len() - 1 {
                        text.remove(cursor.pos + 1);
                    }
                }
                Key::Character(character) => {
                    if character == "v" && is_ctrl_pressed && cfg!(feature = "clipboard") {
                        #[cfg(feature = "clipboard")]
                        if let Some(clipboard) = clipboard_mng.clipboard.as_mut() {
                            let append_text: String = clipboard
                                .get_text()
                                .unwrap_or_default()
                                .chars()
                                .filter(|&c| !is_ignored(ignore_list, allow_list, c.to_string()))
                                .collect();

                            text.insert_str(cursor.pos, append_text.as_str());
                            cursor.pos += append_text.len();
                        } else {
                            continue;
                        }
                    } else {
                        if is_ignored(ignore_list, allow_list, character.to_string())
                            || (texteditable.max_length > 0 && text.len() > texteditable.max_length)
                        {
                            continue;
                        }
                        let append_text = character.to_string();

                        text.insert_str(cursor.pos, append_text.as_str());
                        cursor.pos += append_text.len();
                    }
                }
                Key::ArrowLeft => {
                    if cursor.pos > 0 {
                        text.remove(cursor.pos);

                        cursor.pos -= 1;
                        text.insert(cursor.pos, **display_cursor);
                    }
                }
                Key::ArrowRight => {
                    if cursor.pos < text.len() - 1 {
                        text.remove(cursor.pos);

                        cursor.pos += 1;
                        text.insert(cursor.pos, **display_cursor);
                    }
                }
                Key::Home => {
                    text.remove(cursor.pos);
                    cursor.pos = 0;
                    text.insert(0, **display_cursor);
                }
                Key::End => {
                    text.remove(cursor.pos);
                    cursor.pos = text.len();
                    text.push(**display_cursor);
                }
                _ => continue,
            }
        }
    }
}

fn blink_cursor(
    time: Res<Time>,
    mut blink_interval: ResMut<BlinkInterval>,
    display_text_cursor: Res<DisplayTextCursor>,
    mut query: Query<(&mut Text, &CursorPosition), (With<TextEditFocus>, With<TextEditable>)>,
    config: Res<TextEditConfig>,
) {
    blink_interval.tick(time.delta());
    for (mut text, cursor_pos) in query.iter_mut() {
        if config.blink && blink_interval.just_finished() && text.len() > cursor_pos.pos {
            let current_cursor = text.as_bytes()[cursor_pos.pos] as char;
            let next_cursor = if current_cursor != **display_text_cursor { **display_text_cursor } else { ' ' };
            text.replace_range(cursor_pos.pos..(cursor_pos.pos + 1), String::from(next_cursor).as_str());
        }
    }
}

fn display_placeholder(
    mut query: Query<(&mut Text, &mut TextColor, &mut TextEditable), Without<TextEditFocus>>,
    config: Res<TextEditConfig>,
) {
    for (mut text, mut text_color, mut text_editable) in query.iter_mut() {
        if text.is_empty() && !text_editable.is_placeholder_shown && !text_editable.placeholder.is_empty() {
            **text = text_editable.placeholder.clone();
            text_editable.is_placeholder_shown = true;
            text_editable.orig_text_alpha = text_color.alpha();
            text_color.set_alpha(config.placeholder_alpha);
        }
    }
}

fn is_ignored(ignore_list: &Vec<String>, allow_list: &Vec<String>, key: String) -> bool {
    for pattern in ignore_list {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(&key) {
                return true;
            }
        } else if *pattern == key {
            return true;
        }
    }

    if !allow_list.is_empty() {
        let mut is_included = false;
        for pattern in allow_list {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&key) {
                    is_included = true;
                    break;
                }
            } else if *pattern == key {
                is_included = true;
                break;
            }
        }
        return !is_included;
    }

    false
}
