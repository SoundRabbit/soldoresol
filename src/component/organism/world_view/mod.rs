use super::atom::{
    btn::{self, Btn},
    collapse::{self, Collapse},
    dropdown::{self, Dropdown},
    fa,
    heading::{self, Heading},
    text,
};
use super::molecule::tab_menu::{self, TabMenu};
use super::template::common::Common;
use crate::arena::{block, ArenaMut, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
}

pub enum Msg {
    SetIsShowing(bool),
    SetSelectedTabIdx(usize),
}

pub enum On {}

pub struct WorldView {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    is_showing: bool,
    selected_tab_idx: usize,
}

impl Component for WorldView {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for WorldView {
    fn constructor(props: &Props) -> Self {
        WorldView {
            arena: ArenaMut::clone(&props.arena),
            world: BlockMut::clone(&props.world),
            is_showing: false,
            selected_tab_idx: 0,
        }
    }
}

impl Update for WorldView {
    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::SetIsShowing(is_showing) => {
                self.is_showing = is_showing;
                Cmd::none()
            }
            Msg::SetSelectedTabIdx(selected_tab_idx) => {
                self.selected_tab_idx = selected_tab_idx;
                Cmd::none()
            }
        }
    }
}

impl Render for WorldView {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![
                Btn::light(
                    Attributes::new().title(if self.is_showing {
                        "管理メニューをしまう"
                    } else {
                        "管理メニューを表示"
                    }),
                    Events::new().on_click({
                        let is_showing = self.is_showing;
                        move |_| Msg::SetIsShowing(!is_showing)
                    }),
                    vec![fa::i(if self.is_showing {
                        "fa-caret-right"
                    } else {
                        "fa-caret-left"
                    })],
                ),
                Html::div(
                    Attributes::new()
                        .class(Self::class("content"))
                        .string("data-is-showing", self.is_showing.to_string()),
                    Events::new(),
                    vec![
                        TabMenu::empty(
                            tab_menu::Props {
                                controlled: true,
                                selected: self.selected_tab_idx,
                                tabs: vec![String::from("プレハブ")],
                            },
                            Sub::map(|sub| match sub {
                                tab_menu::On::ChangeSelectedTab(tab_idx) => {
                                    Msg::SetSelectedTabIdx(tab_idx)
                                }
                            }),
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("scroll")),
                            Events::new(),
                            match self.selected_tab_idx {
                                _ => vec![],
                            },
                        ),
                    ],
                ),
            ],
        ))
    }
}

impl Styled for WorldView {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "grid-template-rows": "1fr";
                "max-width": "max-content";
                "height": "100%";
            }

            ".base > button" {
                "padding-left": ".35em";
                "padding-right": ".35em";
            }

            ".content" {
                "background-color": crate::libs::color::Pallet::gray(8);
                "color": crate::libs::color::Pallet::gray(0);
                "overflow": "hidden";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
            }

            ".content[data-is-showing='true']" {
                "min-width": "40ch";
                "max-width": "40ch";
            }

            ".content[data-is-showing='false']" {
                "min-width": "0";
                "max-width": "0";
            }

            ".scroll" {
                "overflow-y": "scroll";
                "height": "100%";
                "padding": ".35rem";
            }

            ".item" {
                "width": "100%";
            }

            ".item-content" {
                "margin-left": "1rem";
                "border-left": format!(".35rem solid {}", crate::libs::color::Pallet::gray(0));
                "padding-left": ".65rem";
                "padding-top": ".65rem";
            }
        }
    }
}
