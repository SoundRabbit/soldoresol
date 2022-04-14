use super::atom::{
    btn::{self, Btn},
    common::Common,
    dropdown::{self, Dropdown},
    fa,
    slider::{self, Slider},
    text,
};
use super::organism::{
    modal_resource::{self, ModalResource},
    popup_color_pallet::{self, PopupColorPallet},
};
use crate::arena::{block, ArenaMut, BlockKind, BlockMut};
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use crate::table::table_tool::{self, TableTool};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetSetectedToolIdx(usize),
    SetTool(usize, TableTool),
    SetShowingModal(ShowingModal),
}

pub enum On {
    SelectTool(TableTool),
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct TableMenu {
    tools: SelectList<TableTool>,
    showing_modal: ShowingModal,
    arena: ArenaMut,
    world: BlockMut<block::World>,
}

pub enum ShowingModal {
    None,
    SelectBlockTexture(usize, Rc<table_tool::Boxblock>),
    SelectCharacterTexture(usize, Rc<table_tool::Character>),
}

impl Component for TableMenu {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for TableMenu {}

impl Constructor for TableMenu {
    fn constructor(props: Self::Props) -> Self {
        Self {
            tools: SelectList::new(
                vec![
                    TableTool::Selecter(Rc::new(table_tool::Selecter::Point)),
                    TableTool::Pen(Rc::new(table_tool::Pen {
                        color: crate::libs::color::Pallet::gray(9),
                        width: 0.5,
                    })),
                    TableTool::Craftboard(Rc::new(table_tool::Craftboard {
                        size: [10.0, 10.0, 10.0],
                    })),
                    TableTool::Eraser(Rc::new(table_tool::Eraser { width: 1.0 })),
                    TableTool::Character(Rc::new(table_tool::Character {
                        size: 1.0,
                        tex_size: 1.5,
                        color: crate::libs::color::Pallet::gray(5),
                        texture: None,
                    })),
                    TableTool::Boxblock(Rc::new(table_tool::Boxblock {
                        color: crate::libs::color::Pallet::blue(5),
                        size: [1.0, 1.0, 1.0],
                        texture: None,
                        shape: block::boxblock::Shape::Cube,
                    })),
                ],
                0,
            ),
            showing_modal: ShowingModal::None,
            arena: props.arena,
            world: props.world,
        }
    }
}

impl TableMenu {
    pub fn initial_selected() -> TableTool {
        TableTool::Selecter(Rc::new(table_tool::Selecter::Point))
    }
}

impl Update for TableMenu {
    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetTool(idx, updated) => {
                self.showing_modal = ShowingModal::None;
                if let Some(tool) = self.tools.get_mut(idx) {
                    *tool = updated.clone();
                    if idx == self.tools.selected_idx() {
                        return Cmd::submit(On::SelectTool(updated));
                    }
                }
                Cmd::none()
            }
            Msg::SetSetectedToolIdx(idx) => {
                self.tools.set_selected_idx(idx);
                if let Some(selected) = self.tools.selected() {
                    Cmd::submit(On::SelectTool(selected.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for TableMenu {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                self.render_icon_list(),
                self.render_tool_option(),
                self.render_modal(),
            ],
        ))
    }
}

impl TableMenu {
    fn render_modal(&self) -> Html {
        match &self.showing_modal {
            ShowingModal::None => Common::none(),
            ShowingModal::SelectBlockTexture(tool_idx, tool) => ModalResource::empty(
                self,
                None,
                modal_resource::Props {
                    arena: ArenaMut::clone(&self.arena),
                    world: BlockMut::clone(&self.world),
                    filter: set! { BlockKind::BlockTexture },
                    title: String::from(modal_resource::title::VIEW_ALL_RESOURCE),
                    is_selecter: true,
                },
                Sub::map({
                    let tool_idx = *tool_idx;
                    let tool = Rc::clone(&tool);
                    move |sub| match sub {
                        modal_resource::On::UpdateBlocks { insert, update } => {
                            Msg::Sub(On::UpdateBlocks { insert, update })
                        }
                        modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                        modal_resource::On::SelectBlockTexture(texture) => {
                            let mut tool = tool.as_ref().clone();
                            tool.texture = Some(texture);
                            Msg::SetTool(tool_idx, TableTool::Boxblock(Rc::new(tool)))
                        }
                        modal_resource::On::SelectNone => {
                            let mut tool = tool.as_ref().clone();
                            tool.texture = None;
                            Msg::SetTool(tool_idx, TableTool::Boxblock(Rc::new(tool)))
                        }
                        _ => Msg::NoOp,
                    }
                }),
            ),
            ShowingModal::SelectCharacterTexture(tool_idx, tool) => ModalResource::empty(
                self,
                None,
                modal_resource::Props {
                    arena: ArenaMut::clone(&self.arena),
                    world: BlockMut::clone(&self.world),
                    filter: set! { BlockKind::ImageData },
                    title: String::from(modal_resource::title::VIEW_ALL_RESOURCE),
                    is_selecter: true,
                },
                Sub::map({
                    let tool_idx = *tool_idx;
                    let tool = Rc::clone(&tool);
                    move |sub| match sub {
                        modal_resource::On::UpdateBlocks { insert, update } => {
                            Msg::Sub(On::UpdateBlocks { insert, update })
                        }
                        modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                        modal_resource::On::SelectImageData(texture) => {
                            let mut tool = tool.as_ref().clone();
                            tool.texture = Some(texture);
                            Msg::SetTool(tool_idx, TableTool::Character(Rc::new(tool)))
                        }
                        modal_resource::On::SelectNone => {
                            let mut tool = tool.as_ref().clone();
                            tool.texture = None;
                            Msg::SetTool(tool_idx, TableTool::Character(Rc::new(tool)))
                        }
                        _ => Msg::NoOp,
                    }
                }),
            ),
        }
    }

