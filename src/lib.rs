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
//! If you don't care to game state and want to always run input text, use `EditTextPluginNoState`:
//! ```rust
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Add the plugin
//!         .add_plugins(TextEditPluginNoState)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//! ```
//!
//! ### Component
//!
//! Insert component `TextEditable` and `Interaction` into any text entity that needs to be editable.
//! ```rust
//! commands.spawn((
//!     TextEditable, // Mark text is editable
//!     Interaction::None, // Mark entity is interactable
//!     TextBundle::from_section(
//!         "Input Text 1",
//!         TextStyle {
//!             font_size: 20.,
//!             ..default()
//!         },
//!     ),
//! ));
//! ```
//!
//! Only text that is focused by clicking gets keyboard input.

use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{
    ButtonInput, Changed, Commands, Component, Deref, DerefMut, Entity, Event, EventReader, EventWriter, in_state,
    IntoSystemConfigs, MouseButton, Query, Res, Resource, States, Text, With, Without,
};
use bevy::ui::Interaction;

macro_rules! add_systems {
    ($app: expr, $states: expr) => {
        $app.insert_resource(DisplayTextCursor(DEFAULT_CURSOR.to_string()))
            .add_event::<TextFocusEvent>();
        if let Some(states) = $states {
            for state in states {
                $app.add_systems(
                    Update,
                    (
                        listen_interaction,
                        listen_keyboard_input,
                        focus_text_box.after(listen_interaction),
                    )
                        .run_if(in_state(state.clone())),
                );
            }
        } else {
            $app.add_systems(
                Update,
                (
                    listen_interaction,
                    listen_keyboard_input,
                    focus_text_box.after(listen_interaction),
                ),
            );
        }
    };
}

const DEFAULT_CURSOR: &str = "|";

/// Current position of cursor in the text
#[derive(Component, Default, Deref, DerefMut)]
pub struct CursorPosition(usize);

/// The text that will be displayed as cursor. Default is `|`.
#[derive(Resource, Deref, DerefMut)]
pub struct DisplayTextCursor(String);

#[derive(Default)]
pub struct TextEditPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in
    pub states: Option<Vec<T>>,
}

impl<T> Plugin for TextEditPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        add_systems!(app, &self.states);
    }
}

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
        let non: Option<Vec<FakeGameState>> = None;
        add_systems!(app, non);
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum FakeGameState {
    #[default]
    NA,
}

/// Mark a text entity is focused. Normally done by mouse click.
#[derive(Component)]
pub struct TextEditFocus;

/// Mark a text is editable.
#[derive(Component)]
pub struct TextEditable;

#[derive(Event, Deref, DerefMut)]
struct TextFocusEvent(Entity);

fn unfocus_text_box(
    commands: &mut Commands,
    text_focus: &mut Query<(Entity, &CursorPosition, &mut Text), With<TextEditFocus>>,
    ignore_entity: Option<Entity>,
) {
    for (e, pos, mut text) in text_focus.iter_mut() {
        if ignore_entity.is_none() || e != ignore_entity.unwrap() {
            commands.entity(e).remove::<TextEditFocus>();

            if text.sections[0].value.len() > **pos {
                text.sections[0].value.remove(**pos);
            }
            commands.entity(e).remove::<CursorPosition>();
            commands.entity(e).remove::<TextEditFocus>();
        }
    }
}

fn focus_text_box(
    mut commands: Commands,
    mut texts: Query<(&mut Text, Entity), (With<TextEditFocus>, Without<CursorPosition>)>,
    display_cursor: Res<DisplayTextCursor>,
    mut event_reader: EventReader<TextFocusEvent>,
) {
    for e in event_reader.read() {
        for (mut text, text_e) in texts.iter_mut() {
            if **e == text_e {
                commands
                    .entity(**e)
                    .insert(CursorPosition(text.sections[0].value.len()));
                text.sections[0].value.push_str(display_cursor.as_str());
            }
        }
    }
}

fn listen_interaction(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut interactions: Query<(&Interaction, Entity), (Changed<Interaction>, With<TextEditable>)>,
    mut focusing_texts: Query<(Entity, &CursorPosition, &mut Text), With<TextEditFocus>>,
    mut event_writer: EventWriter<TextFocusEvent>,
) {
    if interactions.is_empty() && input.just_pressed(MouseButton::Left) {
        unfocus_text_box(&mut commands, &mut focusing_texts, None);
        return;
    }

    for (interaction, e) in interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            let mut focusing_list = Vec::new();
            for (focusing_e, _, _) in focusing_texts.iter() {
                focusing_list.push(focusing_e);
            }

            unfocus_text_box(&mut commands, &mut focusing_texts, Some(e));
            if !focusing_list.contains(&e) {
                commands.entity(e).insert(TextEditFocus);
                event_writer.send(TextFocusEvent(e));
            }
        }
    }
}

fn listen_keyboard_input(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<(&mut Text, &mut CursorPosition), With<TextEditFocus>>,
    display_cursor: Res<DisplayTextCursor>,
) {
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if event.state == ButtonState::Released {
            continue;
        }

        for (mut text, mut pos) in edit_text.iter_mut() {
            if text.sections.len() > 0 {
                let (first, second) = text.sections[0].value.split_at(**pos);
                let mut first = String::from(first);
                let mut second = String::from(second);
                match &event.logical_key {
                    Key::Space => {
                        first.push(' ');
                        **pos += 1;
                    }
                    Key::Backspace => {
                        if **pos > 0 {
                            first.pop();
                            **pos -= 1;
                        }
                    }
                    Key::Character(character) => {
                        first.push_str(character);
                        **pos += character.len();
                    }
                    Key::ArrowLeft => {
                        if **pos > 0 {
                            if let Some(c) = first.pop() {
                                second.insert(1, c);
                            }
                            **pos -= 1;
                        }
                    }
                    Key::ArrowRight => {
                        if **pos < text.sections[0].value.len() - 1 {
                            let c = second.remove(1);
                            first.push(c);
                            **pos += 1;
                        }
                    }
                    Key::Home => {
                        if **pos > 0 && **pos < text.sections[0].value.len() {
                            text.sections[0].value.remove(**pos);
                            first.clone_from(&(**display_cursor));
                            second.clone_from(&text.sections[0].value);
                            **pos = 0;
                        }
                    }
                    Key::End => {
                        if **pos < text.sections[0].value.len() - 1 {
                            text.sections[0].value.remove(**pos);
                            first.clone_from(&text.sections[0].value);
                            second.clone_from(&(**display_cursor));
                            **pos = text.sections[0].value.len();
                        }
                    }
                    _ => continue,
                }
                text.sections[0].value = format!("{}{}", first, second);
            }
        }
    }
}
