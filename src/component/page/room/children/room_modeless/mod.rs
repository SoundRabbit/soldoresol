use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::tab_btn::{self, TabBtn};
use super::molecule::modeless::{self, Modeless};
use super::util::Prop;
use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self, ResourceId};
use crate::libs::color::color_system;
use crate::libs::select_list::SelectList;
use crate::libs::type_id::type_id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use regex::Regex;
use wasm_bindgen::JsCast;

pub mod boxblock;
pub mod character;
pub mod chat_channel;

use boxblock::Boxblock;
use character::Character;
use chat_channel::ChatChannel;

pub enum Content {
    ChatChannel(BlockId),
    Character(BlockId),
    Boxblock(BlockId),
}

pub struct Props {
    pub content: Prop<SelectList<Content>>,
    pub z_index: usize,
    pub container_element: Prop<web_sys::Element>,
    pub page_x: i32,
    pub page_y: i32,
    pub minimized: bool,
    pub block_arena: block::ArenaRef,
    pub resource_arena: resource::ArenaRef,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetLocation { page_x: i32, page_y: i32 },
    SetSize { size: [f32; 2] },
}

pub enum On {
    Close,
    Minimize,
    Restore,
    Focus,
    DragTabStart {
        tab_idx: usize,
    },
    DropTab {
        tab_idx: Option<usize>,
    },
    SelectTab {
        tab_idx: usize,
    },
    SetCharacterCommonProps {
        character_id: BlockId,
        name: Option<String>,
        display_name: Option<String>,
        description: Option<String>,
        name_color: Option<crate::libs::color::Pallet>,
    },
    SetCharacterTextureId {
        character_id: BlockId,
        tex_idx: usize,
        resource_id: Option<ResourceId>,
    },
    AddCharacterTexture {
        character_id: BlockId,
    },
    RemoveCharacterTexture {
        character_id: BlockId,
        tex_idx: usize,
    },
    SetCharacterTextureIdx {
        character_id: BlockId,
        tex_idx: usize,
    },
    SetCharacterTextureName {
        character_id: BlockId,
        tex_idx: usize,
        tex_name: String,
    },
    SetBoxblockCommonProps {
        boxblock_id: BlockId,
        name: Option<String>,
        display_name: Option<String>,
        color: Option<crate::libs::color::Pallet>,
        size: Option<[f32; 3]>,
    },
    SetPropertyName {
        property_id: BlockId,
        name: String,
    },
    AddPropertyChild {
        block_id: BlockId,
        name: String,
    },
    RemoveProperty {
        property_id: BlockId,
        idx: usize,
    },
    AddPropertyValue {
        property_id: BlockId,
    },
    SetPropertyValue {
        property_id: BlockId,
        idx: usize,
        value: block::property::Value,
    },
    RemovePropertyValue {
        property_id: BlockId,
        idx: usize,
    },
    SetPropertyValueMode {
        property_id: BlockId,
        value_mode: block::property::ValueMode,
    },
}

pub struct RoomModeless {
    content: Prop<SelectList<Content>>,
    z_index: usize,
    container_element: Prop<web_sys::Element>,
    page_x: i32,
    page_y: i32,
    size: [f32; 2],
    minimized: bool,
    block_arena: block::ArenaRef,
    resource_arena: resource::ArenaRef,
}

impl Constructor for RoomModeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            content: props.content,
            z_index: props.z_index,
            container_element: props.container_element,
            page_x: props.page_x,
            page_y: props.page_y,
            size: [0.3, 0.3],
            minimized: props.minimized,
            block_arena: props.block_arena,
            resource_arena: props.resource_arena,
        }
    }
}

impl Component for RoomModeless {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.content = props.content;
        self.z_index = props.z_index;
        self.container_element = props.container_element;
        self.minimized = props.minimized;
        self.block_arena = props.block_arena;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => {
                crate::debug::log_1("Msg::Sub");
                Cmd::sub(sub)
            }
            Msg::SetLocation { page_x, page_y } => {
                self.page_x = page_x;
                self.page_y = page_y;
                Cmd::none()
            }
            Msg::SetSize { size } => {
                self.size = size;
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(if self.minimized {
            self.render_minimized()
        } else {
            self.render_stored()
        })
    }
}

impl RoomModeless {
    pub fn tab_id() -> String {
        type_id::<Self>()
    }

    fn render_minimized(&self) -> Html {
        self.render_header()
    }

