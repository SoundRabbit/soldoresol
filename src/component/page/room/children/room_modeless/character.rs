use super::super::atom::btn::{self, Btn};
use super::super::atom::dropdown::{self, Dropdown};
use super::super::util::styled::{Style, Styled};
use crate::arena::block;
use async_std::sync::{Arc, Mutex};
use kagura::prelude::*;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub character_id: block::BlockId,
}

pub enum Msg {}

pub enum On {}

pub struct Character {
    block_arena: block::ArenaRef,
    character_id: block::BlockId,
    element_id: ElementId,
}

struct ElementId {
    input_character_name: String,
}

impl Constructor for Character {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            block_arena: props.block_arena,
            character_id: props.character_id,
            element_id: ElementId {
                input_character_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
        }
    }
}

impl Component for Character {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.character_id = props.character_id;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(
                    &self.character_id,
                    |character: &block::character::Character| self.render_character(character),
                )
                .unwrap_or(Html::none()),
        )
    }
}

impl Character {
    fn render_select_option(value: impl Into<String>, text: impl Into<String>) -> Html {
        Html::option(
            Attributes::new().value(value),
            Events::new(),
            vec![Html::text(text)],
        )
    }

    fn render_character(&self, character: &block::character::Character) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("common")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("common-props")),
                        Events::new(),
                        vec![
                            Html::input(
                                Attributes::new().value(character.name()),
                                Events::new(),
                                vec![],
                            ),
                            Html::textarea(
                                Attributes::new()
                                    .value(character.name())
                                    .class(Self::class("common-description")),
                                Events::new(),
                                vec![],
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("common-imgs")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Self::class("common-imgs-type")),
                                Events::new(),
                                vec![
                                    Dropdown::with_children(
                                        dropdown::Props {
                                            direction: dropdown::Direction::Bottom,
                                            text: String::from(character.current_tex_name()),
                                            toggle_type: dropdown::ToggleType::Click,
                                            variant: btn::Variant::Menu,
                                        },
                                        Subscription::none(),
                                        character
                                            .tex_names()
                                            .into_iter()
                                            .map(|tex_name| {
                                                Btn::with_child(
                                                    btn::Props {
                                                        variant: btn::Variant::Menu,
                                                    },
                                                    Subscription::none(),
                                                    Html::text(tex_name),
                                                )
                                            })
                                            .collect(),
                                    ),
                                    Btn::with_child(
                                        btn::Props {
                                            variant: btn::Variant::Primary,
                                        },
                                        Subscription::none(),
                                        Html::text("追加"),
                                    ),
                                    Btn::with_child(
                                        btn::Props {
                                            variant: btn::Variant::Danger,
                                        },
                                        Subscription::none(),
                                        Html::text("削除"),
                                    ),
                                ],
                            ),
                            Html::input(
                                Attributes::new().value(character.current_tex_name()),
                                Events::new(),
                                vec![],
                            ),
                        ],
                    ),
                ],
            )],
        )
    }
}

impl Styled for Character {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "row";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "base textarea" {
                "resize": "none";
            }

            "common" {
                "display": "grid";
                "grid-template-columns": "1fr 15rem";
                "grid-template-rows": "20rem";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "common-props" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "common-imgs-type" {
                "display": "grid";
                "grid-template-columns": "1fr max-content max-content";
            }
        }
    }
}
