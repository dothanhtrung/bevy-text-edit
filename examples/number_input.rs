//! This is experimental, you don't need to care about this

#[cfg(feature = "experimental")]
use bevy::color::palettes::tailwind::{NEUTRAL_600, ZINC_800};
use bevy::prelude::*;
use bevy::window::WindowResolution;
#[cfg(feature = "experimental")]
use bevy_text_edit::experimental::number_input::{spawn_number_input_text, NumberInputChanged, NumberInputSetting};
use bevy_text_edit::TextEditPluginNoState;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(400., 300.),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(TextEditPluginNoState);

    #[cfg(feature = "experimental")]
    app.add_systems(Startup, setup);

    app.run();
}

#[cfg(feature = "experimental")]
#[derive(Component)]
struct Result;

#[cfg(feature = "experimental")]
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut id = None;
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|builder| {
            let setting = NumberInputSetting {
                width: Val::Px(200.),
                height: Val::Px(60.),
                text_bg: ZINC_800.into(),
                btn_bg: NEUTRAL_600.into(),
                max: 100,
                min: -10,
                ..default()
            };

            id = Some(spawn_number_input_text(builder, 1, setting));

            builder.spawn((Text::new("Result:"), Result));
        });

    commands.entity(id.unwrap()).observe(get_result);
}

#[cfg(feature = "experimental")]
fn get_result(trigger: Trigger<NumberInputChanged>, mut query: Query<&mut Text, With<Result>>) {
    for mut text in query.iter_mut() {
        **text = format!("Result: {}", trigger.0);
    }
}
