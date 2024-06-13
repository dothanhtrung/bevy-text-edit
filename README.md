bevy_text_edit
==============

[![crates.io](https://img.shields.io/crates/v/bevy_text_edit)](https://crates.io/crates/bevy_text_edit)
[![docs.rs](https://docs.rs/bevy_text_edit/badge.svg)](https://docs.rs/bevy_text_edit)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit)
[![pipeline status](https://gitlab.com/kimtinh/bevy-text-edit/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy-text-edit/-/commits/master)


Quickstart
----------

### Plugin

Add plugin `EditTextPlugin` to the app and define which states it will run in:

```rust
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(EditTextPlugin::default().in_state(vec![GameState::Menu]))
        .run;
}
```

If you don't care to game state and want to always run input text, use `EditTextPluginNoState`:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(EditTextPluginNoState)
        .add_systems(Startup, setup)
        .run();
}
```

### Component

Insert component `TextEditable` and `Interaction` into any text entity that needs to be editable:

```rust
commands.spawn((
    TextEditable, // Mark text is editable
    Interaction::None, // Mark entity is interactable
    TextBundle::from_section(
        "Input Text 1",
        TextStyle {
            font_size: 20.,
            ..default()
        },
    ),
));
```

Only text that is focused by clicking is get keyboard input.  
If you want to make a text field editable by default, insert component `TextEditFocus` to it when spawn:

```rust
commands.spawn((
    TextEditFocus, // Focus edit text on this entity
    TextEditable, // Mark text is editable
    Interaction::None, // Mark entity is interactable
    TextBundle::from_section(
        "Input Text 2",
        TextStyle {
            font_size: 20.,
            ..default()
        },
    ),
));
```

License
-------

Please see [LICENSE](./LICENSE).


Compatible Bevy Versions
------------------------

| bevy | bevy_text_edit         |
|------|------------------------|
| 0.13 | 0.0.1, branch `master` |
