use super::super::model::table::{FillShapeTool, LineShapeTool, PenTool, ShapeTool, TableTool};
use super::atom::btn::{self, Btn};
use super::atom::fa;
use super::atom::slider::{self, Slider};
use super::molecule::color_pallet::{self, ColorPallet};
use super::molecule::tab_menu::{self, TabMenu};
use super::util::styled::{Style, Styled};
use super::util::Prop;
use crate::libs::clone_ref::CloneRef;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;

pub struct Props {
    pub tools: Prop<SelectList<TableTool>>,
}

pub enum Msg {
    Sub(On),
    SetSelectedIdx(usize),
    SetShowSub(bool),
}

pub enum On {
    ChangeSelectedIdx { idx: usize },
    SetSelectedTool { tool: TableTool },
}

pub struct SideMenu {
    tools: Prop<SelectList<TableTool>>,
    selected_idx: usize,

    show_sub: bool,
}

impl Constructor for SideMenu {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        let selected_idx = props.tools.selected_idx();

        Self {
            tools: props.tools,
            selected_idx: selected_idx,
            show_sub: false,
        }
    }
}

impl Component for SideMenu {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.tools = props.tools;
        if self.selected_idx != self.tools.selected_idx() {
            self.selected_idx = self.tools.selected_idx();
            self.show_sub = true;
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Sub(sub) => Cmd::Sub(sub),
            Msg::SetSelectedIdx(idx) => {
                if idx != self.tools.selected_idx() {
                    Cmd::sub(On::ChangeSelectedIdx { idx })
                } else {
                    Cmd::none()
                }
            }
            Msg::SetShowSub(show_sub) => {
                self.show_sub = show_sub;
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![self.render_main(), self.render_sub()],
        ))
    }
}

