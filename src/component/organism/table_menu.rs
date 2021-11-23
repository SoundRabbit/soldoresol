use super::atom::{
    btn::{self, Btn},
    fa,
    slider::{self, Slider},
    table::table_tool::{self, TableTool},
    text,
};
use super::organism::popup_color_pallet::{self, PopupColorPallet};
use super::template::common::Common;
use crate::libs::select_list::SelectList;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {}

pub enum Msg {
    NoOp,
    SetSetectedToolIdx(usize),
    SetSelectedTool(TableTool),
}

pub enum On {}

pub struct TableMenu {
    tools: SelectList<TableTool>,
}

impl Component for TableMenu {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for TableMenu {
    fn constructor(props: &Props) -> Self {
        Self {
            tools: SelectList::new(
                vec![
                    TableTool::Selecter(Rc::new(table_tool::Selecter::Point)),
                    TableTool::Pen(Rc::new(table_tool::Pen {
                        color: crate::libs::color::Pallet::gray(9),
                        width: 0.5,
                    })),
                    TableTool::Eraser(Rc::new(table_tool::Eraser { width: 1.0 })),
                    TableTool::Character(Rc::new(table_tool::Character {
                        name: String::from(""),
                    })),
                    TableTool::Boxblock(Rc::new(table_tool::Boxblock {
                        color: crate::libs::color::Pallet::blue(5),
                        size: [1.0, 1.0, 1.0],
                    })),
                ],
                0,
            ),
        }
    }
}

impl Update for TableMenu {
    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetSelectedTool(tool) => {
                if let Some(selected) = self.tools.selected_mut() {
                    *selected = tool;
                }
                Cmd::none()
            }
            Msg::SetSetectedToolIdx(idx) => {
                self.tools.set_selected_idx(idx);
                Cmd::none()
            }
        }
    }
}

impl Render for TableMenu {
    fn render(&self, _props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![self.render_icon_list(), self.render_tool_option()],
        ))
    }
}

impl TableMenu {
    fn render_icon_list(&self) -> Html<Self> {
        let selected_idx = self.tools.selected_idx();
        Html::div(
            Attributes::new().class(Self::class("icons")),
            Events::new(),
            self.tools
                .iter()
                .enumerate()
                .map(|(idx, tool)| Self::render_icon(tool, idx, selected_idx))
                .collect(),
        )
    }

    fn render_icon(tool: &TableTool, idx: usize, selected_idx: usize) -> Html<Self> {
        let (title, child) = match tool {
            TableTool::Selecter(..) => ("選択", fa::i("fa-mouse-pointer")),
            TableTool::Pen(..) => ("鉛筆", fa::i("fa-pencil-alt")),
            TableTool::Eraser(..) => ("消しゴム", fa::i("fa-eraser")),
            TableTool::Character(..) => ("キャラコマ", fa::i("fa-user")),
            TableTool::Boxblock(..) => ("ブロック", fa::i("fa-cube")),
        };
        Html::div(
            Attributes::new().class(Self::class("icon")),
            Events::new(),
            vec![
                Btn::with_variant(
                    if idx == selected_idx {
                        btn::Variant::Primary
                    } else {
                        btn::Variant::Secondary
                    },
                    Attributes::new().title(title),
                    Events::new().on_click(move |_| Msg::SetSetectedToolIdx(idx)),
                    vec![child],
                ),
                text::span(title),
            ],
        )
    }

