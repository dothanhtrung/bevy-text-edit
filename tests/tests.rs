use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::{ButtonState, InputPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimePlugin;
use bevy_text_edit::{TextEditFocus, TextEditPluginAnyState, TextEditable, TextEdited};

const TEXT_1: &str = "Text_Section1";
const TEXT_2: &str = "Text_Section2";

#[test]
fn arrow() {
    let (mut app, text1_e, _) = setup(vec![], vec![], 0);

    // 1 Arrow left
    send_key(app.world_mut(), KeyCode::ArrowLeft, Key::ArrowLeft);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section|1".to_string());

    // 1 Arrow right
    send_key(app.world_mut(), KeyCode::ArrowRight, Key::ArrowRight);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section1|".to_string());
}

#[test]
fn backspace() {
    let (mut app, text1_e, _) = setup(vec![], vec![], 0);

    // 1 Backspace
    send_key(app.world_mut(), KeyCode::Backspace, Key::Backspace);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section|".to_string());
}

#[test]
fn input_text() {
    let (mut app, text1_e, _) = setup(vec![], vec![], 0);

    send_key(app.world_mut(), KeyCode::Space, Key::Space);
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section1 a|".to_string());
}

#[test]
fn delete() {
    let (mut app, text1_e, _) = setup(vec![], vec![], 0);

    // Delete 1
    send_key(app.world_mut(), KeyCode::ArrowLeft, Key::ArrowLeft);
    send_key(app.world_mut(), KeyCode::Delete, Key::Delete);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section|".to_string());

    // Delete at the end of line
    send_key(app.world_mut(), KeyCode::Delete, Key::Delete);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section|".to_string());
}

#[test]
fn home_end() {
    let (mut app, text1_e, _) = setup(vec![], vec![], 0);

    // Home
    send_key(app.world_mut(), KeyCode::Home, Key::Home);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "|Text_Section1".to_string());

    // Backspace at the beginning
    send_key(app.world_mut(), KeyCode::Backspace, Key::Backspace);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "|Text_Section1".to_string());

    // End
    send_key(app.world_mut(), KeyCode::End, Key::End);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section1|".to_string());
}

#[test]
fn ignore_char_test() {
    let (mut app, text1_e, _) = setup(vec!["a".into(), " ".into(), "[1-3]".into()], vec![], 0);

    send_key(app.world_mut(), KeyCode::Space, Key::Space);
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyB, Key::Character("b".into()));
    send_key(app.world_mut(), KeyCode::Digit2, Key::Character("2".into()));
    send_key(app.world_mut(), KeyCode::Digit4, Key::Character("4".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section1b4|".to_string());
}

#[test]
fn allow_char_test() {
    let (mut app, text1_e, _) = setup(vec![], vec!["a".into(), " ".into(), "[1-3]".into()], 0);

    send_key(app.world_mut(), KeyCode::Space, Key::Space);
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyB, Key::Character("b".into()));
    send_key(app.world_mut(), KeyCode::Digit0, Key::Character("0".into()));
    send_key(app.world_mut(), KeyCode::Digit1, Key::Character("1".into()));
    send_key(app.world_mut(), KeyCode::Digit3, Key::Character("3".into()));
    send_key(app.world_mut(), KeyCode::Digit4, Key::Character("4".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section1 a13|".to_string());
}

#[test]
fn max_length() {
    let (mut app, text1_e, _) = setup(vec![], vec![], 15);

    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.0, "Text_Section1aa|".to_string());
}

fn setup(ignore: Vec<String>, allow: Vec<String>, max_length: usize) -> (App, Entity, Entity) {
    let mut app = App::new();
    let mut text1 = Entity::from_raw(0);
    let mut text2 = Entity::from_raw(0);

    app.add_plugins((
        WindowPlugin::default(),
        InputPlugin,
        TimePlugin,
        TextEditPluginAnyState::any(),
    ));

    app.world_mut()
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            text1 = parent
                .spawn((
                    TextEditable {
                        filter_out: ignore,
                        filter_in: allow,
                        max_length,
                        placeholder: String::from("Placeholder"),
                        ..default()
                    },
                    TextEditFocus,
                    Text::new(TEXT_1),
                ))
                .observe(get_text)
                .id();

            text2 = parent
                .spawn((TextEditable::default(), Interaction::None, Text::new(TEXT_2)))
                .id();
        });

    (app, text1, text2)
}

fn get_text(trigger: Trigger<TextEdited>) {
    info!("{}", trigger.text);
}

fn send_key(world: &mut World, key_code: KeyCode, logical_key: Key) {
    let mut window = world.query::<(Entity, &Window)>();
    let (window, _) = window.single(world);

    world.resource_mut::<Events<KeyboardInput>>().send(KeyboardInput {
        key_code,
        logical_key,
        state: ButtonState::Pressed,
        window,
        repeat: false,
    });
}
