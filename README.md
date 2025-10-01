<div align="center">

bevy_text_edit
==============

[![crates.io](https://img.shields.io/crates/v/bevy_text_edit)](https://crates.io/crates/bevy_text_edit)
[![docs.rs](https://docs.rs/bevy_text_edit/badge.svg)](https://docs.rs/bevy_text_edit)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-text-edit)
[![pipeline status](https://gitlab.com/kimtinh/bevy-text-edit/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy-text-edit/-/commits/master)

[![Gitlab](https://img.shields.io/badge/gitlab-%23181717.svg?style=for-the-badge&logo=gitlab&logoColor=white)](https://gitlab.com/kimtinh/bevy-text-edit)
[![Github](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/dothanhtrung/bevy-text-edit)

![](screenshots/text_edit.gif)

![](screenshots/virtual_keyboard.png)

</div>

A very easy-to-use plugin for input text in Bevy. Good enough for game setting and command console.

Features:
* [x] Switchable between multiple text boxes.
* [x] Moving the text cursor using arrow keys and Home/End.
* [x] Limit input length.
* [x] Filter input text with regex.
* [x] Placeholder.
* [x] Paste with `Ctrl+v`.
* [x] In-game virtual keyboard.
  * [x] Repeated key.

Not support:
* [ ] IME.
* In-game virtual keyboard.
  * [ ] Multi-language.
* [ ] Select text.
* [ ] Copy.


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

If you don't care to game state and want to always run input text, use `TextEditPluginAnyState`:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(TextEditPluginAnyState::any())
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

Only text focused by clicking gets keyboard input.

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
// Get by event
fn get_text(
    mut event: MessageReader<TextEdited>,
) {
    for e in event.read() {
        info!("Entity {}: {}", e.entity, e.text);
    }
}
```

```rust
// Get by observing
fn setup(mut commands: Commands) {
    commands.spawn((
        TextEditable::default(),
        Text::new("Input Text"),
    )).observe(get_text);
}

fn get_text(
    trigger: On<TextEdited>,
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

| bevy | bevy_text_edit |
|------|----------------|
| 0.17 | 0.7            |
| 0.16 | 0.6            |
| 0.15 | 0.4-0.5        |
| 0.14 | 0.1-0.3        |
| 0.13 | 0.0.1-0.0.5    |

---------

<div align="center">

![git_bevy-text-edit](https://count.getloli.com/@git_bevy-text-edit?name=git_bevy-text-edit&theme=random&padding=10&offset=0&align=top&scale=1&pixelated=1&darkmode=auto)

</div>
