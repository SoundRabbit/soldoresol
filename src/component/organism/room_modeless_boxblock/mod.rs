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
    pub data: BlockMut<block::Boxblock>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetShowingModal(ShowingModal),
    SetColor(crate::libs::color::Pallet),
    SetName(String),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetShape(block::boxblock::Shape),
    SetSize([f64; 3]),
    SetTexture(Option<BlockRef<resource::BlockTexture>>),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessBoxblock {
    boxblock: BlockMut<block::Boxblock>,
    showing_modal: ShowingModal,
}

pub enum ShowingModal {
    None,
    SelectBlockTexture,
}

impl Component for RoomModelessBoxblock {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomModelessBoxblock {}

impl Constructor for RoomModelessBoxblock {
    fn constructor(props: Self::Props) -> Self {
        Self {
            boxblock: BlockMut::clone(&props.data),
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for RoomModelessBoxblock {
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.boxblock = BlockMut::clone(&props.data);
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
            Msg::SetColor(color) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_color(color);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetName(name) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_name(name);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetDisplayName0(display_name) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_display_name((Some(display_name), None));
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetDisplayName1(display_name) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_display_name((None, Some(display_name)));
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetShape(shape) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_shape(shape);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetSize(size) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_size(size);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetTexture(texture) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_texture(texture);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
        }
    }
}

impl Render<Html> for RoomModelessBoxblock {
    type Children = ();
    fn render(&self, props: &Props, _: Self::Children) -> Html {
        Self::styled(Html::fragment(vec![
            self.render_tabs(),
            match &self.showing_modal {
                ShowingModal::None => Html::none(),
                ShowingModal::SelectBlockTexture => ModalResource::empty(
                    self,
                    None,
                    modal_resource::Props {
                        arena: ArenaMut::clone(&props.arena),
                        world: BlockMut::clone(&props.world),
                        title: String::from(modal_resource::title::SELECT_BLOCK_TEXTURE),
                        filter: set! { BlockKind::BlockTexture },
                        is_selecter: true,
                    },
                    Sub::map(|sub| match sub {
                        modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                        modal_resource::On::UpdateBlocks { insert, update } => {
                            Msg::Sub(On::UpdateBlocks { insert, update })
                        }
                        modal_resource::On::SelectBlockTexture(texture) => {
                            Msg::SetTexture(Some(texture))
                        }
                        modal_resource::On::SelectNone => Msg::SetTexture(None),
                        _ => Msg::NoOp,
                    }),
                ),
            },
        ]))
    }
}

impl RoomModelessBoxblock {
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
                                boxblock: BlockMut::clone(&self.boxblock),
                            },
                            Sub::map(|sub| match sub {
                                tab_0::On::OpenModal(modal) => Msg::SetShowingModal(modal),
                                tab_0::On::SetColor(pallet) => Msg::SetColor(pallet),
                                tab_0::On::SetDisplayName0(dn0) => Msg::SetDisplayName0(dn0),
                                tab_0::On::SetDisplayName1(dn1) => Msg::SetDisplayName1(dn1),
                                tab_0::On::SetName(name) => Msg::SetName(name),
                                tab_0::On::SetShape(shape) => Msg::SetShape(shape),
                                tab_0::On::SetSize(size) => Msg::SetSize(size),
                            }),
                        ),
                    )],
                ),
            )],
        )
    }
}

impl Styled for RoomModelessBoxblock {
    fn style() -> Style {
        style! {}
    }
}
