use super::super::model::table::{
    BoxblockTool, CharacterTool, EraserTool, FillShapeTool, LineShapeTool, PenTool, PointlightTool,
    ShapeTool, TableTool, TerranblockTool,
};
use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::slider::{self, Slider};
use super::atom::text;
use super::modal_imported_files::{self, ModalImportedFiles};
use super::molecule::color_pallet::{self, ColorPallet};
use super::molecule::tab_menu::{self, TabMenu};
use super::util::styled::{Style, Styled};
use super::util::Prop;
use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self, ResourceId};
use crate::libs::clone_of::CloneOf;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;

pub struct Props {
    pub tools: Prop<SelectList<TableTool>>,
    pub block_arena: block::ArenaRef,
    pub resource_arena: resource::ArenaRef,
    pub selecting_table_id: BlockId,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetSelectedIdx(usize),
    SetShowSub(bool),
    SetModal(Modal),
    CloseModalSub(On),
}

pub enum On {
    ChangeSelectedIdx {
        idx: usize,
    },
    SetSelectedTool {
        tool: TableTool,
    },
    ChangeTableProps {
        table_id: BlockId,
        size: Option<[f32; 2]>,
        grid_color: Option<Pallet>,
        background_color: Option<Pallet>,
        background_image: Option<Option<ResourceId>>,
        env_light_intensity: Option<f32>,
    },
}

pub struct SideMenu {
    tools: Prop<SelectList<TableTool>>,
    selected_idx: usize,
    block_arena: block::ArenaRef,
    resource_arena: resource::ArenaRef,
    selecting_table_id: BlockId,

    show_sub: bool,
    modal: Modal,
}

pub enum Modal {
    None,
    SelectTableBackgroundImage,
    SelectCharacterTexture(CharacterTool),
}

impl Constructor for SideMenu {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        let selected_idx = props.tools.selected_idx();

        Self {
            tools: props.tools,
            selected_idx: selected_idx,
            block_arena: props.block_arena,
            resource_arena: props.resource_arena,
            selecting_table_id: props.selecting_table_id,

            show_sub: false,
            modal: Modal::None,
        }
    }
}

impl Component for SideMenu {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.tools = props.tools;
        self.block_arena = props.block_arena;
        if self.selected_idx != self.tools.selected_idx() {
            self.selected_idx = self.tools.selected_idx();
            self.show_sub = true;
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
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
            Msg::SetModal(modal) => {
                self.modal = modal;
                Cmd::none()
            }
            Msg::CloseModalSub(sub) => {
                self.modal = Modal::None;
                Cmd::sub(sub)
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                self.render_main(),
                self.render_sub(),
                match &self.modal {
                    Modal::None => Html::none(),
                    Modal::SelectTableBackgroundImage => self.render_modal_select_image({
                        let table_id = BlockId::clone(&self.selecting_table_id);
                        move |r_id| {
                            Msg::CloseModalSub(On::ChangeTableProps {
                                table_id,
                                size: None,
                                grid_color: None,
                                background_color: None,
                                background_image: Some(Some(r_id)),
                                env_light_intensity: None,
                            })
                        }
                    }),
                    Modal::SelectCharacterTexture(character) => self.render_modal_select_image({
                        let mut character = CharacterTool::clone_of(character);
                        move |r_id| {
                            character.tex_id = Some(r_id);
                            Msg::Sub(On::SetSelectedTool {
                                tool: TableTool::Character(character),
                            })
                        }
                    }),
                },
            ],
        ))
    }
}

