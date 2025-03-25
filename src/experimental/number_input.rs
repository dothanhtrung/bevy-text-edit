use crate::{TextEditable, TextEdited};
use bevy::prelude::{
    default, AlignItems, BuildChildren, Button, ChildBuild, ChildBuilder, Click, Color, Commands, Component, Deref,
    DerefMut, Entity, Event, JustifyContent, JustifyItems, JustifyText, Node, Parent, Pointer, Query, Text, TextColor,
    TextFont, TextLayout, Trigger, UiRect, Val,
};
use bevy::ui::{AlignContent, BackgroundColor, FlexDirection};
use bevy_support_misc::ui::button::ButtonColorEffect;
use std::cmp::{max, min};

#[derive(Component)]
struct NumberInput {
    max: i64,
    min: i64,
}

#[derive(Component, Deref, DerefMut)]
#[require(Button)]
struct NumberButton(Option<Entity>);

#[derive(Default)]
pub struct NumberInputSetting {
    pub min: i64,
    pub max: i64,
    pub text_bg: Color,
    pub btn_bg: Color,
    pub text_font: TextFont,
    pub text_color: Color,
    pub width: Val,
    pub height: Val,
}

#[derive(Event, Deref, DerefMut)]
pub struct NumberInputChanged(pub i64);

pub fn spawn_number_input_text(builder: &mut ChildBuilder, number: i64, setting: NumberInputSetting) -> Entity {
    builder
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center,
            width: setting.width,
            height: setting.height,
            ..default()
        })
        .with_children(|builder| {
            let mut id = None;
            builder
                .spawn((
                    Node {
                        width: Val::Percent(80.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::End,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor::from(setting.text_bg),
                ))
                .with_children(|builder| {
                    let max_length = max(setting.max.to_string().len(), setting.min.to_string().len());
                    id = Some(
                        builder
                            .spawn((
                                Node {
                                    width: Val::Percent(100.),
                                    ..default()
                                },
                                TextLayout::new_with_justify(JustifyText::Right),
                                Text::new(number.to_string()),
                                TextEditable {
                                    filter_in: vec!["[0-9.-]".to_string()],
                                    max_length,
                                    ..default()
                                },
                                TextColor::from(setting.text_color),
                                setting.text_font.clone(),
                                NumberInput {
                                    max: setting.max,
                                    min: setting.min,
                                },
                            ))
                            .observe(change_value)
                            .id(),
                    );
                });

            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    height: Val::Percent(100.),
                    aspect_ratio: Some(1.0),
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn((
                            ButtonColorEffect::default(),
                            NumberButton(id),
                            BackgroundColor::from(setting.btn_bg),
                            Node {
                                height: Val::Percent(48.),
                                width: Val::Percent(100.),
                                margin: UiRect::bottom(Val::Percent(4.)),
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                ..default()
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn((Text::new("+".to_string()), setting.text_font.clone()));
                        })
                        .observe(increase);
                    builder
                        .spawn((
                            ButtonColorEffect::default(),
                            NumberButton(id),
                            BackgroundColor::from(setting.btn_bg),
                            Node {
                                height: Val::Percent(48.),
                                width: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                ..default()
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn((Text::new("-".to_string()), setting.text_font));
                        })
                        .observe(reduce);
                });
        })
        .id()
}

fn change_value(
    trigger: Trigger<TextEdited>,
    mut query: Query<(&mut Text, &NumberInput)>,
    parent_query: Query<&Parent>,
    commands: Commands,
) {
    let e = trigger.entity();
    let edited_text = trigger.text.clone();
    if let Ok((mut text, setting)) = query.get_mut(e) {
        if let Ok(num) = edited_text.parse::<i64>() {
            let new_num = max(min(setting.max, num), setting.min);
            **text = new_num.to_string();

            number_input_notify(commands, parent_query, e, new_num);
        }
    }
}

fn increase(
    trigger: Trigger<Pointer<Click>>,
    mut text_query: Query<(&mut Text, &NumberInput)>,
    button_query: Query<&NumberButton>,
    parent_query: Query<&Parent>,
    commands: Commands,
) {
    if let Ok(NumberButton(Some(e))) = button_query.get(trigger.entity()) {
        if let Ok((mut text, setting)) = text_query.get_mut(*e) {
            if let Ok(num) = text.parse::<i64>() {
                let new_num = min(setting.max, num + 1);
                **text = new_num.to_string();

                number_input_notify(commands, parent_query, *e, new_num);
            }
        }
    }
}

fn reduce(
    trigger: Trigger<Pointer<Click>>,
    mut text_query: Query<(&mut Text, &NumberInput)>,
    button_query: Query<&NumberButton>,
    parent_query: Query<&Parent>,
    commands: Commands,
) {
    if let Ok(NumberButton(Some(e))) = button_query.get(trigger.entity()) {
        if let Ok((mut text, setting)) = text_query.get_mut(*e) {
            if let Ok(num) = text.parse::<i64>() {
                let new_num = max(setting.min, num - 1);
                **text = new_num.to_string();

                number_input_notify(commands, parent_query, *e, new_num);
            }
        }
    }
}

fn number_input_notify(mut commands: Commands, parent_query: Query<&Parent>, e: Entity, new_num: i64) {
    if let Ok(parent) = parent_query.get(e) {
        if let Ok(grand_parent) = parent_query.get(**parent) {
            commands.trigger_targets(NumberInputChanged(new_num), **grand_parent);
        }
    }
}
