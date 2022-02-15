use super::molecule::tab_menu::{self, TabMenu};
use super::organism::modal_resource::{self, ModalResource};
use super::template::common::Common;
use crate::arena::{block, resource, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;

mod tab_0;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub data: BlockMut<block::Craftboard>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetShowingModal(ShowingModal),
    SetSelectedTabIdx(usize),
    SetName(String),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetXSize(f64),
    SetYSize(f64),
    SetZSize(f64),
    SetGridColor(crate::libs::color::Pallet),
    SetTexture(usize, Option<BlockRef<resource::ImageData>>),
}

pub enum ShowingModal {
    None,
    SelectTexture(usize),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessCraftboard {
    craftboard: BlockMut<block::Craftboard>,
    selected_tab_idx: usize,
    showing_modal: ShowingModal,
    element_id: ElementId,
}

ElementId! {
    input_craftboard_name,
    input_craftboard_display_name
}

impl Component for RoomModelessCraftboard {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModelessCraftboard {
    fn constructor(props: &Props) -> Self {
        Self {
            craftboard: BlockMut::clone(&props.data),
            selected_tab_idx: 0,
            showing_modal: ShowingModal::None,
            element_id: ElementId::new(),
        }
    }
}

impl Update for RoomModelessCraftboard {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.craftboard = BlockMut::clone(&props.data);
        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
            Msg::SetSelectedTabIdx(tab_idx) => {
                self.selected_tab_idx = tab_idx;
                Cmd::none()
            }
            Msg::SetName(name) => {
                self.craftboard.update(|craftboard| {
                    craftboard.set_name(name);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetDisplayName0(display_name) => {
                self.craftboard.update(|craftboard| {
                    craftboard.set_display_name((Some(display_name), None));
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetDisplayName1(display_name) => {
                self.craftboard.update(|craftboard| {
                    craftboard.set_display_name((None, Some(display_name)));
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetXSize(x_size) => {
                self.craftboard.update(|craftboard| {
                    let s = craftboard.size();
                    craftboard.set_size([x_size, s[1], s[2]])
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetYSize(y_size) => {
                self.craftboard.update(|craftboard| {
                    let s = craftboard.size();
                    craftboard.set_size([s[0], y_size, s[2]])
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetZSize(z_size) => {
                self.craftboard.update(|craftboard| {
                    let s = craftboard.size();
                    craftboard.set_size([s[0], s[1], z_size])
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetGridColor(grid_color) => {
                self.craftboard.update(|craftboard| {
                    craftboard.set_grid_color(grid_color);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetTexture(tex_idx, texture) => {
                self.craftboard.update(|craftboard| {
                    let mut textures = craftboard.textures().clone();
                    textures[tex_idx] = texture;
                    craftboard.set_textures(textures);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
        }
    }
}

impl Render for RoomModelessCraftboard {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::fragment(vec![
            self.render_tabs(),
            match &self.showing_modal {
                ShowingModal::None => Html::none(),
                ShowingModal::SelectTexture(tex_idx) => ModalResource::empty(
                    modal_resource::Props {
                        arena: ArenaMut::clone(&props.arena),
                        world: BlockMut::clone(&props.world),
                        title: String::from(modal_resource::title::SELECT_TEXTURE),
                        filter: set! { BlockKind::ImageData },
                        is_selecter: true,
                    },
                    Sub::map({
                        let tex_idx = *tex_idx;
                        move |sub| match sub {
                            modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                            modal_resource::On::UpdateBlocks { insert, update } => {
                                Msg::Sub(On::UpdateBlocks { insert, update })
                            }
                            modal_resource::On::SelectImageData(texture) => {
                                Msg::SetTexture(tex_idx, Some(texture))
                            }
                            modal_resource::On::SelectNone => Msg::SetTexture(tex_idx, None),
                            _ => Msg::NoOp,
                        }
                    }),
                ),
            },
        ]))
    }
}

impl RoomModelessCraftboard {
    fn render_tabs(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![TabMenu::with_children(
                tab_menu::Props {
                    selected: self.selected_tab_idx,
                    tabs: vec![String::from("Common")],
                    controlled: true,
                },
                Sub::map(|sub| match sub {
                    tab_menu::On::ChangeSelectedTab(tab_idx) => Msg::SetSelectedTabIdx(tab_idx),
                }),
                vec![if self.selected_tab_idx == 0 {
                    self.render_tab0()
                } else {
                    Common::none()
                }],
            )],
        )
    }
}

impl Styled for RoomModelessCraftboard {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "padding-top": ".65rem";
            }

            ".tab0-main" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "overflow-y": "scroll";
            }

            ".tab0-content" {
                "display": "grid";
                "column-gap": ".65rem";
                "row-gap": ".65rem";
                "align-items": "start";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "grid-auto-rows": "max-content";
            }

            ".tab0-content img" {
                "width": "100%";
                "max-height": "20rem";
                "object-fit": "contain";
            }
        }
    }
}
