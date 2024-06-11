// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//!
//! Add plugin `EditTextPlugin` to the app and define which states it will run in:
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
//!         .add_plugins(EditTextPluginNoState)
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
//! Only text that is focused by clicking is get keyboard input.
//! If you want to make a text field editable by default, insert component `TextEditFocus` to it when spawn:
//! ```rust
//! commands.spawn((
//!     TextEditFocus, // Focus on this entity
//!     TextEditable, // Mark text is editable
//!     Interaction::None, // Mark entity is interactable
//!     TextBundle::from_section(
//!         "Input Text 2",
//!         TextStyle {
//!             font_size: 20.,
//!             ..default()
//!         },
//!     ),
//! ));
//! ```

use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{
    Changed, Commands, Component, Deref, DerefMut, Entity, EventReader, in_state, IntoSystemConfigs, Query, Resource,
    States, Text, With,
};
use bevy::ui::Interaction;

const DEFAULT_CURSOR: &str = "|";

#[derive(Resource, Deref, DerefMut)]
pub struct TextCursor(String);

#[derive(Default)]
pub struct TextEditPlugin<T>
where
    T: States,
{
    pub states: Option<Vec<T>>,
}

impl<T> Plugin for TextEditPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(TextCursor(DEFAULT_CURSOR.to_string()));

        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(
                    Update,
                    (change_focus, listen_keyboard_input).run_if(in_state(state.clone())),
                );
            }
        } else {
            app.add_systems(Update, (change_focus, listen_keyboard_input));
        }
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

#[derive(Default)]
pub struct TextEditPluginNoState;

impl Plugin for TextEditPluginNoState {
    fn build(&self, app: &mut App) {
        app.insert_resource(TextCursor(DEFAULT_CURSOR.to_string()));
        app.add_systems(Update, (change_focus, listen_keyboard_input));
    }
}

#[derive(Component)]
pub struct TextEditFocus;

#[derive(Component)]
pub struct TextEditable;

fn change_focus(
    mut commands: Commands,
    interactions: Query<(&Interaction, Entity), (Changed<Interaction>, With<TextEditable>)>,
    text_focus: Query<Entity, With<TextEditFocus>>,
) {
    for (interaction, e) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            for old_e in text_focus.iter() {
                commands.entity(old_e).remove::<TextEditFocus>();
            }
            commands.entity(e).insert(TextEditFocus);
        }
    }
}

fn listen_keyboard_input(mut events: EventReader<KeyboardInput>, mut edit_text: Query<&mut Text, With<TextEditFocus>>) {
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if event.state == ButtonState::Released {
            continue;
        }

        for mut text in edit_text.iter_mut() {
            if text.sections.len() > 0 {
                match &event.logical_key {
                    Key::Space => {
                        text.sections[0].value.push(' ');
                    }
                    Key::Backspace => {
                        text.sections[0].value.pop();
                    }
                    Key::Character(character) => {
                        text.sections[0].value.push_str(character);
                    }
                    _ => continue,
                }
            }
        }
    }
}
