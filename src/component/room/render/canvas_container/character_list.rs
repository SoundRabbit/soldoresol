use super::super::common;
use super::Msg;
use crate::{
    block::{self, chat::item::Icon, BlockId},
    Resource,
};
use kagura::prelude::*;

pub fn render<'a>(
    block_field: &block::Field,
    characters: impl Iterator<Item = &'a BlockId>,
    resource: &Resource,
) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .class("cover-a")
            .class("flex-v"),
        Events::new(),
        block_field
            .listed::<block::Character>(characters.collect())
            .map(|(_, character)| render_item(block_field, character, resource))
            .collect(),
    )
}

fn render_item(
    block_field: &block::Field,
    character: &block::Character,
    resource: &Resource,
) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("chat-item")
            .class("bg-color-light-t")
            .class("container-a"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new()
                    .class("chat-icon linear-v")
                    .style("justify-items", "center"),
                Events::new(),
                vec![{
                    let icon = character
                        .texture_id()
                        .map(|r_id| Icon::Resource(*r_id))
                        .unwrap_or(Icon::DefaultUser);
                    common::chat_icon(
                        Attributes::new().class("icon-medium"),
                        &icon,
                        character.name(),
                        resource,
                    )
                }],
            ),
            Html::div(
                Attributes::new().class("chat-args"),
                Events::new(),
                vec![Html::text(character.name())],
            ),
            Html::div(
                Attributes::new()
                    .class("chat-payload")
                    .class("chat-payload-denth")
                    .class("keyvalue")
                    .class("keyvalue-align-start"),
                Events::new(),
                if let Some(block::property::Value::Children(children)) = block_field
                    .get::<block::Property>(character.property_id())
                    .map(|p| p.value())
                {
                    render_item_payload(
                        block_field,
                        selected_property(block_field, children.iter().collect())
                            .iter()
                            .collect(),
                    )
                } else {
                    vec![]
                },
            ),
        ],
    )
}

fn render_item_payload(block_field: &block::Field, props: Vec<&BlockId>) -> Vec<Html<Msg>> {
    block_field
        .listed::<block::Property>(props)
        .map(|(_, prop)| match prop.value() {
            block::property::Value::Children(children) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(
                    Attributes::new()
                        .class("keyvalue")
                        .class("keyvalue-align-start"),
                    Events::new(),
                    render_item_payload(block_field, children.iter().collect()),
                ),
            ],
            block::property::Value::None => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(Attributes::new(), Events::new(), vec![]),
            ],
            block::property::Value::Num(x) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(x.to_string())],
                ),
            ],
            block::property::Value::Str(x) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(Attributes::new(), Events::new(), vec![Html::text(x)]),
            ],
        })
        .flatten()
        .collect()
}

fn selected_property(block_field: &block::Field, props: Vec<&BlockId>) -> Vec<BlockId> {
    let props = block_field.listed::<block::Property>(props);
    let mut selected = vec![];

    for (prop_id, prop) in props {
        if prop.is_selected() {
            selected.push(prop_id);
        } else if let block::property::Value::Children(children) = prop.value() {
            let mut children = selected_property(block_field, children.iter().collect());
            selected.append(&mut children);
        }
    }

    selected
}
