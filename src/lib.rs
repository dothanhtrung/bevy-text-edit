// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//!
//! Add plugin `TextEditPlugin` to the app and define which states it will run in:
//!
//! ```rust
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
//!         .run;
//! }
//! ```
//!
//! If you don't care to game state and want to always run input text, use `TextEditPluginNoState`:
//! ```rust
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     // Add the plugin
//!     .add_plugins(TextEditPluginNoState)
//!     .add_systems(Startup, setup)
//!     .run();
//! ```
//!
//! ### Component
//!
//! Insert component `TextEditable` and `Interaction` into any text entity that needs to be editable:
//!
//! ```rust
//! commands.spawn((
//!     TextEditable::default(), // Mark text is editable
//!     Interaction::None,       // Mark entity is interactable
//!     TextBundle::from_section(
//!         "Input Text 1",
//!         TextStyle::default(),
//!     ),
//! ));
//! ```
//!
//! Only text that is focused by clicking gets keyboard input.
//!
//!
//! It is also possible to limit which characters are allowed to enter through `allow` and `ignore` attribute. Regex is supported:
//! ```rust
//! commands.spawn((
//!     TextEditable {
//!         allow: vec!["[0-9]".into(), " ".into()], // Only allow number and space
//!         ignore: vec!["5".into()],                // Ignore number 5
//!     },
//!     Interaction::None,
//!     TextBundle::from_section(
//!         "Input Text 1",
//!         TextStyle::default(),
//!     ),
//! ));
//! ```

use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{
    ButtonInput, Changed, Commands, Component, Deref, DerefMut, Entity, EventReader, IntoSystemConfigs, MouseButton,
    Query, Res, Resource, Text, With, Without,
};
#[cfg(feature = "state")]
use bevy::prelude::{in_state, States};
use bevy::ui::Interaction;
use regex_lite::Regex;

macro_rules! plugin_systems {
    ( ) => {
        (listen_changing_focus, focus_text_box, listen_keyboard_input).chain()
    };
}

const DEFAULT_CURSOR: &str = "|";

/// Current position of cursor in the text
#[derive(Component, Default)]
pub struct CursorPosition {
    pub section: usize,
    pub pos: usize,
}

/// The text that will be displayed as cursor. Default is `|`.
#[derive(Resource, Deref, DerefMut)]
pub struct DisplayTextCursor(String);

/// The main plugin
#[cfg(feature = "state")]
#[derive(Default)]
pub struct TextEditPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in
    pub states: Option<Vec<T>>,
}

#[cfg(feature = "state")]
impl<T> Plugin for TextEditPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayTextCursor(DEFAULT_CURSOR.to_string()));
        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        } else {
            app.add_systems(Update, plugin_systems!());
        }
    }
}

#[cfg(feature = "state")]
impl<T> TextEditPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states: Some(states) }
    }
}

/// Use this if you don't care to state and want this plugin's systems run always.
#[derive(Default)]
pub struct TextEditPluginNoState;

impl Plugin for TextEditPluginNoState {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayTextCursor(DEFAULT_CURSOR.to_string()))
            .add_systems(Update, plugin_systems!());
    }
}

/// Mark a text entity is focused. Normally done by mouse click.
#[derive(Component)]
pub struct TextEditFocus;

/// Mark a text is editable.  
/// You can limit which characters are allowed to enter through `allow` and `ignore` attribute. Regex is supported:
/// ```rust
/// commands.spawn((
///     TextEditable {
///         allow: vec!["[0-9]".into(), " ".into()], // Only allow number and space
///         ignore: vec!["5".into()],                // Ignore number 5
///     },
///     Interaction::None,
///     TextBundle::from_section(
///         "Input Text 1",
///         TextStyle::default(),
///     ),
/// ));
/// ```
#[derive(Component)]
pub struct TextEditable {
    /// Character in this list won't be added to the text.
    pub filter_out: Vec<String>,

    /// If not empty, only character in this list will be added to the text.
    pub filter_in: Vec<String>,

    /// Maximum text length. Default is 254.
    pub max_length: usize,
}

impl Default for TextEditable {
    fn default() -> Self {
        Self {
            filter_out: Default::default(),
            filter_in: Default::default(),
            max_length: 254,
        }
    }
}

fn unfocus_text_box(
    commands: &mut Commands,
    text_focus: &mut Query<(Entity, &CursorPosition, &mut Text), With<TextEditFocus>>,
    ignore_entity: Option<Entity>,
) {
    for (e, cursor, mut text) in text_focus.iter_mut() {
        if ignore_entity.is_none() || e != ignore_entity.unwrap() {
            commands.entity(e).remove::<TextEditFocus>();

            if text.sections.len() > cursor.section && text.sections[cursor.section].value.len() > cursor.pos {
                text.sections[cursor.section].value.remove(cursor.pos);
            }
            commands.entity(e).remove::<CursorPosition>();
            commands.entity(e).remove::<TextEditFocus>();
        }
    }
}

fn focus_text_box(
    mut commands: Commands,
    mut focused_texts: Query<(&mut Text, Entity), (With<TextEditFocus>, Without<CursorPosition>)>,
    display_cursor: Res<DisplayTextCursor>,
) {
    for (mut text, e) in focused_texts.iter_mut() {
        if !text.sections.is_empty() {
            let section = text.sections.len() - 1;
            let pos = text.sections[section].value.len();
            commands.entity(e).insert(CursorPosition { section, pos });
            text.sections
                .last_mut()
                .unwrap()
                .value
                .push_str(display_cursor.as_str());
        }
    }
}