    fn render_stored(&self) -> Html {
        Modeless::with_child(
            modeless::Props {
                z_index: self.z_index,
                container_element: Some(Prop::clone(&self.container_element)),
                page_x: self.page_x,
                page_y: self.page_y,
                size: self.size.clone(),
                ..Default::default()
            },
            Subscription::new(|sub| match sub {
                modeless::On::Focus => Msg::Sub(On::Focus),
                modeless::On::Move(page_x, page_y) => Msg::SetLocation { page_x, page_y },
                modeless::On::Resize(size) => Msg::SetSize { size },
            }),
            Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![self.render_header(), self.render_content()],
            ),
        )
    }

    fn render_controller_sotored() -> Vec<Html> {
        vec![
            Btn::secondary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::Sub(On::Minimize)),
                vec![fa::i("fa-window-minimize")],
            ),
            Btn::secondary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::Sub(On::Close)),
                vec![fa::i("fa-times")],
            ),
        ]
    }

    fn render_controller_minimized() -> Vec<Html> {
        vec![Btn::secondary(
            Attributes::new(),
            Events::new().on_click(|_| Msg::Sub(On::Restore)),
            vec![fa::i("fa-window-restore")],
        )]
    }

    fn render_header(&self) -> Html {
        let order = if self.minimized { self.z_index } else { 0 };

        Html::div(
            Attributes::new()
                .class(Self::class("header"))
                .style("order", order.to_string()),
            Events::new()
                .on("dragover", |e| {
                    e.prevent_default();
                    Msg::NoOp
                })
                .on("drop", |e| {
                    let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                    let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                    if data == Self::tab_id() {
                        e.prevent_default();
                        e.stop_propagation();
                        Msg::Sub(On::DropTab { tab_idx: None })
                    } else {
                        Msg::NoOp
                    }
                }),
            vec![
                vec![Html::div(
                    Attributes::new().class(Self::class("header-tab-container")),
                    Events::new(),
                    self.render_header_tabs(),
                )],
                if self.minimized {
                    Self::render_controller_minimized()
                } else {
                    Self::render_controller_sotored()
                },
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    fn render_header_tabs(&self) -> Vec<Html> {
        let mut tabs = self.content
            .iter()
            .enumerate()
            .map(|(tab_idx, a_content)| {
                let is_selected = tab_idx == self.content.selected_idx();
                let tab_heading = match a_content {
                    Content::ChatChannel(channel_id) => vec![
                        fa::i("fa-comment"), 
                        Html::text(self.block_arena.map(channel_id, |channel: &block::chat::channel::Channel| {
                            format!(" {}", channel.name())
                        }).unwrap_or(String::new()))
                    ],
                    Content::Character(character_id) => vec![
                        fa::i("fa-user"),
                        Html::text(self.block_arena.map(character_id, |character: &block::character::Character| {
                            format!(" {}", character.name())
                        }).unwrap_or(String::new()))
                    ],
                    Content::Boxblock(boxblock_id) => vec![
                        fa::i("fa-cube"),
                        Html::text(self.block_arena.map(boxblock_id, |boxblock: &block::boxblock::Boxblock| {
                            format!(" {}", boxblock.name())
                        }).unwrap_or(String::new()))
                    ]
                };
                TabBtn::new(
                    true,
                    is_selected,
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_|Msg::Sub(On::SelectTab { tab_idx }))
                        .on("dragstart", move |e| {
                            let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                            e.stop_propagation();
                            let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                            unwrap_or!(data_transfer.set_data("text/plain", &Self::tab_id()).ok(); Msg::NoOp);
                            Msg::Sub(On::DragTabStart { tab_idx })
                    })
                    .on("drop", move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                        let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                        let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                        if data == Self::tab_id() {
                            e.prevent_default();
                            e.stop_propagation();
                            Msg::Sub(On::DropTab { tab_idx: Some(tab_idx) })
                        } else {
                            Msg::NoOp
                        }
                    }),
                    tab_heading
                )
            })
            .collect::<Vec<_>>();

        tabs.push(Dropdown::with_children(
            dropdown::Props {
                text: String::from("追加"),
                variant: btn::Variant::TransparentDark,
                ..Default::default()
            },
            Subscription::none(),
            vec![Btn::menu(
                Attributes::new(),
                Events::new(),
                vec![fa::i("fa-chat"), Html::text(" 新規チャンネル")],
            )],
        ));

        tabs
    }

    fn render_content(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("body")),
            Events::new().on_mousedown(|e| {
                e.stop_propagation();
                Msg::NoOp
            }),
            vec![match self.content.selected() {
                Some(Content::ChatChannel(channel_id)) => ChatChannel::empty(
                    chat_channel::Props {
                        block_arena: block::ArenaRef::clone(&self.block_arena),
                        channel_id: BlockId::clone(channel_id),
                    },
                    Subscription::none(),
                ),
                Some(Content::Character(character_id)) => Character::empty(
                    character::Props {
                        block_arena: block::ArenaRef::clone(&self.block_arena),
                        resource_arena: resource::ArenaRef::clone(&self.resource_arena),
                        character_id: BlockId::clone(character_id),
                    },
                    Subscription::new({
                        let character_id = BlockId::clone(character_id);
                        move |sub| match sub {
                            character::On::SetCommonProps {
                                name,
                                display_name,
                                description,
                                name_color,
                            } => Msg::Sub(On::SetCharacterCommonProps {
                                character_id,
                                name,
                                display_name,
                                description,
                                name_color,
                            }),
                            character::On::SetTextureId {
                                tex_idx,
                                resource_id,
                            } => Msg::Sub(On::SetCharacterTextureId {
                                character_id,
                                tex_idx,
                                resource_id,
                            }),
                            character::On::AddTexture => {
                                Msg::Sub(On::AddCharacterTexture { character_id })
                            }
                            character::On::RemoveTexture { tex_idx } => {
                                Msg::Sub(On::RemoveCharacterTexture {
                                    character_id,
                                    tex_idx,
                                })
                            }
                            character::On::SetTextureIdx { tex_idx } => {
                                Msg::Sub(On::SetCharacterTextureIdx {
                                    character_id,
                                    tex_idx,
                                })
                            }
                            character::On::SetTextureName { tex_idx, tex_name } => {
                                Msg::Sub(On::SetCharacterTextureName {
                                    character_id,
                                    tex_idx,
                                    tex_name,
                                })
                            }
                            character::On::SetPropertyName { property_id, name } => {
                                Msg::Sub(On::SetPropertyName { property_id, name })
                            }
                            character::On::AddPropertyChild { property_id, name } => {
                                Msg::Sub(On::AddPropertyChild {
                                    block_id: property_id.unwrap_or(character_id),
                                    name,
                                })
                            }
                            character::On::AddPropertyValue { property_id } => {
                                Msg::Sub(On::AddPropertyValue { property_id })
                            }
                            character::On::SetPropertyValue {
                                property_id,
                                idx,
                                value,
                            } => Msg::Sub(On::SetPropertyValue {
                                property_id,
                                idx,
                                value,
                            }),
                            character::On::RemovePropertyValue { property_id, idx } => {
                                Msg::Sub(On::RemovePropertyValue { property_id, idx })
                            }
                            character::On::SetPropertyValueMode {
                                property_id,
                                value_mode,
                            } => Msg::Sub(On::SetPropertyValueMode {
                                property_id,
                                value_mode,
                            }),
                            character::On::RemoveProperty { property_id, idx } => {
                                Msg::Sub(On::RemoveProperty { property_id, idx })
                            }
                        }
                    }),
                ),
                Some(Content::Boxblock(boxblock_id)) => Boxblock::empty(
                    boxblock::Props {
                        block_arena: block::ArenaRef::clone(&self.block_arena),
                        resource_arena: resource::ArenaRef::clone(&self.resource_arena),
                        boxblock_id: BlockId::clone(boxblock_id),
                    },
                    Subscription::new({
                        let boxblock_id = BlockId::clone(boxblock_id);
                        move |sub| match sub {
                            boxblock::On::SetCommonProps {
                                name,
                                display_name,
                                size,
                                color,
                            } => Msg::Sub(On::SetBoxblockCommonProps {
                                boxblock_id,
                                name,
                                display_name,
                                size,
                                color,
                            }),
                        }
                    }),
                ),
                _ => Html::none(),
            }],
        )
    }
}

impl Styled for RoomModeless {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "align-items": "stretch";
                "width": "100%";
                "height": "100%";
            }

            ".header" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-colmuns": "max-content";
                "grid-auto-flow": "column";
                "background-color": format!("{}", color_system::gray(255, 8));
            }

            ".header-tab-container" {
                "display": "flex";
                "flex-wrap": "wrap";
            }

            ".body" {
                "background-color": format!("{}", color_system::gray(255, 0));
                "overflow": "hidden";
            }
        }
    }
}