    fn render_tool_option(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("option")),
            Events::new(),
            vec![match self.tools.selected() {
                Some(TableTool::Pen(tool)) => Self::render_tool_option_pen(tool),
                Some(TableTool::Boxblock(tool)) => Self::render_tool_option_boxblock(tool),
                _ => Html::none(),
            }],
        )
    }

    fn render_tool_option_pen(pen: &Rc<table_tool::Pen>) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("pen")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Common::keyvalue()),
                Events::new(),
                vec![
                    text::span("線幅"),
                    Slider::empty(
                        slider::Props {
                            position: slider::Position::Linear {
                                val: pen.width,
                                min: 0.1,
                                max: 10.0,
                                step: 0.1,
                            },
                            range_is_editable: false,
                            theme: slider::Theme::Light,
                        },
                        Sub::map({
                            let pen = Rc::clone(&pen);
                            move |sub| match sub {
                                slider::On::Input(width) => {
                                    let mut pen = pen.as_ref().clone();
                                    pen.width = width;
                                    Msg::SetSelectedTool(TableTool::Pen(Rc::new(pen)))
                                }
                                _ => Msg::NoOp,
                            }
                        }),
                    ),
                    text::span("色"),
                    PopupColorPallet::empty(
                        popup_color_pallet::Props {
                            default_selected: pen.color,
                            direction: popup_color_pallet::Direction::Bottom,
                        },
                        Sub::map({
                            let pen = Rc::clone(&pen);
                            move |sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    let mut pen = pen.as_ref().clone();
                                    pen.color = color;
                                    Msg::SetSelectedTool(TableTool::Pen(Rc::new(pen)))
                                }
                            }
                        }),
                    ),
                ],
            )],
        )
    }

    fn render_tool_option_boxblock(boxblock: &Rc<table_tool::Boxblock>) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("boxblock")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Common::keyvalue()),
                Events::new(),
                vec![
                    text::span("X幅"),
                    Self::render_tool_option_boxblock_size(boxblock.size[0], {
                        let boxblock = Rc::clone(&boxblock);
                        move |x| {
                            let mut boxblock = boxblock.as_ref().clone();
                            boxblock.size[0] = x;
                            Msg::SetSelectedTool(TableTool::Boxblock(Rc::new(boxblock)))
                        }
                    }),
                    text::span("Y幅"),
                    Self::render_tool_option_boxblock_size(boxblock.size[1], {
                        let boxblock = Rc::clone(&boxblock);
                        move |y| {
                            let mut boxblock = boxblock.as_ref().clone();
                            boxblock.size[1] = y;
                            Msg::SetSelectedTool(TableTool::Boxblock(Rc::new(boxblock)))
                        }
                    }),
                    text::span("Z幅"),
                    Self::render_tool_option_boxblock_size(boxblock.size[2], {
                        let boxblock = Rc::clone(&boxblock);
                        move |z| {
                            let mut boxblock = boxblock.as_ref().clone();
                            boxblock.size[2] = z;
                            Msg::SetSelectedTool(TableTool::Boxblock(Rc::new(boxblock)))
                        }
                    }),
                    text::span("色"),
                    PopupColorPallet::empty(
                        popup_color_pallet::Props {
                            default_selected: boxblock.color,
                            direction: popup_color_pallet::Direction::Bottom,
                        },
                        Sub::map({
                            let boxblock = Rc::clone(&boxblock);
                            move |sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    let mut boxblock = boxblock.as_ref().clone();
                                    boxblock.color = color;
                                    Msg::SetSelectedTool(TableTool::Boxblock(Rc::new(boxblock)))
                                }
                            }
                        }),
                    ),
                ],
            )],
        )
    }

    fn render_tool_option_boxblock_size(
        val: f64,
        mut f: impl FnMut(f64) -> Msg + 'static,
    ) -> Html<Self> {
        Slider::empty(
            slider::Props {
                position: slider::Position::Linear {
                    val: val,
                    min: 0.1,
                    max: 10.0,
                    step: 0.1,
                },
                range_is_editable: false,
                theme: slider::Theme::Light,
            },
            Sub::map(move |sub| match sub {
                slider::On::Input(val) => f(val),
                _ => Msg::NoOp,
            }),
        )
    }
}

impl Styled for TableMenu {
    fn style() -> Style {
        style! {
            ".base" {
                "height": "100%";
                "color": crate::libs::color::Pallet::gray(0);
                "position": "relative";
            }

            ".icons" {
                "background-color": crate::libs::color::Pallet::gray(8);
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": ".65rem";
                "padding": ".65rem .35rem";
                "height": "100%";
            }

            ".icon" {
                "display": "flex";
                "flex-direction": "column";
                "align-items": "stretch";
            }

            ".icon > span" {
                "font-size": ".85rem";
                "text-align": "center";
            }

            ".option" {
                "position": "absolute";
                "top": ".65rem";
                "left": "calc(100% + .35rem)";
                "z-index": super::constant::z_index::MASK;
                "min-width": "max-content";
                "color": crate::libs::color::Pallet::gray(9);
            }
        }
    }
}
