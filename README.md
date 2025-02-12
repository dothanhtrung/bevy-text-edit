bevy_text_edit
==============

[![crates.io](https://img.shields.io/crates/v/bevy_text_edit)](https://crates.io/crates/bevy_text_edit)
[![docs.rs](https://docs.rs/bevy_text_edit/badge.svg)](https://docs.rs/bevy_text_edit)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit)
[![pipeline status](https://gitlab.com/kimtinh/bevy-text-edit/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy-text-edit/-/commits/master)

![](examples/text_edit.gif)

Quickstart
----------

### Plugin

Add plugin `TextEditPlugin` to the app and define which states it will run in:

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
        .add_plugins(TextEditPlugin::new(vec![GameState::Menu]))
        .run;
}
```

If you don't care to game state and want to always run input text, use `TextEditPluginNoState`:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(TextEditPluginNoState)
        .add_systems(Startup, setup)
        .run();
}
```

### Component

Insert component `TextEditable` into any text entity that needs to be editable:

```rust
commands.spawn((
    TextEditable::default(), // Mark text is editable
    Text::new("Input Text 1"),
));
```

Only text that is focused by clicking gets keyboard input.


It is also possible to limit which characters are allowed to enter through `filter_in` and `filter_out` attribute
(regex is supported):

```rust
commands.spawn((
    TextEditable {
        filter_in: vec!["[0-9]".into(), " ".into()], // Only allow number and space
        filter_out: vec!["5".into()],                // Ignore number 5
    },
    Text::new("Input Text 1"),
));
```

### Get text

The edited text can be retrieved from event `TextEdited`.
```rust
fn get_text(
    mut event: EventReader<TextEdited>,
) {
    for e in event.read() {
        info!("Entity {}: {}", e.entity, e.text);
    }
}
```

License
-------

Please see [LICENSE](./LICENSE).


Compatible Bevy Versions
------------------------

| bevy | bevy_text_edit       |
|------|----------------------|
| 0.15 | 0.4, branch `master` |
| 0.14 | 0.1-0.3              |
| 0.13 | 0.0.1-0.0.5          |
