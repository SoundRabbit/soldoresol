use super::atom::fa;
use super::atom::slider::{self, Slider};
use super::atom::text;
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
    pub data: BlockMut<block::Craftboard>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetXSize(f64),
    SetYSize(f64),
    SetGridColor(crate::libs::color::Pallet),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessCraftboard {
    craftboard: BlockMut<block::Craftboard>,
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
                    craftboard.set_size([x_size, s[1]])
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.craftboard.id() },
                })
            }
            Msg::SetYSize(y_size) => {
                self.craftboard.update(|craftboard| {
                    let s = craftboard.size();
                    craftboard.set_size([s[0], y_size])
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
        }
    }
}

impl Render for RoomModelessCraftboard {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.craftboard
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.craftboard
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
            ],
        ))
    }
}

impl RoomModelessCraftboard {
    fn render_header(&self, craftboard: &block::Craftboard) -> Html<Self> {
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

    fn render_main(&self, craftboard: &block::Craftboard) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("main")),
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

impl Styled for RoomModelessCraftboard {
    fn style() -> Style {
        style! {
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
