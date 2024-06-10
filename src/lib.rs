// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{Changed, Commands, Component, Entity, EventReader, Query, Text, With};
use bevy::ui::Interaction;

pub struct EditTextPlugin;

impl Plugin for EditTextPlugin {
    fn build(&self, app: &mut App) {
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
