use super::atom::{
    btn::Btn,
    heading::{self, Heading},
    text,
};
use super::molecule::modal::{self, Modal};
use super::template::common::Common;
use crate::arena::{block, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Props {
    pub world: BlockRef<block::World>,
    pub selected: Vec<BlockMut<block::Character>>,
}

pub enum Msg {
    PushSelected(BlockMut<block::Character>),
    RemoveSelected(usize),
    Cancel,
    Select,
}

pub enum On {
    Cancel,
    Select(Vec<BlockMut<block::Character>>),
}

pub struct ModalChatUser {
    selected_index: Vec<BlockMut<block::Character>>,
    selected: HashSet<U128Id>,
}

impl Component for ModalChatUser {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ModalChatUser {
    fn constructor(props: &Props) -> Self {
        Self {
            selected_index: props.selected.iter().map(BlockMut::clone).collect(),
            selected: props.selected.iter().map(|b| b.id()).collect(),
        }
    }
}

impl Update for ModalChatUser {
    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Cancel => Cmd::sub(On::Cancel),
            Msg::Select => Cmd::sub(On::Select(
                self.selected_index.iter().map(BlockMut::clone).collect(),
            )),
            Msg::PushSelected(item) => {
                self.selected.insert(item.id());
                self.selected_index.push(item);
                Cmd::none()
            }
            Msg::RemoveSelected(idx) => {
                if idx < self.selected_index.len() {
                    let character = self.selected_index.remove(idx);
                    self.selected.remove(&character.id());
                }
                Cmd::none()
            }
        }
    }
}

impl Render for ModalChatUser {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Modal::with_children(
            modal::Props {
                header_title: String::from("チャットで使用するキャラクター"),
                footer_message: String::from(""),
            },
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Cancel,
            }),
            vec![Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new()
                            .class(Self::class("content"))
                            .class(Self::class("container")),
                        Events::new(),
                        vec![self.render_unselected(props), self.render_selected()],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("container")),
                        Events::new(),
                        vec![Btn::primary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::Select),
                            vec![Html::text("決定")],
                        )],
                    ),
                ],
            )],
        ))
    }
}

impl ModalChatUser {
    fn render_unselected(&self, props: &Props) -> Html<Self> {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Heading::h4(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("使用可能なキャラクター")],
                ),
                Html::div(
                    Attributes::new().class(Self::class("list")),
                    Events::new(),
                    props
                        .world
                        .map(|world| {
                            world
                                .characters()
                                .iter()
                                .filter_map(|character| {
                                    let character_block = BlockMut::clone(&character);
                                    character.map(|character| {
                                        if self.selected.contains(&character_block.id()) {
                                            Btn::menu_as_primary(
                                                Attributes::new(),
                                                Events::new(),
                                                vec![Html::text(character.name())],
                                            )
                                        } else {
                                            Btn::menu(
                                                Attributes::new(),
                                                Events::new().on_click({
                                                    move |_| Msg::PushSelected(character_block)
                                                }),
                                                vec![Html::text(character.name())],
                                            )
                                        }
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or(vec![]),
                ),
            ],
        )
    }

    fn render_selected(&self) -> Html<Self> {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Heading::h4(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("選択済みキャラクター")],
                ),
                Html::div(
                    Attributes::new().class(Self::class("list")),
                    Events::new(),
                    self.selected_index
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, character)| {
                            character.map(|character| {
                                Html::div(
                                    Attributes::new().class(Common::valuekey()),
                                    Events::new(),
                                    vec![
                                        text::div(character.name()),
                                        Btn::danger(
                                            Attributes::new(),
                                            Events::new()
                                                .on_click(move |_| Msg::RemoveSelected(idx)),
                                            vec![Html::text("削除")],
                                        ),
                                    ],
                                )
                            })
                        })
                        .collect(),
                ),
            ],
        )
    }
}

impl Styled for ModalChatUser {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "height": "100%";
                "display": "grid";
                "grid-template-rows": "1fr max-content";
            }
            ".content" {
                "display": "grid";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
            }
            ".container" {
                "padding": ".5em 1em";
            }
            ".list" {
                "overflow-y": "scroll";
            }
        }
    }
}
