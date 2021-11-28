use super::*;
use super::{
    super::atom::btn::{self, Btn},
    super::atom::dropdown::{self, Dropdown},
    super::atom::fa,
    super::atom::slider::{self, Slider},
    super::atom::text,
    super::organism::popup_color_pallet::{self, PopupColorPallet},
    super::template::common::Common,
};

impl RoomModeless {
    pub fn render_boxblock(&self, boxblock: &block::Boxblock) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_boxblock_header(boxblock),
                self.render_boxblock_main(boxblock),
            ],
        )
    }

    fn render_boxblock_header(&self, boxblock: &block::Boxblock) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(Self::class("common-label"))
                        .string("for", &self.element_id.input_boxblock_name),
                    Events::new(),
                    vec![fa::i("fa-cube")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_boxblock_name)
                        .value(&self.inputing_boxblock_name),
                    Events::new(),
                    vec![],
                ),
                Btn::primary(Attributes::new(), Events::new(), vec![Html::text("更新")]),
            ],
        )
    }

    fn render_boxblock_main(&self, boxblock: &block::Boxblock) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("boxblock-main")),
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
                                        Events::new(),
                                        vec![Html::text("立方体")],
                                    ),
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("球体")],
                                    ),
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new(),
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
                            Sub::none(),
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
                            Sub::none(),
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
                            Sub::none(),
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
                            Sub::none(),
                        ),
                        text::span("テクスチャ"),
                        boxblock
                            .texture()
                            .as_ref()
                            .map(|texture| {
                                texture.map(|texture| {
                                    Html::img(
                                        Attributes::new().src(texture.data().url().to_string()),
                                        Events::new(),
                                        vec![],
                                    )
                                })
                            })
                            .unwrap_or(None)
                            .unwrap_or_else(|| {
                                Btn::secondary(
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text("テクスチャを選択")],
                                )
                            }),
                    ],
                ),
            ],
        )
    }

    pub fn style_boxblock() -> Style {
        style! {
            ".dropdown" {
                "overflow": "visible !important";
            }

            ".boxblock-main" {
                "display": "grid";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "align-items": "start";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "column-gap": ".65rem";
            }
        }
    }
}