impl SideMenu {
    fn render_main(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            self.tools
                .iter()
                .enumerate()
                .map(|(tool_idx, table_tool)| match table_tool {
                    TableTool::Hr(text) => Html::div(
                        Attributes::new().class(Self::class("main-hr")),
                        Events::new(),
                        vec![Html::text(text as &String)],
                    ),
                    _ => Btn::with_child(
                        btn::Props {
                            variant: if tool_idx == self.tools.selected_idx() {
                                btn::Variant::Primary
                            } else {
                                btn::Variant::Dark
                            },
                        },
                        Subscription::new(move |sub| match sub {
                            btn::On::Click => Msg::SetSelectedIdx(tool_idx),
                        }),
                        fa::i(match table_tool {
                            TableTool::Hr(..) => unreachable!(),
                            TableTool::Selector => "fa-mouse-pointer",
                            TableTool::Pen(..) => "fa-pen",
                            TableTool::Shape(..) => "fa-shapes",
                            TableTool::Eraser => "fa-eraser",
                        }),
                    ),
                })
                .collect(),
        )
    }

    fn render_sub(&self) -> Html {
        if self.show_sub {
            self.render_sub_opend()
        } else {
            self.render_sub_closed()
        }
    }

    fn render_sub_closed(&self) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub"))
                .class(Self::class("sub--closed")),
            Events::new(),
            vec![Btn::with_child(
                btn::Props {
                    variant: btn::Variant::Dark,
                },
                Subscription::new(|sub| match sub {
                    btn::On::Click => Msg::SetShowSub(true),
                }),
                fa::i("fa-caret-right"),
            )],
        )
    }

    fn render_sub_opend(&self) -> Html {
        let selected_tool_name = self.tools.selected().map(|tool| tool.name()).unwrap_or("");
        Html::div(
            Attributes::new()
                .class(Self::class("sub"))
                .class(Self::class("sub--opend")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("sub-header")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text(format!("［{}］ツール", selected_tool_name))],
                        ),
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Dark,
                            },
                            Subscription::new(|sub| match sub {
                                btn::On::Click => Msg::SetShowSub(false),
                            }),
                            fa::i("fa-caret-left"),
                        ),
                    ],
                ),
                match self.tools.selected() {
                    Some(TableTool::Pen(tool)) => self.render_sub_pen(tool),
                    Some(TableTool::Shape(tool)) => self.render_sub_shape(tool),
                    _ => Html::div(Attributes::new(), Events::new(), vec![]),
                },
            ],
        )
    }

    fn render_sub_pen(&self, tool: &PenTool) -> Html {
        Html::div(
            Attributes::new().class(Self::class("sub-body")),
            Events::new(),
            vec![
                Html::div(Attributes::new(), Events::new(), vec![Html::text("線幅")]),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Inf {
                            val: tool.line_width,
                            mid: 2.0,
                            step: 0.01,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut tool = PenTool::clone(tool);
                        move |sub| match sub {
                            slider::On::Input(val) => {
                                tool.line_width = val;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Pen(tool),
                                })
                            }
                        }
                    }),
                ),
                ColorPallet::empty(
                    color_pallet::Props {
                        default_selected: tool.pallet.clone(),
                        alpha: tool.alpha,
                    },
                    Subscription::new({
                        let mut tool = PenTool::clone(tool);
                        move |sub| match sub {
                            color_pallet::On::SetColor(pallet, alpha) => {
                                tool.pallet = pallet;
                                tool.alpha = alpha;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Pen(tool),
                                })
                            }
                        }
                    }),
                ),
            ],
        )
    }

    fn render_sub_shape(&self, tools: &SelectList<ShapeTool>) -> Html {
        Html::div(
            Attributes::new().class(Self::class("sub-body")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("sub-tool-list")),
                    Events::new(),
                    tools
                        .iter()
                        .enumerate()
                        .map(|(tool_idx, shape_tool)| {
                            Btn::with_children(
                                btn::Props {
                                    variant: if tool_idx == tools.selected_idx() {
                                        btn::Variant::Primary
                                    } else {
                                        btn::Variant::Dark
                                    },
                                },
                                Subscription::new({
                                    let mut tools = SelectList::clone(tools);
                                    move |sub| match sub {
                                        btn::On::Click => {
                                            tools.set_selected_idx(tool_idx);
                                            Msg::Sub(On::SetSelectedTool {
                                                tool: TableTool::Shape(tools),
                                            })
                                        }
                                    }
                                }),
                                match shape_tool {
                                    ShapeTool::Line(..) => {
                                        vec![fa::i("fa-slash"), Html::text(" 直線")]
                                    }
                                    ShapeTool::Rect(..) => {
                                        vec![fa::far_i("fa-square"), Html::text(" 長方形")]
                                    }
                                    ShapeTool::Ellipse(..) => {
                                        vec![fa::far_i("fa-circle"), Html::text(" 楕円")]
                                    }
                                },
                            )
                        })
                        .collect(),
                ),
                match tools.selected() {
                    Some(ShapeTool::Line(line_shape)) => {
                        self.render_sub_shape_line(line_shape, tools)
                    }
                    Some(ShapeTool::Rect(fill_shape)) => {
                        self.render_sub_shape_fill(fill_shape, tools)
                    }
                    Some(ShapeTool::Ellipse(fill_shape)) => {
                        self.render_sub_shape_fill(fill_shape, tools)
                    }
                    _ => Html::none(),
                },
            ],
        )
    }

    fn render_sub_shape_line(
        &self,
        line_shape: &LineShapeTool,
        tools: &SelectList<ShapeTool>,
    ) -> Html {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Html::div(Attributes::new(), Events::new(), vec![Html::text("線幅")]),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Inf {
                            val: line_shape.line_width,
                            mid: 2.0,
                            step: 0.01,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut line_shape = LineShapeTool::clone(line_shape);
                        let mut tools = SelectList::clone(tools);
                        move |sub| match sub {
                            slider::On::Input(val) => {
                                line_shape.line_width = val;
                                if let Some(tool) = tools.selected_mut() {
                                    *tool = ShapeTool::Line(line_shape);
                                }
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Shape(tools),
                                })
                            }
                        }
                    }),
                ),
                ColorPallet::empty(
                    color_pallet::Props {
                        default_selected: line_shape.pallet.clone(),
                        alpha: line_shape.alpha,
                    },
                    Subscription::new({
                        let mut line_shape = LineShapeTool::clone(line_shape);
                        let mut tools = SelectList::clone(tools);
                        move |sub| match sub {
                            color_pallet::On::SetColor(pallet, alpha) => {
                                line_shape.pallet = pallet;
                                line_shape.alpha = alpha;
                                if let Some(tool) = tools.selected_mut() {
                                    *tool = ShapeTool::Line(line_shape);
                                }
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Shape(tools),
                                })
                            }
                        }
                    }),
                ),
            ],
        )
    }

    fn render_sub_shape_fill(
        &self,
        fill_shape: &FillShapeTool,
        tools: &SelectList<ShapeTool>,
    ) -> Html {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Html::div(Attributes::new(), Events::new(), vec![Html::text("線幅")]),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Inf {
                            val: fill_shape.line_width,
                            mid: 2.0,
                            step: 0.01,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut fill_shape = FillShapeTool::clone(fill_shape);
                        let mut tools = SelectList::clone(tools);
                        move |sub| match sub {
                            slider::On::Input(val) => {
                                fill_shape.line_width = val;
                                if let Some(tool) = tools.selected_mut() {
                                    tool.set_fill(fill_shape);
                                }
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Shape(tools),
                                })
                            }
                        }
                    }),
                ),
                TabMenu::with_children(
                    tab_menu::Props {
                        selected: 0,
                        tabs: vec![String::from("線"), String::from("塗りつぶし")],
                        controlled: false,
                    },
                    Subscription::none(),
                    vec![
                        ColorPallet::empty(
                            color_pallet::Props {
                                default_selected: fill_shape.line_pallet.clone(),
                                alpha: fill_shape.line_alpha,
                            },
                            Subscription::new({
                                let mut fill_shape = FillShapeTool::clone(fill_shape);
                                let mut tools = SelectList::clone(tools);
                                move |sub| match sub {
                                    color_pallet::On::SetColor(pallet, alpha) => {
                                        fill_shape.line_pallet = pallet;
                                        fill_shape.line_alpha = alpha;
                                        if let Some(tool) = tools.selected_mut() {
                                            tool.set_fill(fill_shape);
                                        }
                                        Msg::Sub(On::SetSelectedTool {
                                            tool: TableTool::Shape(tools),
                                        })
                                    }
                                }
                            }),
                        ),
                        ColorPallet::empty(
                            color_pallet::Props {
                                default_selected: fill_shape.fill_pallet.clone(),
                                alpha: fill_shape.fill_alpha,
                            },
                            Subscription::new({
                                let mut fill_shape = FillShapeTool::clone(fill_shape);
                                let mut tools = SelectList::clone(tools);
                                move |sub| match sub {
                                    color_pallet::On::SetColor(pallet, alpha) => {
                                        fill_shape.fill_pallet = pallet;
                                        fill_shape.fill_alpha = alpha;
                                        if let Some(tool) = tools.selected_mut() {
                                            tool.set_fill(fill_shape);
                                        }
                                        Msg::Sub(On::SetSelectedTool {
                                            tool: TableTool::Shape(tools),
                                        })
                                    }
                                }
                            }),
                        ),
                    ],
                ),
            ],
        )
    }
}

