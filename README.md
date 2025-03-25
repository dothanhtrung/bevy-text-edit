<div align="center">

bevy_text_edit
==============

[![crates.io](https://img.shields.io/crates/v/bevy_text_edit)](https://crates.io/crates/bevy_text_edit)
[![docs.rs](https://docs.rs/bevy_text_edit/badge.svg)](https://docs.rs/bevy_text_edit)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit)
[![pipeline status](https://gitlab.com/kimtinh/bevy-text-edit/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy-text-edit/-/commits/master)

[![Gitlab](https://img.shields.io/badge/gitlab-%23181717.svg?style=for-the-badge&logo=gitlab&logoColor=white)](https://gitlab.com/kimtinh/bevy-text-edit)
[![Github](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/dothanhtrung/bevy-text-edit)

![](examples/text_edit.gif)

![](examples/virtual_keyboard.png)

</div>

A very easy to use plugin for input text in Bevy. Good enough for game setting and command console.

Features:
* [x] Switchable between multiple text boxes.
* [x] Moving text cursor using arrow keys and Home/End.
* [x] Limit input length.
* [x] Filter input text with regex.
* [x] Placeholder.
* [x] Built-in virtual keyboard.

Not support:
* [ ] IME.
* [ ] Multi-language.
* [ ] Select text.
* [ ] Copy/paste.
* [ ] Repeated key.

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
fn setup(mut commands: Commands) {
    commands.spawn((
        TextEditable::default(), // Mark text is editable
        Text::new("Input Text 1"),
    ));
}
```

Only text that is focused by clicking gets keyboard input.

It is also possible to limit which characters are allowed to enter through `filter_in` and `filter_out` attribute
(regex is supported):

```rust
fn setup(mut commands: Commands) {
    commands.spawn((
        TextEditable {
            filter_in: vec!["[0-9]".into(), " ".into()], // Only allow number and space
            filter_out: vec!["5".into()],                // Ignore number 5
            ..default()
        },
        Text::new("Input Text 1"),
    ));
}
```

### Get text

The edited text can be retrieved from event or observe trigger `TextEdited`.

```rust
fn get_text(
    mut event: EventReader<TextEdited>,
) {
    for e in event.read() {
        info!("Entity {}: {}", e.entity, e.text);
    }
}
```

```rust
fn setup(mut commands: Commands) {
    commands.spawn((
        TextEditable::default(),
        Text::new("Input Text"),
    )).observe(get_text);
}

fn get_text(
    trigger: Trigger<TextEdited>,
) {
    let text = trigger.text.as_str();
    info!("{}", text);
}

```

License
-------

Please see [LICENSE](./LICENSE).


Compatible Bevy Versions
------------------------

| bevy | bevy_text_edit           |
|------|--------------------------|
| 0.15 | 0.4-0.5, branch `master` |
| 0.14 | 0.1-0.3                  |
| 0.13 | 0.0.1-0.0.5              |
