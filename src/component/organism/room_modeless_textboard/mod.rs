use super::molecule::tab_menu::{self, TabMenu};
use super::organism::modal_resource::{self, ModalResource};
use crate::arena::{block, resource, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

mod tab_0;

use tab_0::Tab0;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub data: block::textboard::Block,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetShowingModal(ShowingModal),
    SetTitle(String),
    SetText(String),
    SetFontSize(f64),
    SetXSize(f64),
    SetZSize(f64),
    SetColor(crate::libs::color::Pallet),
}

pub enum ShowingModal {
    None,
    SelectTexture,
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessTextboard {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    textboard: block::textboard::Block,
    showing_modal: ShowingModal,
}

impl Component for RoomModelessTextboard {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomModelessTextboard {}

impl Constructor for RoomModelessTextboard {
    fn constructor(props: Props) -> Self {
        Self {
            arena: props.arena,
            world: props.world,
            textboard: props.data,
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for RoomModelessTextboard {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.world = props.world;
        self.textboard = props.data;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
            Msg::SetTitle(title) => {
                self.textboard.update(|textboard| {
                    textboard.set_title(title.clone());
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.textboard.id() },
                })
            }
            Msg::SetText(text) => {
                self.textboard.update(|textboard| {
                    textboard.set_text(text.clone());
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.textboard.id() },
                })
            }
            Msg::SetFontSize(font_size) => {
                self.textboard.update(|textboard| {
                    textboard.set_font_size(font_size);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.textboard.id() },
                })
            }
            Msg::SetXSize(x_size) => {
                self.textboard.update(|textboard| {
                    let z_size = textboard.size()[1];
                    textboard.set_size([x_size, z_size]);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.textboard.id() },
                })
            }
            Msg::SetZSize(z_size) => {
                self.textboard.update(|textboard| {
                    let x_size = textboard.size()[0];
                    textboard.set_size([x_size, z_size]);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.textboard.id() },
                })
            }
            Msg::SetColor(color) => {
                self.textboard.update(|textboard| {
                    textboard.set_color(color);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.textboard.id() },
                })
            }
        }
    }
}

impl Render<Html> for RoomModelessTextboard {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::fragment(vec![
            self.render_tabs(),
            match &self.showing_modal {
                ShowingModal::None => Html::none(),
                ShowingModal::SelectTexture => Html::none(),
            },
        ]))
    }
}

impl RoomModelessTextboard {
    fn render_tabs(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![TabMenu::new(
                self,
                None,
                tab_menu::Props {
                    selected: 0,
                    controlled: false,
                },
                Sub::none(),
                (
                    Attributes::new(),
                    Events::new(),
                    vec![(
                        Html::text("Common"),
                        Tab0::empty(
                            self,
                            None,
                            tab_0::Props {
                                textboard: block::textboard::Block::clone(&self.textboard),
                            },
                            Sub::map(|sub| match sub {
                                tab_0::On::OpenModal(modal_kind) => {
                                    Msg::SetShowingModal(modal_kind)
                                }
                                tab_0::On::SetColor(pallet) => Msg::SetColor(pallet),
                                tab_0::On::SetFontSize(font_size) => Msg::SetFontSize(font_size),
                                tab_0::On::SetText(text) => Msg::SetText(text),
                                tab_0::On::SetTitle(name) => Msg::SetTitle(name),
                                tab_0::On::SetXSize(x_size) => Msg::SetXSize(x_size),
                                tab_0::On::SetZSize(y_size) => Msg::SetZSize(y_size),
                            }),
                        ),
                    )],
                ),
            )],
        )
    }
}

impl Styled for RoomModelessTextboard {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "padding-top": ".65rem";
            }
        }
    }
}
