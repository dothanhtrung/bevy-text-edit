bevy_text_edit
==============

[![crates.io](https://img.shields.io/crates/v/bevy_edit_text)](https://crates.io/crates/bevy_edit_text)
[![docs.rs](https://docs.rs/bevy_edit_text/badge.svg)](https://docs.rs/bevy_edit_text)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-edit-text/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-edit-text)
[![pipeline status](https://gitlab.com/kimtinh/bevy-edit-text/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy-edit-text/-/commits/master)


Quickstart
----------

Add plugin `EditTextPlugin` to the app:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(EditTextPlugin)
        .run;
}
```

Insert component `TextEditable` and `Interaction` into any text entity that needs to be editable.
```rust
commands.spawn((
    // Add the component
    TextEditable,
    Interaction::None,
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
    TextEditFocus,
    TextEditable,
    Interaction::None,
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

| bevy | bevy_edit_text         |
|------|------------------------|
| 0.13 | 0.0.1, branch `master` |