impl SideMenu {
    fn render_modal_select_image(&self, msg: impl FnOnce(ResourceId) -> Msg + 'static) -> Html {
        ModalImportedFiles::empty(
            modal_imported_files::Props {
                resource_arena: resource::ArenaRef::clone(&self.resource_arena),
            },
            Subscription::new({
                move |sub| match sub {
                    modal_imported_files::On::Close => Msg::SetModal(Modal::None),
                    modal_imported_files::On::SelectFile(r_id) => msg(r_id),
                }
            }),
        )
    }

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
                            TableTool::TableEditor => "fa-vector-square",
                            TableTool::Pen(..) => "fa-pen",
                            TableTool::Shape(..) => "fa-shapes",
                            TableTool::Eraser(..) => "fa-eraser",
                            TableTool::Character(..) => "fa-users",
                            TableTool::Boxblock(..) => "fa-cube",
                            TableTool::Terranblock(..) => "fa-cubes",
                            TableTool::Pointlight(..) => "fa-lightbulb",
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
                    Some(TableTool::TableEditor) => self.render_sub_table_editor(),
                    Some(TableTool::Eraser(tool)) => self.render_sub_eraser(tool),
                    Some(TableTool::Character(tool)) => self.render_sub_character(tool),
                    Some(TableTool::Boxblock(tool)) => self.render_sub_boxblock(tool),
                    Some(TableTool::Pointlight(tool)) => self.render_sub_pointlight(tool),
                    Some(TableTool::Terranblock(tool)) => self.render_sub_terranblock(tool),
                    _ => Html::div(Attributes::new(), Events::new(), vec![]),
                },
            ],
        )
    }

    fn render_sub_table_editor(&self) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
            Events::new(),
            self.block_arena
                .map(&self.selecting_table_id, |table: &block::table::Table| {
                    let [width, height] = *table.size();
                    let bg_image_id = table.background_texture_id();
                    vec![
                        text::div("幅（x方向）"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Inf {
                                    val: width as f64,
                                    mid: 20.0,
                                    step: 1.0,
                                },
                                range_is_editable: false,
                            },
                            Subscription::new({
                                let table_id = BlockId::clone(&self.selecting_table_id);
                                move |sub| match sub {
                                    slider::On::Input(width) => Msg::Sub(On::ChangeTableProps {
                                        table_id,
                                        size: Some([width as f32, height]),
                                        grid_color: None,
                                        background_color: None,
                                        background_image: None,
                                        env_light_intensity: None,
                                    }),
                                    _ => Msg::NoOp,
                                }
                            }),
                        ),
                        text::div("奥行き（y方向）"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Inf {
                                    val: height as f64,
                                    mid: 20.0,
                                    step: 1.0,
                                },
                                range_is_editable: false,
                            },
                            Subscription::new({
                                let table_id = BlockId::clone(&self.selecting_table_id);
                                move |sub| match sub {
                                    slider::On::Input(height) => Msg::Sub(On::ChangeTableProps {
                                        table_id,
                                        size: Some([width, height as f32]),
                                        grid_color: None,
                                        background_color: None,
                                        background_image: None,
                                        env_light_intensity: None,
                                    }),
                                    _ => Msg::NoOp,
                                }
                            }),
                        ),
                        text::div("環境光"),
                        text::div("［明るさ］"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    val: table.env_light_intensity() as f64,
                                    min: 0.0,
                                    max: 1.0,
                                    step: 0.1,
                                },
                                range_is_editable: false,
                            },
                            Subscription::new({
                                let table_id = BlockId::clone(&self.selecting_table_id);
                                move |sub| match sub {
                                    slider::On::Input(env_light_intensity) => {
                                        Msg::Sub(On::ChangeTableProps {
                                            table_id,
                                            size: None,
                                            grid_color: None,
                                            background_color: None,
                                            background_image: None,
                                            env_light_intensity: Some(env_light_intensity as f32),
                                        })
                                    }
                                    _ => Msg::NoOp,
                                }
                            }),
                        ),
                        TabMenu::with_children(
                            tab_menu::Props {
                                selected: 0,
                                tabs: vec![String::from("色 1"), String::from("色 2")],
                                controlled: false,
                            },
                            Subscription::none(),
                            vec![
                                ColorPallet::empty(
                                    color_pallet::Props {
                                        default_selected: table.grid_color().clone(),
                                        ..Default::default()
                                    },
                                    Subscription::new({
                                        let table_id = BlockId::clone(&self.selecting_table_id);
                                        move |sub| match sub {
                                            color_pallet::On::SelectColor(color) => {
                                                Msg::Sub(On::ChangeTableProps {
                                                    table_id,
                                                    size: None,
                                                    grid_color: Some(color),
                                                    background_color: None,
                                                    background_image: None,
                                                    env_light_intensity: None,
                                                })
                                            }
                                        }
                                    }),
                                ),
                                ColorPallet::empty(
                                    color_pallet::Props {
                                        default_selected: table.background_color().clone(),
                                        ..Default::default()
                                    },
                                    Subscription::new({
                                        let table_id = BlockId::clone(&self.selecting_table_id);
                                        move |sub| match sub {
                                            color_pallet::On::SelectColor(color) => {
                                                Msg::Sub(On::ChangeTableProps {
                                                    table_id,
                                                    size: None,
                                                    grid_color: None,
                                                    background_color: Some(color),
                                                    background_image: None,
                                                    env_light_intensity: None,
                                                })
                                            }
                                        }
                                    }),
                                ),
                            ],
                        ),
                        bg_image_id
                            .and_then(|r_id| {
                                self.resource_arena.get_as::<resource::ImageData>(r_id)
                            })
                            .map(|bg_image| {
                                Html::img(
                                    Attributes::new().src(bg_image.url().as_ref()),
                                    Events::new(),
                                    vec![],
                                )
                            })
                            .unwrap_or(Html::none()),
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Primary,
                            },
                            Subscription::new(|sub| match sub {
                                btn::On::Click => Msg::SetModal(Modal::SelectTableBackgroundImage),
                            }),
                            Html::text("画像を選択する"),
                        ),
                    ]
                })
                .unwrap_or(vec![]),
        )
    }

    fn render_sub_pen(&self, tool: &PenTool) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
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
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                ColorPallet::empty(
                    color_pallet::Props {
                        default_selected: tool.pallet.clone(),
                        title: Some(String::from("ペン色")),
                    },
                    Subscription::new({
                        let mut tool = PenTool::clone(tool);
                        move |sub| match sub {
                            color_pallet::On::SelectColor(pallet) => {
                                tool.pallet = pallet;
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
                                    let mut tools = SelectList::clone_of(tools);
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
            Attributes::new().class(Self::class("sub-menu")),
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
                        let mut tools = SelectList::clone_of(tools);
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
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                ColorPallet::empty(
                    color_pallet::Props {
                        default_selected: line_shape.pallet.clone(),
                        title: Some(String::from("線色")),
                    },
                    Subscription::new({
                        let mut line_shape = LineShapeTool::clone(line_shape);
                        let mut tools = SelectList::clone_of(tools);
                        move |sub| match sub {
                            color_pallet::On::SelectColor(pallet) => {
                                line_shape.pallet = pallet;
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
            Attributes::new().class(Self::class("sub-menu")),
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
                        let mut fill_shape = FillShapeTool::clone_of(fill_shape);
                        let mut tools = SelectList::clone_of(tools);
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
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                TabMenu::with_children(
                    tab_menu::Props {
                        selected: 0,
                        tabs: vec![String::from("線色"), String::from("塗り潰し色")],
                        controlled: false,
                    },
                    Subscription::none(),
                    vec![
                        ColorPallet::empty(
                            color_pallet::Props {
                                default_selected: fill_shape.line_pallet.clone(),
                                ..Default::default()
                            },
                            Subscription::new({
                                let mut fill_shape = FillShapeTool::clone_of(fill_shape);
                                let mut tools = SelectList::clone_of(tools);
                                move |sub| match sub {
                                    color_pallet::On::SelectColor(pallet) => {
                                        fill_shape.line_pallet = pallet;
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
                                ..Default::default()
                            },
                            Subscription::new({
                                let mut fill_shape = FillShapeTool::clone_of(fill_shape);
                                let mut tools = SelectList::clone_of(tools);
                                move |sub| match sub {
                                    color_pallet::On::SelectColor(pallet) => {
                                        fill_shape.fill_pallet = pallet;
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

    fn render_sub_eraser(&self, tool: &EraserTool) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
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
                        let mut tool = EraserTool::clone(tool);
                        move |sub| match sub {
                            slider::On::Input(val) => {
                                tool.line_width = val;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Eraser(tool),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            min: 0.0,
                            max: 100.0,
                            val: tool.alpha as f64,
                            step: 1.0,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut tool = EraserTool::clone(tool);
                        move |sub| match sub {
                            slider::On::Input(val) => {
                                tool.alpha = val.round() as u8;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Eraser(tool),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
            ],
        )
    }

    fn render_sub_character(&self, character: &CharacterTool) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
            Events::new(),
            vec![
                text::div("コマサイズ"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Inf {
                            val: character.size,
                            mid: 1.0,
                            step: 0.1,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut character = CharacterTool::clone_of(character);
                        move |sub| match sub {
                            slider::On::Input(size) => {
                                character.size = size;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Character(character),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                text::div("キャラクターの身長"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Inf {
                            val: character.height,
                            mid: 1.0,
                            step: 0.1,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut character = CharacterTool::clone_of(character);
                        move |sub| match sub {
                            slider::On::Input(tex_scale) => {
                                character.height = tex_scale;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Character(character),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                character
                    .tex_id
                    .as_ref()
                    .and_then(|r_id| self.resource_arena.get_as::<resource::ImageData>(r_id))
                    .map(|bg_image| {
                        Html::img(
                            Attributes::new().src(bg_image.url().as_ref()),
                            Events::new(),
                            vec![],
                        )
                    })
                    .unwrap_or(Html::none()),
                Btn::with_child(
                    btn::Props {
                        variant: btn::Variant::Primary,
                    },
                    Subscription::new({
                        let character = CharacterTool::clone_of(character);
                        move |sub| match sub {
                            btn::On::Click => {
                                Msg::SetModal(Modal::SelectCharacterTexture(character))
                            }
                        }
                    }),
                    Html::text("画像を選択する"),
                ),
            ],
        )
    }

    fn render_sub_boxblock(&self, boxblock: &BoxblockTool) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
            Events::new(),
            vec![
                text::div("幅（x方向）"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            val: boxblock.size[0],
                            min: 0.0,
                            max: 10.0,
                            step: 0.5,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut boxblock = BoxblockTool::clone_of(boxblock);
                        move |sub| match sub {
                            slider::On::Input(a) => {
                                boxblock.size[0] = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Boxblock(boxblock),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                text::div("奥行き（y方向）"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            val: boxblock.size[1],
                            min: 0.0,
                            max: 10.0,
                            step: 0.5,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut boxblock = BoxblockTool::clone_of(boxblock);
                        move |sub| match sub {
                            slider::On::Input(a) => {
                                boxblock.size[1] = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Boxblock(boxblock),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                text::div("高さ（z方向）"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            val: boxblock.size[2],
                            min: 0.0,
                            max: 10.0,
                            step: 0.5,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut boxblock = BoxblockTool::clone_of(boxblock);
                        move |sub| match sub {
                            slider::On::Input(a) => {
                                boxblock.size[2] = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Boxblock(boxblock),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                Dropdown::with_children(
                    dropdown::Props {
                        variant: btn::Variant::DarkLikeMenu,
                        direction: dropdown::Direction::Bottom,
                        text: match &boxblock.shape {
                            block::boxblock::Shape::Cube => String::from("立方体"),
                            block::boxblock::Shape::Sphere => String::from("球体"),
                            block::boxblock::Shape::Cyliner => String::from("円柱"),
                        },
                        toggle_type: dropdown::ToggleType::Click,
                    },
                    Subscription::none(),
                    vec![
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Menu,
                            },
                            Subscription::new({
                                let mut boxblock = BoxblockTool::clone_of(boxblock);
                                move |sub| match sub {
                                    btn::On::Click => {
                                        boxblock.shape = block::boxblock::Shape::Cube;
                                        Msg::Sub(On::SetSelectedTool {
                                            tool: TableTool::Boxblock(boxblock),
                                        })
                                    }
                                }
                            }),
                            Html::text("立方体"),
                        ),
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Menu,
                            },
                            Subscription::new({
                                let mut boxblock = BoxblockTool::clone_of(boxblock);
                                move |sub| match sub {
                                    btn::On::Click => {
                                        boxblock.shape = block::boxblock::Shape::Sphere;
                                        Msg::Sub(On::SetSelectedTool {
                                            tool: TableTool::Boxblock(boxblock),
                                        })
                                    }
                                }
                            }),
                            Html::text("球体"),
                        ),
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Menu,
                            },
                            Subscription::new({
                                let mut boxblock = BoxblockTool::clone_of(boxblock);
                                move |sub| match sub {
                                    btn::On::Click => {
                                        boxblock.shape = block::boxblock::Shape::Cyliner;
                                        Msg::Sub(On::SetSelectedTool {
                                            tool: TableTool::Boxblock(boxblock),
                                        })
                                    }
                                }
                            }),
                            Html::text("円柱"),
                        ),
                    ],
                ),
                ColorPallet::empty(
                    color_pallet::Props {
                        default_selected: boxblock.color.clone(),
                        title: Some(String::from("ブロック色")),
                    },
                    Subscription::new({
                        let mut boxblock = BoxblockTool::clone_of(boxblock);
                        move |sub| match sub {
                            color_pallet::On::SelectColor(a) => {
                                boxblock.color = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Boxblock(boxblock),
                                })
                            }
                        }
                    }),
                ),
            ],
        )
    }

    fn render_sub_terranblock(&self, terranblock: &TerranblockTool) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
            Events::new(),
            vec![ColorPallet::empty(
                color_pallet::Props {
                    default_selected: terranblock.color.clone(),
                    title: Some(String::from("ブロック色")),
                },
                Subscription::new({
                    let mut terranblock = TerranblockTool::clone_of(terranblock);
                    move |sub| match sub {
                        color_pallet::On::SelectColor(a) => {
                            terranblock.color = a;
                            Msg::Sub(On::SetSelectedTool {
                                tool: TableTool::Terranblock(terranblock),
                            })
                        }
                    }
                }),
            )],
        )
    }

    fn render_sub_pointlight(&self, pointlight: &PointlightTool) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("sub-body"))
                .class(Self::class("sub-menu")),
            Events::new(),
            vec![
                text::div("明るさ"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            val: pointlight.light_intensity,
                            min: 0.0,
                            max: 20.0,
                            step: 0.1,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut pointlight = PointlightTool::clone_of(pointlight);
                        move |sub| match sub {
                            slider::On::Input(a) => {
                                pointlight.light_intensity = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Pointlight(pointlight),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                text::div("減衰開始距離"),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            val: pointlight.light_attenation,
                            min: 0.0,
                            max: 10.0,
                            step: 0.5,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let mut pointlight = PointlightTool::clone_of(pointlight);
                        move |sub| match sub {
                            slider::On::Input(a) => {
                                pointlight.light_attenation = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Pointlight(pointlight),
                                })
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
                ColorPallet::empty(
                    color_pallet::Props {
                        default_selected: pointlight.color.clone(),
                        title: Some(String::from("光源色")),
                    },
                    Subscription::new({
                        let mut pointlight = PointlightTool::clone_of(pointlight);
                        move |sub| match sub {
                            color_pallet::On::SelectColor(a) => {
                                pointlight.color = a;
                                Msg::Sub(On::SetSelectedTool {
                                    tool: TableTool::Pointlight(pointlight),
                                })
                            }
                        }
                    }),
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
                "padding-bottom": "0.65em";
            }

            "sub-menu" {
                "display": "grid";
                "grid-template-columns": "100%";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "row";
                "row-gap": "0.65rem";
                "justify-items": "stretch";
                "overflow-y": "scroll";
            }

            "sub-menu img" {
                "width": "100%";
            }
        }
    }
}
