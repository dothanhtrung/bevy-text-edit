#![doc=include_str!("../README.md")]

#[cfg(feature = "experimental")]
pub mod experimental;
pub mod virtual_keyboard;

use crate::virtual_keyboard::{
    VirtualKey,
    VirtualKeyboard,
    VirtualKeyboardPlugin,
    VirtualKeyboardPos,
};
#[cfg(feature = "clipboard")]
use arboard::Clipboard;
use bevy::app::{
    App,
    Plugin,
    Update,
};
use bevy::ecs::observer::On;
use bevy::input::ButtonState;
use bevy::input::keyboard::{
    Key,
    KeyboardInput,
};
use bevy::input::mouse::MouseButton;
use bevy::input::touch::Touches;
#[cfg(feature = "log")]
use bevy::log::error;
use bevy::picking::events::PointerPress;
use bevy::prelude::{
    Alpha,
    ButtonInput,
    Commands,
    Component,
    Deref,
    DerefMut,
    Entity,
    EntityEvent,
    GlobalTransform,
    Ime,
    IntoScheduleConfigs,
    KeyCode,
    Message,
    MessageReader,
    MessageWriter,
    Query,
    Res,
    ResMut,
    Resource,
    States,
    Text,
    Time,
    Timer,
    TimerMode,
    With,
    Without,
    in_state,
};
use bevy::text::TextColor;
use bevy::ui::Pressed;
use bevy::ui_widgets::Button;
use regex_lite::Regex;

macro_rules! plugin_systems {
    ( ) => {
        (
            click_outside_to_unfocus,
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
            .add_message::<TextFocusChanged>()
            .add_message::<TextEdited>();

        #[cfg(feature = "clipboard")]
        app.insert_resource(ClipboardMng::new());

        app.add_observer(listen_changing_focus);

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
#[derive(Message)]
pub enum TextFocusChanged {
    /// Show on top or bottom depend on the global_y of focused text entity
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
#[require(Text, GlobalTransform, Button)]
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

#[derive(Message, EntityEvent, Clone)]
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

fn submit_and_unfocus(
    commands: &mut Commands,
    focus_entity: Entity,
    cursor: &CursorPosition,
    text: &mut Text,
    text_editable: &TextEditable,
    ignore_entity: Option<Entity>,
    text_edited_event: &mut MessageWriter<TextEdited>,
) {
    if ignore_entity.is_none() || focus_entity != ignore_entity.unwrap() {
        commands.entity(focus_entity).remove::<TextEditFocus>();

        if text.len() > cursor.pos {
            text.remove(cursor.pos);
        }
        commands.entity(focus_entity).remove::<CursorPosition>();
        commands.entity(focus_entity).remove::<TextEditFocus>();

        let edited_text = if text_editable.is_placeholder_shown { String::new() } else { text.0.clone() };

        let text_edited = TextEdited {
            text: edited_text,
            entity: focus_entity,
        };
        text_edited_event.write(text_edited.clone());
        commands.trigger(text_edited);
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

pub fn click_outside_to_unfocus(
    mut commands: Commands,
    text_editable_interact: Query<Entity, (With<TextEditable>, With<Pressed>)>,
    virtual_key_interact: Query<Entity, (With<VirtualKey>, With<Pressed>, Without<TextEditable>)>,
    virtual_keyboard_interact: Query<
        Entity,
        (
            With<VirtualKeyboard>,
            With<Pressed>,
            Without<VirtualKey>,
            Without<TextEditable>,
        ),
    >,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut focusing_texts: Query<(Entity, &CursorPosition, &mut Text, &TextEditable), With<TextEditFocus>>,
    mut text_edited_event: MessageWriter<TextEdited>,
    mut focus_event: MessageWriter<TextFocusChanged>,
) {
    if (mouse_input.just_pressed(MouseButton::Left) || touches.any_just_pressed())
        && text_editable_interact.is_empty()
        && virtual_key_interact.is_empty()
        && virtual_keyboard_interact.is_empty()
    {
        for (focus_entity, cursor, mut text, text_editable) in focusing_texts.iter_mut() {
            submit_and_unfocus(
                &mut commands,
                focus_entity,
                cursor,
                &mut text,
                text_editable,
                None,
                &mut text_edited_event,
            );
        }
        focus_event.write(TextFocusChanged::Hide);
    }
}

pub fn listen_changing_focus(
    trigger: On<PointerPress>,
    mut commands: Commands,
    text_interactions: Query<&GlobalTransform, With<TextEditable>>,
    mut focusing_texts: Query<(Entity, &CursorPosition, &mut Text, &TextEditable), With<TextEditFocus>>,
    mut text_edited_event: MessageWriter<TextEdited>,
    mut focus_event: MessageWriter<TextFocusChanged>,
) {
    if let Ok(global_transform) = text_interactions.get(trigger.entity) {
        let mut focusing_list = Vec::new();
        for (focusing_e, _, _, _) in focusing_texts.iter() {
            focusing_list.push(focusing_e);
        }

        // Unfocus all text box except which is currently clicked on
        for (focus_entity, cursor, mut text, text_editable) in focusing_texts.iter_mut() {
            submit_and_unfocus(
                &mut commands,
                focus_entity,
                cursor,
                &mut text,
                text_editable,
                Some(trigger.entity),
                &mut text_edited_event,
            );
        }

        if !focusing_list.contains(&trigger.entity) {
            commands.entity(trigger.entity).insert(TextEditFocus);
        }
        focus_event.write(TextFocusChanged::Show(global_transform.translation().y));
    }
}

fn listen_keyboard_input(
    mut commands: Commands,
    mut events: MessageReader<KeyboardInput>,
    mut ime_reader: MessageReader<Ime>,
    mut edit_text: Query<(Entity, &mut CursorPosition, &mut Text, &TextEditable), With<TextEditFocus>>,
    display_cursor: Res<DisplayTextCursor>,
    #[cfg(feature = "clipboard")] mut clipboard_mng: ResMut<ClipboardMng>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_edited_event: MessageWriter<TextEdited>,
    mut focus_event: MessageWriter<TextFocusChanged>,
) {
    let is_ctrl_pressed = keyboard_input.pressed(KeyCode::ControlRight) || keyboard_input.pressed(KeyCode::ControlLeft);
    for (focus_entity, mut cursor, mut text, texteditable) in edit_text.iter_mut() {
        let ignore_list = &texteditable.filter_out;
        let allow_list = &texteditable.filter_in;
        for ime in ime_reader.read() {
            match ime {
                Ime::Preedit { .. } => {}
                Ime::Commit { value, .. } => {
                    let mut append_text = String::new();
                    for char in value.chars() {
                        if !is_ignored(ignore_list, allow_list, char.to_string())
                            && (texteditable.max_length == 0
                                || (text.len() + append_text.len()) < texteditable.max_length)
                        {
                            append_text.push(char);
                        }
                    }

                    text.insert_str(cursor.pos, append_text.as_str());
                    cursor.pos += append_text.len();
                }
                Ime::Enabled { .. } => {}
                Ime::Disabled { .. } => {}
            }
        }

        for event in events.read() {
            if event.state == ButtonState::Released {
                continue;
            }

            match &event.logical_key {
                Key::Enter | Key::Escape => {
                    submit_and_unfocus(
                        &mut commands,
                        focus_entity,
                        &cursor,
                        &mut text,
                        texteditable,
                        None,
                        &mut text_edited_event,
                    );
                    focus_event.write(TextFocusChanged::Hide);
                }
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