    fn render_icon_list(&self) -> Html {
        let selected_idx = self.tools.selected_idx();
        Html::div(
            Attributes::new().class(Self::class("icons")),
            Events::new(),
            self.tools
                .iter()
                .enumerate()
                .map(|(idx, tool)| self.render_icon(tool, idx, selected_idx))
                .collect(),
        )
    }

    fn render_icon(&self, tool: &TableTool, idx: usize, selected_idx: usize) -> Html {
        let (title, child) = match tool {
            TableTool::Selecter(..) => ("選択", fa::fas_i("fa-mouse-pointer")),
            TableTool::Craftboard(..) => ("盤面", fa::fas_i("fa-border-all")),
            TableTool::Pen(..) => ("鉛筆", fa::fas_i("fa-pencil-alt")),
            TableTool::Eraser(..) => ("消しゴム", fa::fas_i("fa-eraser")),
            TableTool::Character(..) => ("キャラコマ", fa::fas_i("fa-user")),
            TableTool::Boxblock(..) => ("ブロック", fa::fas_i("fa-cube")),
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
                    Events::new().on_click(self, move |_| Msg::SetSetectedToolIdx(idx)),
                    vec![child],
                ),
                text::span(title),
            ],
        )
    }

    fn render_tool_option(&self) -> Html {
        let tool_idx = self.tools.selected_idx();
        Html::div(
            Attributes::new()
                .class(Self::class("option"))
                .class("pure-form"),
            Events::new(),
            vec![match self.tools.get(tool_idx) {
                Some(TableTool::Pen(tool)) => self.render_tool_option_pen(tool_idx, tool),
                Some(TableTool::Craftboard(tool)) => {
                    self.render_tool_option_craftboard(tool_idx, tool)
                }
                Some(TableTool::Boxblock(tool)) => self.render_tool_option_boxblock(tool_idx, tool),
                Some(TableTool::Character(tool)) => {
                    self.render_tool_option_character(tool_idx, tool)
                }
                _ => Html::none(),
            }],
        )
    }

