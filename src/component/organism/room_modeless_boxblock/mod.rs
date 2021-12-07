use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::slider::{self, Slider};
use super::atom::text;
use super::organism::modal_resource::{self, ModalResource};
use super::organism::popup_color_pallet::{self, PopupColorPallet};
use super::organism::room_modeless::RoomModeless;
use super::template::common::Common;
use crate::arena::{block, resource, ArenaMut, BlockKind, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;

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
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetShape(block::boxblock::Shape),
    SetSize([f64; 3]),
    SetTexture(Option<BlockMut<resource::BlockTexture>>),
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
    element_id: ElementId,
}

pub enum ShowingModal {
    None,
    SelectBlockTexture,
}

ElementId! {
    input_boxblock_name,
    input_boxblock_display_name
}

impl Component for RoomModelessBoxblock {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModelessBoxblock {
    fn constructor(props: &Props) -> Self {
        Self {
            boxblock: BlockMut::clone(&props.data),
            showing_modal: ShowingModal::None,
            element_id: ElementId::new(),
        }
    }
}

impl Update for RoomModelessBoxblock {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.boxblock = BlockMut::clone(&props.data);
        Cmd::none()
    }

    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
            Msg::SetColor(color) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_color(color);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetDisplayName0(display_name) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_display_name((Some(display_name), None));
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetDisplayName1(display_name) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_display_name((None, Some(display_name)));
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetShape(shape) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_shape(shape);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetSize(size) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_size(size);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
            Msg::SetTexture(texture) => {
                self.boxblock.update(|boxblock| {
                    boxblock.set_texture(texture);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.boxblock.id() },
                })
            }
        }
    }
}

impl Render for RoomModelessBoxblock {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.boxblock
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.boxblock
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
                match &self.showing_modal {
                    ShowingModal::None => Html::none(),
                    ShowingModal::SelectBlockTexture => ModalResource::empty(
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
            ],
        ))
    }
}

impl RoomModelessBoxblock {
    fn render_header(&self, boxblock: &block::Boxblock) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_boxblock_name),
                    Events::new(),
                    vec![fa::i("fa-cube")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_name)
                        .value(boxblock.name()),
                    Events::new(),
                    vec![],
                ),
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_boxblock_display_name),
                    Events::new(),
                    vec![Html::text("表示名")],
                ),
                Html::input(
                    Attributes::new().value(&boxblock.display_name().1),
                    Events::new().on_input(Msg::SetDisplayName1),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_display_name)
                        .value(&boxblock.display_name().0),
                    Events::new().on_input(Msg::SetDisplayName0),
                    vec![],
                ),
            ],
        )
    }

    fn render_main(&self, boxblock: &block::Boxblock) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new()
                                .class(Common::banner())
                                .class(Self::class("dropdown")),
                            Events::new(),
                            vec![Dropdown::with_children(
                                dropdown::Props {
                                    text: dropdown::Text::Text(String::from(
                                        match boxblock.shape() {
                                            block::boxblock::Shape::Cube => "立方体",
                                            block::boxblock::Shape::Slope => "斜面",
                                            block::boxblock::Shape::Sphere => "球体",
                                            block::boxblock::Shape::Cylinder => "円柱",
                                        },
                                    )),
                                    direction: dropdown::Direction::Bottom,
                                    toggle_type: dropdown::ToggleType::Click,
                                    variant: btn::Variant::DarkLikeMenu,
                                },
                                Sub::none(),
                                vec![
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(|_| {
                                            Msg::SetShape(block::boxblock::Shape::Cube)
                                        }),
                                        vec![Html::text("立方体")],
                                    ),
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(|_| {
                                            Msg::SetShape(block::boxblock::Shape::Slope)
                                        }),
                                        vec![Html::text("斜面")],
                                    ),
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(|_| {
                                            Msg::SetShape(block::boxblock::Shape::Sphere)
                                        }),
                                        vec![Html::text("球体")],
                                    ),
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(|_| {
                                            Msg::SetShape(block::boxblock::Shape::Cylinder)
                                        }),
                                        vec![Html::text("円柱")],
                                    ),
                                ],
                            )],
                        ),
                        text::span("X幅"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: boxblock.size()[0],
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map({
                                let size = boxblock.size().clone();
                                move |sub| match sub {
                                    slider::On::Input(x) => {
                                        let mut size = size.clone();
                                        size[0] = x;
                                        Msg::SetSize(size)
                                    }
                                    _ => Msg::NoOp,
                                }
                            }),
                        ),
                        text::span("Y幅"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: boxblock.size()[1],
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map({
                                let size = boxblock.size().clone();
                                move |sub| match sub {
                                    slider::On::Input(y) => {
                                        let mut size = size.clone();
                                        size[1] = y;
                                        Msg::SetSize(size)
                                    }
                                    _ => Msg::NoOp,
                                }
                            }),
                        ),
                        text::span("Z幅"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: boxblock.size()[2],
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map({
                                let size = boxblock.size().clone();
                                move |sub| match sub {
                                    slider::On::Input(z) => {
                                        let mut size = size.clone();
                                        size[2] = z;
                                        Msg::SetSize(size)
                                    }
                                    _ => Msg::NoOp,
                                }
                            }),
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("色"),
                        PopupColorPallet::empty(
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: boxblock.color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => Msg::SetColor(color),
                            }),
                        ),
                        text::span("テクスチャ"),
                        boxblock
                            .texture()
                            .as_ref()
                            .map(|texture| {
                                texture.map(|texture| {
                                    Html::img(
                                        Attributes::new()
                                            .src(texture.data().url().to_string())
                                            .class(Common::bg_transparent()),
                                        Events::new().on_click(|_| {
                                            Msg::SetShowingModal(ShowingModal::SelectBlockTexture)
                                        }),
                                        vec![],
                                    )
                                })
                            })
                            .unwrap_or(None)
                            .unwrap_or_else(|| {
                                Btn::secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| {
                                        Msg::SetShowingModal(ShowingModal::SelectBlockTexture)
                                    }),
                                    vec![Html::text("テクスチャを選択")],
                                )
                            }),
                    ],
                ),
            ],
        )
    }
}

impl Styled for RoomModelessBoxblock {
    fn style() -> Style {
        style! {
            ".dropdown" {
                "overflow": "visible !important";
            }

            ".main" {
                "display": "grid";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "align-items": "start";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "column-gap": ".65rem";
                "overflow-y": "scroll";
            }
        }
    }
}
