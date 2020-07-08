use super::super::super::super::{btn, color_picker, modal};
use super::Msg;
use crate::{
    block::{self, chat::item::Icon, BlockId},
    resource::{Data, ResourceId},
    Color, Resource,
};
use kagura::prelude::*;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

mod common {
    pub use super::super::super::common::*;
}

pub fn header(name: impl Into<String>) -> Html<Msg> {
    modal::header(
        Attributes::new()
            .style("display", "grid")
            .style("grid-template-columns", "1fr max-content"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("text-label"),
                Events::new(),
                vec![Html::text(name)],
            ),
            Html::div(
                Attributes::new().class("linear-h"),
                Events::new(),
                vec![btn::close(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::CloseModal),
                )],
            ),
        ],
    )
}

pub fn select_image(
    resource: &Resource,
    on_select: impl FnOnce(ResourceId) -> Msg + 'static,
) -> Html<Msg> {
    let on_select = Rc::new(RefCell::new(Some(Box::new(on_select))));

    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("画像を選択"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v grid container")
                        .style("min-height", "50vh"),
                    Events::new(),
                    resource
                        .all()
                        .filter_map(|(data_id, data)| {
                            if let Data::Image { url, .. } = data {
                                Some((data_id, url))
                            } else {
                                None
                            }
                        })
                        .map(|(data_id, img_url)| {
                            Html::div(
                                Attributes::new().class("grid-w-2 clickable"),
                                Events::new().on_click({
                                    let data_id = *data_id;
                                    let on_select = Rc::clone(&on_select);
                                    move |_| {
                                        if let Some(on_select) = on_select.borrow_mut().take() {
                                            on_select(data_id)
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                }),
                                vec![Html::img(
                                    Attributes::new()
                                        .class("pure-img")
                                        .string("src", img_url.as_str()),
                                    Events::new(),
                                    vec![],
                                )],
                            )
                        })
                        .collect(),
                ),
                modal::footer(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
                ),
            ],
        )],
    )
}

pub fn color_picker(on_select: impl FnOnce(Color) -> Msg + 'static) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("色の選択"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v")
                        .class("centering")
                        .class("centering-a"),
                    Events::new(),
                    vec![color_picker::all(Msg::NoOp, on_select)],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

pub fn character_selecter(
    block_field: &block::Field,
    resource: &Resource,
    world: &block::World,
    selected: &HashSet<BlockId>,
    on_select: impl FnMut(BlockId, bool) -> Msg + 'static,
) -> Html<Msg> {
    let on_select = Rc::new(RefCell::new(Box::new(on_select)));

    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            6,
            Attributes::new(),
            Events::new(),
            vec![
                header("キャラクターの選択"),
                modal::body(
                    Attributes::new().class("scroll-v"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new()
                            .class("container-a")
                            .class("keyvalueoption"),
                        Events::new(),
                        block_field
                            .listed::<block::Character>(world.characters().collect())
                            .map(|(character_id, character)| {
                                let is_selected = selected.contains(&character_id);
                                vec![
                                    {
                                        let icon = character
                                            .texture_id()
                                            .map(|r_id| Icon::Resource(*r_id))
                                            .unwrap_or(Icon::DefaultUser);
                                        common::chat_icon(
                                            Attributes::new().class("icon-medium").string(
                                                "data-character-id",
                                                character_id.to_string(),
                                            ),
                                            &icon,
                                            character.name(),
                                            resource,
                                        )
                                    },
                                    Html::div(
                                        Attributes::new()
                                            .string("data-character-id", character_id.to_string()),
                                        Events::new(),
                                        vec![Html::text(character.name())],
                                    ),
                                    btn::check(
                                        selected.contains(&character_id),
                                        Attributes::new(),
                                        Events::new().on_click({
                                            let on_select = Rc::clone(&on_select);
                                            move |_| {
                                                (&mut *on_select.borrow_mut())(
                                                    character_id,
                                                    !is_selected,
                                                )
                                            }
                                        }),
                                    ),
                                ]
                            })
                            .flatten()
                            .collect(),
                    )],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