    fn render_tool_option_pen(&self, tool_idx: usize, pen: &Rc<table_tool::Pen>) -> Html {
        Html::div(
            Attributes::new().class(Self::class("pen")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Common::keyvalue()),
                Events::new(),
                vec![
                    text::span("線幅"),
                    Slider::new(
                        self,
                        None,
                        slider::Position::Linear {
                            val: pen.width,
                            min: 0.1,
                            max: 10.0,
                            step: 0.1,
                        },
                        Sub::map({
                            let pen = Rc::clone(&pen);
                            move |sub| match sub {
                                slider::On::Input(width) => {
                                    let mut pen = pen.as_ref().clone();
                                    pen.width = width;
                                    Msg::SetTool(tool_idx, TableTool::Pen(Rc::new(pen)))
                                }
                                _ => Msg::NoOp,
                            }
                        }),
                        slider::Props {
                            range_is_editable: false,
                            theme: slider::Theme::Light,
                        },
                    ),
                    text::span("色"),
                    PopupColorPallet::empty(
                        self,
                        None,
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
                                    Msg::SetTool(tool_idx, TableTool::Pen(Rc::new(pen)))
                                }
                            }
                        }),
                    ),
                ],
            )],
        )
    }

    fn render_tool_option_craftboard(
        &self,
        tool_idx: usize,
        craftboard: &Rc<table_tool::Craftboard>,
    ) -> Html {
        Html::div(
            Attributes::new().class(Self::class("craftboard")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Common::keyvalue()),
                Events::new(),
                vec![
                    text::span("X幅"),
                    self.render_tool_option_craftboard_size(tool_idx, craftboard, 0, 1.0),
                    text::span("Y幅"),
                    self.render_tool_option_craftboard_size(tool_idx, craftboard, 1, 1.0),
                    text::span("Z幅"),
                    self.render_tool_option_craftboard_size(tool_idx, craftboard, 2, 0.0),
                ],
            )],
        )
    }

    fn render_tool_option_craftboard_size(
        &self,
        tool_idx: usize,
        craftboard: &Rc<table_tool::Craftboard>,
        coord_idx: usize,
        min: f64,
    ) -> Html {
        Slider::new(
            self,
            None,
            slider::Position::Linear {
                val: craftboard.size[coord_idx],
                min: min,
                max: 100.0,
                step: 1.0,
            },
            Sub::map({
                let craftboard = Rc::clone(&craftboard);
                move |sub| match sub {
                    slider::On::Input(val) => {
                        let mut craftboard = craftboard.as_ref().clone();
                        craftboard.size[coord_idx] = val;
                        Msg::SetTool(tool_idx, TableTool::Craftboard(Rc::new(craftboard)))
                    }
                    _ => Msg::NoOp,
                }
            }),
            slider::Props {
                range_is_editable: false,
                theme: slider::Theme::Light,
            },
        )
    }

    fn render_tool_option_character(
        &self,
        tool_idx: usize,
        character: &Rc<table_tool::Character>,
    ) -> Html {
        Html::div(
            Attributes::new().class(Self::class("character")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Common::keyvalue()),
                Events::new(),
                vec![
                    text::span("サイズ"),
                    Slider::new(
                        self,
                        None,
                        slider::Position::Linear {
                            val: character.size,
                            min: 0.1,
                            max: 10.0,
                            step: 0.1,
                        },
                        Sub::map({
                            let character = Rc::clone(&character);
                            move |sub| match sub {
                                slider::On::Input(val) => {
                                    let mut character = character.as_ref().clone();
                                    character.size = val.max(0.1);
                                    Msg::SetTool(tool_idx, TableTool::Character(Rc::new(character)))
                                }
                                _ => Msg::NoOp,
                            }
                        }),
                        slider::Props {
                            range_is_editable: false,
                            theme: slider::Theme::Light,
                        },
                    ),
                    text::span("立ち絵サイズ"),
                    Slider::new(
                        self,
                        None,
                        slider::Position::Linear {
                            val: character.tex_size,
                            min: 0.1,
                            max: 10.0,
                            step: 0.1,
                        },
                        Sub::map({
                            let character = Rc::clone(&character);
                            move |sub| match sub {
                                slider::On::Input(val) => {
                                    let mut character = character.as_ref().clone();
                                    character.tex_size = val.max(0.0);
                                    Msg::SetTool(tool_idx, TableTool::Character(Rc::new(character)))
                                }
                                _ => Msg::NoOp,
                            }
                        }),
                        slider::Props {
                            range_is_editable: false,
                            theme: slider::Theme::Light,
                        },
                    ),
                    text::span("色"),
                    PopupColorPallet::empty(
                        self,
                        None,
                        popup_color_pallet::Props {
                            default_selected: character.color,
                            direction: popup_color_pallet::Direction::Bottom,
                        },
                        Sub::map({
                            let character = Rc::clone(&character);
                            move |sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    let mut character = character.as_ref().clone();
                                    character.color = color;
                                    Msg::SetTool(tool_idx, TableTool::Character(Rc::new(character)))
                                }
                            }
                        }),
                    ),
                    text::span("立ち絵"),
                    {
                        let events = Events::new().on_click(self, {
                            let character = Rc::clone(&character);
                            move |_| {
                                Msg::SetShowingModal(ShowingModal::SelectCharacterTexture(
                                    tool_idx, character,
                                ))
                            }
                        });
                        if let Some(src) = character
                            .texture
                            .as_ref()
                            .and_then(|texture| texture.map(|texture| texture.url().to_string()))
                        {
                            Html::img(
                                Attributes::new()
                                    .draggable("false")
                                    .class(Common::bg_transparent())
                                    .class(Self::class("block-texture"))
                                    .src(src),
                                events,
                                vec![],
                            )
                        } else {
                            Btn::secondary(
                                Attributes::new(),
                                events,
                                vec![Html::text("テクスチャを選択")],
                            )
                        }
                    },
                ],
            )],
        )
    }

    fn render_tool_option_boxblock(
        &self,
        tool_idx: usize,
        boxblock: &Rc<table_tool::Boxblock>,
    ) -> Html {
        Html::div(
            Attributes::new().class(Self::class("boxblock")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Common::keyvalue()),
                Events::new(),
                vec![
                    text::span("形状"),
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
                            vec![Html::text(match &boxblock.shape {
                                block::boxblock::Shape::Cube => "立方体",
                                block::boxblock::Shape::Slope => "斜面",
                                block::boxblock::Shape::Sphere => "球体",
                                block::boxblock::Shape::Cylinder => "円柱",
                            })],
                            vec![
                                self.render_tool_option_boxblock_shape(
                                    tool_idx,
                                    boxblock,
                                    block::boxblock::Shape::Cube,
                                    "立方体",
                                ),
                                self.render_tool_option_boxblock_shape(
                                    tool_idx,
                                    boxblock,
                                    block::boxblock::Shape::Slope,
                                    "斜面",
                                ),
                                self.render_tool_option_boxblock_shape(
                                    tool_idx,
                                    boxblock,
                                    block::boxblock::Shape::Sphere,
                                    "球体",
                                ),
                                self.render_tool_option_boxblock_shape(
                                    tool_idx,
                                    boxblock,
                                    block::boxblock::Shape::Cylinder,
                                    "円柱",
                                ),
                            ],
                        ),
                    ),
                    text::span("X幅"),
                    self.render_tool_option_boxblock_size(tool_idx, boxblock, 0),
                    text::span("Y幅"),
                    self.render_tool_option_boxblock_size(tool_idx, boxblock, 1),
                    text::span("Z幅"),
                    self.render_tool_option_boxblock_size(tool_idx, boxblock, 2),
                    text::span("色"),
                    PopupColorPallet::empty(
                        self,
                        None,
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
                                    Msg::SetTool(tool_idx, TableTool::Boxblock(Rc::new(boxblock)))
                                }
                            }
                        }),
                    ),
                    text::span("テクスチャ"),
                    {
                        let events = Events::new().on_click(self, {
                            let boxblock = Rc::clone(&boxblock);
                            move |_| {
                                Msg::SetShowingModal(ShowingModal::SelectBlockTexture(
                                    tool_idx, boxblock,
                                ))
                            }
                        });
                        if let Some(src) = boxblock.texture.as_ref().and_then(|texture| {
                            texture.map(|texture| texture.data().url().to_string())
                        }) {
                            Html::img(
                                Attributes::new()
                                    .draggable("false")
                                    .class(Common::bg_transparent())
                                    .class(Self::class("block-texture"))
                                    .src(src),
                                events,
                                vec![],
                            )
                        } else {
                            Btn::secondary(
                                Attributes::new(),
                                events,
                                vec![Html::text("テクスチャを選択")],
                            )
                        }
                    },
                ],
            )],
        )
    }

    fn render_tool_option_boxblock_shape(
        &self,
        tool_idx: usize,
        boxblock: &Rc<table_tool::Boxblock>,
        shape: block::boxblock::Shape,
        text: impl Into<String>,
    ) -> Html {
        Btn::menu(
            Attributes::new(),
            Events::new().on_click(self, {
                let boxblock = Rc::clone(&boxblock);
                move |_| {
                    let mut boxblock = boxblock.as_ref().clone();
                    boxblock.shape = shape;
                    Msg::SetTool(tool_idx, TableTool::Boxblock(Rc::new(boxblock)))
                }
            }),
            vec![Html::text(text)],
        )
    }

    fn render_tool_option_boxblock_size(
        &self,
        tool_idx: usize,
        boxblock: &Rc<table_tool::Boxblock>,
        coord_idx: usize,
    ) -> Html {
        Slider::new(
            self,
            None,
            slider::Position::Linear {
                val: boxblock.size[coord_idx],
                min: 0.1,
                max: 10.0,
                step: 0.1,
            },
            Sub::map({
                let boxblock = Rc::clone(&boxblock);
                move |sub| match sub {
                    slider::On::Input(val) => {
                        let mut boxblock = boxblock.as_ref().clone();
                        boxblock.size[coord_idx] = val;
                        Msg::SetTool(tool_idx, TableTool::Boxblock(Rc::new(boxblock)))
                    }
                    _ => Msg::NoOp,
                }
            }),
            slider::Props {
                range_is_editable: false,
                theme: slider::Theme::Light,
            },
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

            ".block-texture" {
                "width": "7.5rem";
                "height": "7.5rem";
                "object-fit": "contain";
            }
        }
    }
}
