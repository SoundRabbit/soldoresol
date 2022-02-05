use super::super::atom::fa;
use super::super::atom::slider::{self, Slider};
use super::super::atom::text;
use super::super::organism::popup_color_pallet::{self, PopupColorPallet};
use super::super::organism::room_modeless::RoomModeless;
use super::*;

impl RoomModelessCraftboard {
    pub fn render_tab0(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.craftboard
                    .map(|data| self.render_tab0_header(data))
                    .unwrap_or(Common::none()),
                self.craftboard
                    .map(|data| self.render_tab0_main(data))
                    .unwrap_or(Common::none()),
            ],
        )
    }

    fn render_tab0_header(&self, craftboard: &block::Craftboard) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_craftboard_name),
                    Events::new(),
                    vec![fa::i("fa-user")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_craftboard_name)
                        .value(craftboard.name()),
                    Events::new(),
                    vec![],
                ),
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_craftboard_display_name),
                    Events::new(),
                    vec![Html::text("表示名")],
                ),
                Html::input(
                    Attributes::new().value(&craftboard.display_name().1),
                    Events::new().on_input(Msg::SetDisplayName1),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_craftboard_display_name)
                        .value(&craftboard.display_name().0),
                    Events::new().on_input(Msg::SetDisplayName0),
                    vec![],
                ),
            ],
        )
    }

    fn render_tab0_main(&self, craftboard: &block::Craftboard) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("tab0-main")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("X幅（横幅）"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 1.0,
                                    max: 100.0,
                                    val: craftboard.size()[0],
                                    step: 1.0,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::SetXSize(x),
                                _ => Msg::NoOp,
                            }),
                        ),
                        text::span("Y幅（奥行き）"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 1.0,
                                    max: 100.0,
                                    val: craftboard.size()[1],
                                    step: 1.0,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(y) => Msg::SetYSize(y),
                                _ => Msg::NoOp,
                            }),
                        ),
                        text::span("Z幅（高さ）"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 1.0,
                                    max: 100.0,
                                    val: craftboard.size()[2],
                                    step: 1.0,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(z) => Msg::SetZSize(z),
                                _ => Msg::NoOp,
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
                                default_selected: craftboard.grid_color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    Msg::SetGridColor(color)
                                }
                            }),
                        ),
                    ],
                ),
            ],
        )
    }
}