pub fn listen_changing_focus(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut text_interactions: Query<(&Interaction, Entity), (Changed<Interaction>, With<TextEditable>)>,
    other_interactions: Query<&Interaction, (Changed<Interaction>, Without<TextEditable>)>,
    mut focusing_texts: Query<(Entity, &CursorPosition, &mut Text), With<TextEditFocus>>,
) {
    let mut clicked_elsewhere = input.just_pressed(MouseButton::Left);
    for oth_itr in other_interactions.iter() {
        if *oth_itr == Interaction::Pressed {
            clicked_elsewhere = true;
        }
    }
    if text_interactions.is_empty() && clicked_elsewhere {
        unfocus_text_box(&mut commands, &mut focusing_texts, None);
        return;
    }

    for (interaction, e) in text_interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            let mut focusing_list = Vec::new();
            for (focusing_e, _, _) in focusing_texts.iter() {
                focusing_list.push(focusing_e);
            }

            unfocus_text_box(&mut commands, &mut focusing_texts, Some(e));
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
) {
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if event.state == ButtonState::Released {
            continue;
        }

        for (mut text, mut cursor, texteditable) in edit_text.iter_mut() {
            let ignore_list = &texteditable.filter_out;
            let allow_list = &texteditable.filter_in;
            let mut text_len = 0;

            if text.sections.len() <= cursor.section {
                continue;
            }

            for section in &text.sections {
                text_len += section.value.len();
            }

            match &event.logical_key {
                Key::Space => {
                    if is_ignored(ignore_list, allow_list, " ".into()) || text_len > texteditable.max_length {
                        continue;
                    }

                    text.sections[cursor.section].value.insert(cursor.pos, ' ');
                    cursor.pos += 1;
                }
                Key::Backspace => {
                    if cursor.pos > 0 {
                        text.sections[cursor.section].value.remove(cursor.pos - 1);
                        cursor.pos -= 1;
                    } else if cursor.section > 0 {
                        text.sections[cursor.section].value.remove(cursor.pos);

                        cursor.section -= 1;
                        text.sections[cursor.section].value.pop();
                        text.sections[cursor.section].value.push_str(display_cursor.as_str());
                        cursor.pos = text.sections[cursor.section].value.len() - 1;
                    }
                }
                Key::Delete => {
                    if cursor.pos < text.sections[cursor.section].value.len() - 1 {
                        text.sections[cursor.section].value.remove(cursor.pos + 1);
                    } else if cursor.section < text.sections.len() - 1 {
                        text.sections[cursor.section].value.remove(cursor.pos);

                        cursor.section += 1;
                        if !text.sections[cursor.section].value.is_empty() {
                            text.sections[cursor.section].value.remove(0);
                        }
                        text.sections[cursor.section]
                            .value
                            .insert_str(0, display_cursor.as_str());
                        cursor.pos = 0;
                    }
                }
                Key::Character(character) => {
                    if is_ignored(ignore_list, allow_list, character.to_string()) || text_len > texteditable.max_length
                    {
                        continue;
                    }

                    text.sections[cursor.section].value.insert_str(cursor.pos, character);
                    cursor.pos += character.len();
                }
                Key::ArrowLeft => {
                    if cursor.pos > 0 {
                        text.sections[cursor.section].value.remove(cursor.pos);

                        cursor.pos -= 1;
                        text.sections[cursor.section]
                            .value
                            .insert_str(cursor.pos, display_cursor.as_str());
                    } else if cursor.section > 0 {
                        text.sections[cursor.section].value.remove(cursor.pos);

                        cursor.section -= 1;
                        if text.sections[cursor.section].value.is_empty() {
                            text.sections[cursor.section].value.push_str(display_cursor.as_str());
                            cursor.pos = 0;
                        } else {
                            let last = text.sections[cursor.section].value.len() - 1;
                            text.sections[cursor.section]
                                .value
                                .insert_str(last, display_cursor.as_str());
                            cursor.pos = last;
                        }
                    }
                }
                Key::ArrowRight => {
                    if cursor.pos < text.sections[cursor.section].value.len() - 1 {
                        text.sections[cursor.section].value.remove(cursor.pos);

                        cursor.pos += 1;
                        text.sections[cursor.section]
                            .value
                            .insert_str(cursor.pos, display_cursor.as_str());
                    } else if cursor.section < text.sections.len() - 1 {
                        text.sections[cursor.section].value.remove(cursor.pos);

                        cursor.section += 1;
                        if text.sections[cursor.section].value.is_empty() {
                            text.sections[cursor.section].value.push_str(display_cursor.as_str());
                            cursor.pos = 0;
                        } else {
                            text.sections[cursor.section]
                                .value
                                .insert_str(1, display_cursor.as_str());
                            cursor.pos = 1;
                        }
                    }
                }
                Key::Home => {
                    text.sections[cursor.section].value.remove(cursor.pos);

                    cursor.section = 0;
                    cursor.pos = 0;
                    text.sections[0].value.insert_str(0, display_cursor.as_str());
                }
                Key::End => {
                    text.sections[cursor.section].value.remove(cursor.pos);

                    cursor.section = text.sections.len() - 1;
                    cursor.pos = text.sections[cursor.section].value.len();
                    text.sections[cursor.section].value.push_str(display_cursor.as_str());
                }
                _ => continue,
            }
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
