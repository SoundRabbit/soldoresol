use super::super::atom::{
    btn::{self, Btn},
    common::Common,
    dropdown::{self, Dropdown},
    fa,
    slider::{self, Slider},
    text,
};
use super::super::organism::{
    popup_color_pallet::{self, PopupColorPallet},
    room_modeless::RoomModeless,
};
use super::ShowingModal;
use crate::arena::{block, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub boxblock: BlockMut<block::Boxblock>,
}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    OpenModal(ShowingModal),
    SetName(String),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetColor(crate::libs::color::Pallet),
    SetShape(block::boxblock::Shape),
    SetSize([f64; 3]),
}

pub struct Tab0 {
    boxblock: BlockMut<block::Boxblock>,
    element_id: ElementId,
}

ElementId! {
    input_boxblock_name,
    input_boxblock_display_name
}

impl Component for Tab0 {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Tab0 {}

impl Constructor for Tab0 {
    fn constructor(props: Props) -> Self {
        Self {
            boxblock: props.boxblock,
            element_id: ElementId::new(),
        }
    }
}

impl Update for Tab0 {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.boxblock = props.boxblock;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(event) => Cmd::submit(event),
        }
    }
}

impl Render<Html> for Tab0 {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
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
            ],
        ))
    }
}

impl Tab0 {
    fn render_header(&self, boxblock: &block::Boxblock) -> Html {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_boxblock_name),
                    Events::new(),
                    vec![fa::fas_i("fa-cube")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_name)
                        .value(boxblock.name()),
                    Events::new().on_input(self, |name| Msg::Sub(On::SetName(name))),
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
                    Events::new().on_input(self, |dn1| Msg::Sub(On::SetDisplayName1(dn1))),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_display_name)
                        .value(&boxblock.display_name().0),
                    Events::new().on_input(self, |dn0| Msg::Sub(On::SetDisplayName0(dn0))),
                    vec![],
                ),
            ],
        )
    }

    fn render_main(&self, boxblock: &block::Boxblock) -> Html {
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
                            vec![self.render_main_shape(boxblock)],
                        ),
                        self.render_main_x(boxblock),
                        self.render_main_y(boxblock),
                        self.render_main_z(boxblock),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("色"),
                        PopupColorPallet::empty(
                            self,
                            None,
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: boxblock.color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    Msg::Sub(On::SetColor(color))
                                }
                            }),
                        ),
                        text::span("テクスチャ"),
                        self.render_main_textures(boxblock),
                    ],
                ),
            ],
        )
    }

    fn render_main_shape(&self, boxblock: &block::Boxblock) -> Html {
        Dropdown::new(
            self,
            None,
            dropdown::Props {
                direction: dropdown::Direction::Bottom,
                toggle_type: dropdown::ToggleType::Click,
                variant: btn::Variant::DarkLikeMenu,
            },
            Sub::none(),
            (
                vec![match boxblock.shape() {
                    block::boxblock::Shape::Cube => Html::text("立方体"),
                    block::boxblock::Shape::Slope => Html::text("斜面"),
                    block::boxblock::Shape::Sphere => Html::text("球体"),
                    block::boxblock::Shape::Cylinder => Html::text("円柱"),
                }],
                vec![
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(self, |_| {
                            Msg::Sub(On::SetShape(block::boxblock::Shape::Cube))
                        }),
                        vec![Html::text("立方体")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(self, |_| {
                            Msg::Sub(On::SetShape(block::boxblock::Shape::Slope))
                        }),
                        vec![Html::text("斜面")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(self, |_| {
                            Msg::Sub(On::SetShape(block::boxblock::Shape::Sphere))
                        }),
                        vec![Html::text("球体")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(self, |_| {
                            Msg::Sub(On::SetShape(block::boxblock::Shape::Cylinder))
                        }),
                        vec![Html::text("円柱")],
                    ),
                ],
            ),
        )
    }

    fn render_main_x(&self, boxblock: &block::Boxblock) -> Html {
        Html::fragment(vec![
            text::span("X幅"),
            Slider::new(
                self,
                None,
                slider::Position::Linear {
                    min: 0.1,
                    max: 10.0,
                    val: boxblock.size()[0],
                    step: 0.1,
                },
                Sub::map({
                    let size = boxblock.size().clone();
                    move |sub| match sub {
                        slider::On::Input(x) => {
                            let mut size = size.clone();
                            size[0] = x;
                            Msg::Sub(On::SetSize(size))
                        }
                        _ => Msg::NoOp,
                    }
                }),
                slider::Props {
                    range_is_editable: false,
                    theme: slider::Theme::Light,
                },
            ),
        ])
    }

    fn render_main_y(&self, boxblock: &block::Boxblock) -> Html {
        Html::fragment(vec![
            text::span("Y幅"),
            Slider::new(
                self,
                None,
                slider::Position::Linear {
                    min: 0.1,
                    max: 10.0,
                    val: boxblock.size()[1],
                    step: 0.1,
                },
                Sub::map({
                    let size = boxblock.size().clone();
                    move |sub| match sub {
                        slider::On::Input(y) => {
                            let mut size = size.clone();
                            size[1] = y;
                            Msg::Sub(On::SetSize(size))
                        }
                        _ => Msg::NoOp,
                    }
                }),
                slider::Props {
                    range_is_editable: false,
                    theme: slider::Theme::Light,
                },
            ),
        ])
    }

    fn render_main_z(&self, boxblock: &block::Boxblock) -> Html {
        Html::fragment(vec![
            text::span("Z幅"),
            Slider::new(
                self,
                None,
                slider::Position::Linear {
                    min: 0.1,
                    max: 10.0,
                    val: boxblock.size()[2],
                    step: 0.1,
                },
                Sub::map({
                    let size = boxblock.size().clone();
                    move |sub| match sub {
                        slider::On::Input(z) => {
                            let mut size = size.clone();
                            size[2] = z;
                            Msg::Sub(On::SetSize(size))
                        }
                        _ => Msg::NoOp,
                    }
                }),
                slider::Props {
                    range_is_editable: false,
                    theme: slider::Theme::Light,
                },
            ),
        ])
    }

    fn render_main_textures(&self, boxblock: &block::Boxblock) -> Html {
        boxblock
            .texture()
            .as_ref()
            .map(|texture| {
                texture.map(|texture| {
                    Html::img(
                        Attributes::new()
                            .draggable("false")
                            .src(texture.data().url().to_string())
                            .class(Common::bg_transparent()),
                        Events::new().on_click(self, |_| {
                            Msg::Sub(On::OpenModal(ShowingModal::SelectBlockTexture))
                        }),
                        vec![],
                    )
                })
            })
            .unwrap_or(None)
            .unwrap_or_else(|| {
                Btn::secondary(
                    Attributes::new(),
                    Events::new().on_click(self, |_| {
                        Msg::Sub(On::OpenModal(ShowingModal::SelectBlockTexture))
                    }),
                    vec![Html::text("テクスチャを選択")],
                )
            })
    }
}

impl Styled for Tab0 {
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
