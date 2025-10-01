use bevy::prelude::*;
use bevy_text_edit::virtual_keyboard::{VirtualKeyboardChanged, VirtualKeyboardTheme, VirtualKeysList};
use bevy_text_edit::{TextEditConfig, TextEditPluginAnyState, TextEditable};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextEditPluginAnyState::any()))
        .add_systems(Startup, (setup, customize_virtual_keyboard))
        .run();
}

fn setup(mut commands: Commands, mut config: ResMut<TextEditConfig>) {
    commands.spawn(Camera2d::default());

    // Enable virtual keyboard
    config.enable_virtual_keyboard = true;

    commands.spawn(TextEditable {
        placeholder: "Enter number here...".to_string(),
        filter_in: vec!["[0-9]".into()],
        ..default()
    });
}

fn customize_virtual_keyboard(
    mut virtual_keyboard_changed: MessageWriter<VirtualKeyboardChanged>,
    mut theme: ResMut<VirtualKeyboardTheme>,
    mut keys_list: ResMut<VirtualKeysList>,
) {
    // Appearance can be customized through VirtualKeyboardTheme
    theme.bg_color = Color::BLACK;
    theme.width = Val::Percent(30.);
    theme.height = Val::Percent(30.);
    theme.key_size_1u = Val::Percent(30.);

    // The key list can be changed through VirtualKeysList
    *keys_list = VirtualKeysList::from(vec![
        vec![
            (("1", ""), KeyCode::Digit1, None, 1.),
            (("2", ""), KeyCode::Digit2, None, 1.),
            (("3", ""), KeyCode::Digit3, None, 1.),
        ],
        vec![
            (("4", ""), KeyCode::Digit4, None, 1.),
            (("5", ""), KeyCode::Digit5, None, 1.),
            (("6", ""), KeyCode::Digit6, None, 1.),
        ],
        vec![
            (("7", ""), KeyCode::Digit7, None, 1.),
            (("8", ""), KeyCode::Digit8, None, 1.),
            (("9", ""), KeyCode::Digit9, None, 1.),
        ],
        vec![(("0", ""), KeyCode::Digit0, None, 1.)],
    ]);

    // Noti event to respawn virtual keyboard
    virtual_keyboard_changed.write(VirtualKeyboardChanged);
}