impl Styled for SideMenu {
    fn style() -> Style {
        style! {
            "base" {
                "background-color": format!("{}", crate::libs::color::color_system::gray(100,8));
                "color": format!("{}", crate::libs::color::color_system::gray(100,0));
                "position": "relative";
                "height": "100%";
            }

            "main" {
                "display": "grid";
                "grid-auto-flow": "row";
                "padding": "0.65em";
                "row-gap": "0.65em";
                "align-content": "start";
                "align-items": "center";
                "height": "100%";
                "border-right": format!("0.1em solid {}", crate::libs::color::color_system::gray(100, 9));
            }

            "sub" {
                "position": "absolute";
                "left": "100%";
                "top": "0";
            }

            "sub--closed" {
                "padding": "0.65em";
            }

            "sub--opend" {
                "background-color": format!("{}", crate::libs::color::color_system::gray(100,8));
                "min-width": "20rem";
                "max-width": "10vw";
                "height": "100%";
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "grid-template-columns": "1fr";
                "border-right": format!("0.1em solid {}", crate::libs::color::color_system::gray(100, 9));
            }

            "sub-header" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "align-items": "center";
                "padding": "0.65em";
                "border-bottom": format!("0.1em solid {}", crate::libs::color::color_system::gray(100, 9));
            }

            "sub-body" {
                "padding": "0.65em";
            }

            "sub-tool-list" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
                "grid-auto-flow": "row";
                "column-gap": "0.65em";
                "row-gap": "0.65em";
            }
        }
    }
}
