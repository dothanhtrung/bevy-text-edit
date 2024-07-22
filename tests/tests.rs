use bevy::input::{ButtonState, InputPlugin};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_text_edit::{TextEditable, TextEditFocus};

#[cfg(feature = "state")]
use bevy_text_edit::TextEditPlugin;

#[cfg(not(feature = "state"))]
use bevy_text_edit::TextEditPluginNoState;

const T1_S1: &str = "Text1_Section1";
const T1_S2: &str = "Text1_Section2";
const T2_S1: &str = "Text2_Section1";
const T2_S2: &str = "Text2_Section2";

#[test]
fn arrow() {
    let (mut app, text1_e, _) = setup(vec![], vec![]);

    // 1 Arrow left
    send_key(app.world_mut(), KeyCode::ArrowLeft, Key::ArrowLeft);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section|2".to_string());

    // Arrow left to other section
    for _ in 0..T1_S2.len() {
        send_key(app.world_mut(), KeyCode::ArrowLeft, Key::ArrowLeft);
    }
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section|1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section2".to_string());

    // Arrow right to other section
    send_key(app.world_mut(), KeyCode::ArrowRight, Key::ArrowRight);
    send_key(app.world_mut(), KeyCode::ArrowRight, Key::ArrowRight);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "T|ext1_Section2".to_string());
}

#[test]
fn backspace() {
    let (mut app, text1_e, _) = setup(vec![], vec![]);

    // 1 Backspace
    send_key(app.world_mut(), KeyCode::Backspace, Key::Backspace);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section|".to_string());

    // Backspace to other section
    for _ in 0..T1_S2.len() {
        send_key(app.world_mut(), KeyCode::Backspace, Key::Backspace);
    }
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section|".to_string());
    assert!(text1.sections[1].value.is_empty());
}

#[test]
fn input_text() {
    let (mut app, text1_e, _) = setup(vec![], vec![]);

    send_key(app.world_mut(), KeyCode::Space, Key::Space);
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section2 a|".to_string());
}

#[test]
fn delete() {
    let (mut app, text1_e, _) = setup(vec![], vec![]);

    // Delete 1
    send_key(app.world_mut(), KeyCode::ArrowLeft, Key::ArrowLeft);
    send_key(app.world_mut(), KeyCode::Delete, Key::Delete);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section|".to_string());

    // Delete to other section
    for _ in 0..T1_S2.len() {
        send_key(app.world_mut(), KeyCode::ArrowLeft, Key::ArrowLeft);
    }
    send_key(app.world_mut(), KeyCode::Delete, Key::Delete);
    send_key(app.world_mut(), KeyCode::Delete, Key::Delete);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section".to_string());
    assert_eq!(text1.sections[1].value, "|ext1_Section".to_string());
}

#[test]
fn home_end() {
    let (mut app, text1_e, _) = setup(vec![], vec![]);

    // Home
    send_key(app.world_mut(), KeyCode::Home, Key::Home);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "|Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section2".to_string());

    // Backspace at the beginning
    send_key(app.world_mut(), KeyCode::Backspace, Key::Backspace);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "|Text1_Section1".to_string());

    // End
    send_key(app.world_mut(), KeyCode::End, Key::End);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section2|".to_string());

    // Delete at the end
    send_key(app.world_mut(), KeyCode::Delete, Key::Delete);
    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[1].value, "Text1_Section2|".to_string());
}

#[test]
fn ignore_char_test() {
    let (mut app, text1_e, _) = setup(vec!["a".into(), " ".into(), "[1-3]".into()], vec![]);

    send_key(app.world_mut(), KeyCode::Space, Key::Space);
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("b".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("2".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("4".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section2b4|".to_string());
}

#[test]
fn allow_char_test() {
    let (mut app, text1_e, _) = setup(vec![], vec!["a".into(), " ".into(), "[1-3]".into()]);

    send_key(app.world_mut(), KeyCode::Space, Key::Space);
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("a".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("b".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("0".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("1".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("3".into()));
    send_key(app.world_mut(), KeyCode::KeyA, Key::Character("4".into()));

    app.update();
    let text1 = app.world().get::<Text>(text1_e).unwrap();
    assert_eq!(text1.sections[0].value, "Text1_Section1".to_string());
    assert_eq!(text1.sections[1].value, "Text1_Section2 a13|".to_string());
}

#[cfg(feature = "state")]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn setup(ignore: Vec<String>, allow: Vec<String>) -> (App, Entity, Entity) {
    let mut app = App::new();
    let mut text1 = Entity::from_raw(0);
    let mut text2 = Entity::from_raw(0);

    #[cfg(feature = "state")]
    app.add_plugins((
        WindowPlugin::default(),
        InputPlugin,
        StatesPlugin,
        TextEditPlugin::new(vec![GameState::Menu]),
    ))
    .init_state::<GameState>();

    #[cfg(not(feature = "state"))]
    app.add_plugins((DefaultPlugins, TextEditPluginNoState));

    let text1_section1 = TextSection {
        value: T1_S1.into(),
        ..default()
    };
    let text1_section2 = TextSection {
        value: T1_S2.into(),
        ..default()
    };
    let text2_section1 = TextSection {
        value: T2_S1.into(),
        ..default()
    };
    let text2_section2 = TextSection {
        value: T2_S2.into(),
        ..default()
    };

    app.world_mut()
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            text1 = parent
                .spawn((
                    TextEditable { ignore, allow },
                    TextEditFocus,
                    Interaction::None,
                    TextBundle::from_sections(vec![text1_section1, text1_section2]),
                ))
                .id();

            text2 = parent
                .spawn((
                    TextEditable::default(),
                    Interaction::None,
                    TextBundle::from_sections(vec![text2_section1, text2_section2]),
                ))
                .id();
        });

    (app, text1, text2)
}

fn send_key(world: &mut World, key_code: KeyCode, logical_key: Key) {
    let mut window = world.query::<(Entity, &Window)>();
    let (window, _) = window.single(world);

    world.resource_mut::<Events<KeyboardInput>>().send(KeyboardInput {
        key_code,
        logical_key,
        state: ButtonState::Pressed,
        window,
    });
}
