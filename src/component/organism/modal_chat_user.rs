use super::atom::{
    btn::Btn,
    common::Common,
    heading::{self, Heading},
    text,
};
use super::molecule::{
    modal::{self, Modal},
    sortable::{self, Sortable},
};
use crate::arena::{block, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

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
    selected_ids: HashSet<U128Id>,
    world: BlockRef<block::World>,
}

impl Component for ModalChatUser {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalChatUser {}

impl Constructor for ModalChatUser {
    fn constructor(props: Props) -> Self {
        let selected_ids = props
            .selected
            .iter()
            .map(|b| b.id())
            .collect::<HashSet<_>>();
        Self {
            selected_index: props.selected,
            selected_ids,
            world: props.world,
        }
    }
}

impl Update for ModalChatUser {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.world = props.world;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Cancel => Cmd::submit(On::Cancel),
            Msg::Select => Cmd::submit(On::Select(
                self.selected_index.iter().map(BlockMut::clone).collect(),
            )),
            Msg::PushSelected(item) => {
                self.selected_ids.insert(item.id());
                self.selected_index.push(item);
                Cmd::none()
            }
            Msg::RemoveSelected(idx) => {
                if idx < self.selected_index.len() {
                    let character = self.selected_index.remove(idx);
                    self.selected_ids.remove(&character.id());
                }
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for ModalChatUser {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Modal::new(
            self,
            None,
            modal::Props {},
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Cancel,
            }),
            (
                String::from("チャットで使用するキャラクター"),
                String::from(""),
                vec![Html::div(
                    Attributes::new().class(Self::class("base")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new()
                                .class(Self::class("content"))
                                .class(Self::class("container")),
                            Events::new(),
                            vec![self.render_unselected(), self.render_selected()],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("container")),
                            Events::new(),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(self, |_| Msg::Select),
                                vec![Html::text("決定")],
                            )],
                        ),
                    ],
                )],
            ),
        ))
    }
}

impl ModalChatUser {
    fn render_unselected(&self) -> Html {
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
                    self.world
                        .map(|world| {
                            world
                                .characters()
                                .iter()
                                .filter_map(|character| {
                                    let character_block = BlockMut::clone(&character);
                                    character.map(|character| {
                                        if self.selected_ids.contains(&character_block.id()) {
                                            Btn::menu_as_primary(
                                                Attributes::new(),
                                                Events::new(),
                                                vec![Html::text(character.name())],
                                            )
                                        } else {
                                            Btn::menu(
                                                Attributes::new(),
                                                Events::new().on_click(self, {
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

    fn render_selected(&self) -> Html {
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
                Sortable::new(
                    self,
                    None,
                    sortable::Props {},
                    Sub::none(),
                    (
                        Attributes::new().class(Self::class("list")),
                        Events::new(),
                        self.selected_index
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, character)| {
                                character.map(|character| {
                                    (
                                        idx,
                                        Attributes::new().class(Common::valuekey()),
                                        Events::new(),
                                        vec![
                                            text::div(character.name()),
                                            Btn::danger(
                                                Attributes::new(),
                                                Events::new().on_click(self, move |_| {
                                                    Msg::RemoveSelected(idx)
                                                }),
                                                vec![Html::text("削除")],
                                            ),
                                        ],
                                    )
                                })
                            })
                            .collect(),
                    ),
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
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "row-gap": "0.25rem";
            }
        }
    }
}
