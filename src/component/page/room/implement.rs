use super::{
    super::atom::btn::{self, Btn},
    super::atom::dropdown::{self, Dropdown},
    super::atom::fa,
    super::atom::header::{self, Header},
    super::template::basic_app::{self, BasicApp},
    super::util::styled::{Style, Styled},
    super::util::{Prop, State},
    children::modal_imported_files::{self, ModalImportedFiles},
    children::modal_new_channel::{self, ModalNewChannel},
    children::room_modeless::{self, RoomModeless},
    children::side_menu::{self, SideMenu},
    model::table::TableTool,
    renderer::{CameraMatrix, Renderer},
};
use crate::arena::block::{self, BlockId};
use crate::arena::player::{self, Player};
use crate::arena::resource::{self, ResourceId};
use crate::arena::Insert;
use crate::libs::modeless_list::ModelessList;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub room: Rc<MeshRoom>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
}

pub enum Awaiting<T> {
    Pending(T),
    Ready(T),
}

pub enum Msg {
    NoOp,
    SetCanvasElement {
        canvas: web_sys::HtmlCanvasElement,
    },
    ResetCanvasSize,
    RenderCanvas,
    SetTableToolIdx {
        idx: usize,
    },
    OpenNewModal {
        modal: Modal,
    },
    OpenNewModeless {
        content: room_modeless::Content,
    },
    OpenNewChatModeless,
    FocusModeless {
        modeless_id: U128Id,
    },
    CloseModeless {
        modeless_id: U128Id,
    },
    MinimizeModeless {
        modeless_id: U128Id,
    },
    RestoreModeless {
        modeless_id: U128Id,
    },
    SetModelessContainerElement {
        element: web_sys::Element,
    },
    SetDraggingModelessTab {
        modeless_id: U128Id,
        tab_idx: usize,
    },
    MoveModelessTab {
        modeless_id: U128Id,
        tab_idx: Option<usize>,
    },
    DropModelessTab {
        page_x: i32,
        page_y: i32,
    },
    SelectModelessTab {
        modeless_id: U128Id,
        tab_idx: usize,
    },
    SetOverlay {
        overlay: Overlay,
    },
    LoadFile {
        files: Vec<web_sys::File>,
        overlay: Option<Overlay>,
    },
    LoadResourceData {
        data: Vec<resource::Data>,
    },
}

pub enum On {}

pub struct Implement {
    peer: Rc<Peer>,
    peer_id: Rc<String>,
    room: Rc<MeshRoom>,
    room_id: Rc<String>,
    client_id: Rc<String>,

    element_id: ElementId,

    table_tools: State<SelectList<TableTool>>,
    modeless_list: ModelessList<ModelessContent>,
    modeless_container_element: Option<State<web_sys::Element>>,
    dragging_modeless_tab: Option<(U128Id, usize)>,

    block_arena: block::Arena,
    player_arena: player::Arena,
    resource_arena: resource::Arena,

    renderer: Option<Renderer>,
    camera_matrix: CameraMatrix,

    chat_id: BlockId,
    world_id: BlockId,

    modal: Modal,
    overlay: Overlay,
}

struct ModelessContent {
    content: State<SelectList<room_modeless::Content>>,
    page_x: i32,
    page_y: i32,
    minimized: bool,
}

struct ElementId {
    header_room_id: String,
}

enum Modal {
    None,
    NewChannel,
    ImportedFiles,
}

enum Overlay {
    None,
    DragFile,
}

impl Constructor for Implement {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        let mut block_arena = block::Arena::new();

        let chat = block::chat::Chat::new(vec![
            block_arena.insert(block::chat::channel::Channel::new(String::from("メイン"))),
            block_arena.insert(block::chat::channel::Channel::new(String::from("サブ"))),
        ]);

        let chat_id = block_arena.insert(chat);

        let drawing_texture_id = block_arena.insert(block::table::texture::Texture::new(
            &[1024, 1024],
            [20.0, 20.0],
        ));
        let table_id = block_arena.insert(block::table::Table::new(
            drawing_texture_id,
            [20.0, 20.0],
            "最初のテーブル",
        ));
        let world_id = block_arena.insert(block::world::World::new(table_id));

        let mut player_arena = player::Arena::new();
        player_arena.insert(Rc::clone(&props.client_id), Player::new());

        Self {
            peer: props.peer,
            peer_id: props.peer_id,
            room: props.room,
            room_id: props.room_id,
            client_id: props.client_id,

            element_id: ElementId {
                header_room_id: format!("{:X}", crate::libs::random_id::u128val()),
            },

            table_tools: State::new(SelectList::new(
                vec![
                    TableTool::Selector,
                    TableTool::Hr(String::from("描画")),
                    TableTool::Pen,
                    TableTool::Shape,
                    TableTool::Eraser,
                ],
                0,
            )),
            modeless_list: ModelessList::new(),
            modeless_container_element: None,
            dragging_modeless_tab: None,

            block_arena,
            player_arena,
            resource_arena: resource::Arena::new(),

            renderer: None,
            camera_matrix: CameraMatrix::new(),

            chat_id,
            world_id,

            modal: Modal::None,
            overlay: Overlay::None,
        }
    }
}

impl Component for Implement {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),

            Msg::SetCanvasElement { canvas } => {
                self.renderer = Some(Renderer::new(Rc::new(canvas)));
                Cmd::task(move |resolve| {
                    let a = Closure::once(
                        Box::new(move || resolve(Msg::ResetCanvasSize)) as Box<dyn FnOnce()>
                    );
                    let _ = web_sys::window()
                        .unwrap()
                        .request_animation_frame(a.as_ref().unchecked_ref());
                    a.forget();
                })
            }

            Msg::ResetCanvasSize => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.reset_size();
                    self.gl_render_async()
                } else {
                    Cmd::none()
                }
            }

            Msg::RenderCanvas => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render(
                        &self.block_arena,
                        &self.resource_arena,
                        &self.world_id,
                        &self.camera_matrix,
                    );
                }
                Cmd::none()
            }

            Msg::SetTableToolIdx { idx } => {
                self.table_tools.set_selected_idx(idx);
                Cmd::none()
            }

            Msg::OpenNewModal { modal } => {
                self.modal = modal;
                Cmd::none()
            }

            Msg::OpenNewModeless { content } => {
                self.modeless_list.push(ModelessContent {
                    content: State::new(SelectList::new(vec![content], 0)),
                    page_x: 0,
                    page_y: 0,
                    minimized: false,
                });
                Cmd::none()
            }

            Msg::OpenNewChatModeless => {
                let tabs = self
                    .block_arena
                    .map(&self.chat_id, |chat: &block::chat::Chat| {
                        chat.channels()
                            .iter()
                            .map(|channel_id| {
                                room_modeless::Content::ChatChannel(BlockId::clone(&channel_id))
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or(vec![]);
                self.modeless_list.push(ModelessContent {
                    content: State::new(SelectList::new(tabs, 0)),
                    page_x: 0,
                    page_y: 0,
                    minimized: false,
                });
                Cmd::none()
            }

            Msg::CloseModeless { modeless_id } => {
                self.modeless_list.remove(&modeless_id);
                Cmd::none()
            }

            Msg::MinimizeModeless { modeless_id } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.minimized = true;
                }
                Cmd::none()
            }

            Msg::RestoreModeless { modeless_id } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.minimized = false;
                }
                Cmd::none()
            }

            Msg::FocusModeless { modeless_id } => {
                self.modeless_list.focus(&modeless_id);
                Cmd::none()
            }

            Msg::SetModelessContainerElement { element } => {
                self.modeless_container_element = Some(State::new(element));
                Cmd::none()
            }

            Msg::SetDraggingModelessTab {
                modeless_id,
                tab_idx,
            } => {
                crate::debug::log_1("SetDraggingModelessTab");
                self.dragging_modeless_tab = Some((modeless_id, tab_idx));
                Cmd::none()
            }

            Msg::MoveModelessTab {
                modeless_id: to_id,
                tab_idx,
            } => {
                if let Some((from_id, from_idx)) = self.dragging_modeless_tab.take() {
                    let tab = if let Some(tab) = self
                        .modeless_list
                        .get_mut(&from_id)
                        .and_then(|c| c.content.remove(from_idx))
                    {
                        Some(tab)
                    } else {
                        None
                    };
                    if let Some((tab, to_content)) =
                        join_some!(tab, self.modeless_list.get_mut(&to_id))
                    {
                        if let Some(tab_idx) = tab_idx {
                            to_content.content.insert(tab_idx, tab);
                        } else {
                            to_content.content.push(tab);
                        }
                    }
                    if let Some(from_content) = self.modeless_list.get(&from_id) {
                        if from_content.content.len() < 1 {
                            self.modeless_list.remove(&from_id);
                        }
                    }
                }
                Cmd::none()
            }

            Msg::DropModelessTab { page_x, page_y } => {
                if let Some((from_id, from_idx)) = self.dragging_modeless_tab.take() {
                    let tab = if let Some(tab) = self
                        .modeless_list
                        .get_mut(&from_id)
                        .and_then(|c| c.content.remove(from_idx))
                    {
                        Some(tab)
                    } else {
                        None
                    };
                    if let Some(tab) = tab {
                        self.modeless_list.push(ModelessContent {
                            content: State::new(SelectList::new(vec![tab], 0)),
                            page_x,
                            page_y,
                            minimized: false,
                        });
                    }
                    if let Some(from_content) = self.modeless_list.get(&from_id) {
                        if from_content.content.len() < 1 {
                            self.modeless_list.remove(&from_id);
                        }
                    }
                }
                Cmd::none()
            }

            Msg::SelectModelessTab {
                modeless_id,
                tab_idx,
            } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.content.set_selected_idx(tab_idx);
                }
                Cmd::none()
            }

            Msg::SetOverlay { overlay } => {
                self.overlay = overlay;
                Cmd::none()
            }

            Msg::LoadFile { files, overlay } => {
                if let Some(overlay) = overlay {
                    self.overlay = overlay;
                }
                Cmd::task(move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        let mut tasks = vec![];
                        for file in files {
                            tasks.push(resource::Data::from_blob(file.into()));
                        }
                        let data = futures::future::join_all(tasks)
                            .await
                            .into_iter()
                            .filter_map(|x| x)
                            .collect::<Vec<_>>();
                        resolve(Msg::LoadResourceData { data })
                    })
                })
            }

            Msg::LoadResourceData { data } => {
                for a_data in data {
                    self.resource_arena.add(a_data);
                }
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(BasicApp::with_children(
            basic_app::Props {},
            Subscription::new({
                let tab_is_dragging = self.dragging_modeless_tab.is_some();
                move |sub| match sub {
                    basic_app::On::DragLeave(_) => Msg::NoOp,
                    basic_app::On::DragOver(e) => {
                        if !tab_is_dragging {
                            e.prevent_default();
                            Msg::SetOverlay {
                                overlay: Overlay::DragFile,
                            }
                        } else {
                            Msg::NoOp
                        }
                    }
                    basic_app::On::Drop(e) => {
                        if !tab_is_dragging {
                            let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                            let file_list = unwrap_or!(data_transfer.files(); Msg::NoOp);

                            e.prevent_default();

                            let mut files = vec![];
                            let file_num = file_list.length();

                            for i in 0..file_num {
                                if let Some(file) = file_list.item(i) {
                                    files.push(file);
                                }
                            }

                            Msg::LoadFile {
                                files,
                                overlay: Some(Overlay::None),
                            }
                        } else {
                            Msg::NoOp
                        }
                    }
                }
            }),
            vec![
                Header::with_children(
                    header::Props::new(),
                    Subscription::none(),
                    vec![
                        self.render_header_row_0(),
                        self.render_header_controller_menu(),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Self::class("body")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("side-menu")),
                            Events::new(),
                            vec![SideMenu::empty(
                                side_menu::Props {
                                    tools: self.table_tools.as_prop(),
                                },
                                Subscription::new(|sub| match sub {
                                    side_menu::On::ChangeSelectedIdx { idx } => {
                                        Msg::SetTableToolIdx { idx }
                                    }
                                }),
                            )],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("main")),
                            Events::new(),
                            vec![self.render_canvas(), self.render_modeless_container()],
                        ),
                    ],
                ),
                self.render_modal(),
                self.render_overlay(),
            ],
        ))
    }
}

impl Implement {
    fn gl_render_async(&mut self) -> Cmd<Msg, On> {
        Cmd::task(move |resolve| {
            let a =
                Closure::once(Box::new(move || resolve(Msg::RenderCanvas)) as Box<dyn FnOnce()>);
            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(a.as_ref().unchecked_ref());
            a.forget();
        })
    }

    fn render_modal(&self) -> Html {
        match &self.modal {
            Modal::None => Html::none(),
            Modal::NewChannel => ModalNewChannel::empty(
                modal_new_channel::Props {
                    client_id: Rc::clone(&self.client_id),
                },
                Subscription::new(|sub| match sub {
                    modal_new_channel::On::Close => Msg::OpenNewModal { modal: Modal::None },
                }),
            ),
            Modal::ImportedFiles => ModalImportedFiles::empty(
                modal_imported_files::Props {
                    resource_arena: self.resource_arena.as_ref(),
                },
                Subscription::none(),
            ),
        }
    }

    fn render_overlay(&self) -> Html {
        match &self.overlay {
            Overlay::None => Html::none(),
            Overlay::DragFile => Html::div(
                Attributes::new()
                    .class(Self::class("overlay"))
                    .class(Self::class("overlay-file-import")),
                Events::new().on_dragleave(|_| Msg::SetOverlay {
                    overlay: Overlay::None,
                }),
                vec![Html::div(
                    Attributes::new().class(Self::class("overlay-file-import-text")),
                    Events::new(),
                    vec![
                        fa::i("fa-file-import"),
                        Html::text("ファイルをドロップして追加"),
                    ],
                )],
            ),
        }
    }

    fn render_header_row_0(&self) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![self.render_header_row_0_left()],
        )
    }

    fn render_header_row_0_left(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("header-room-id")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(Self::class("label"))
                        .string("for", &self.element_id.header_room_id),
                    Events::new(),
                    vec![Html::text("ルームID")],
                ),
                Html::input(
                    Attributes::new()
                        .flag("readonly")
                        .class("pure-input")
                        .id(&self.element_id.header_room_id)
                        .value(self.room_id.as_ref()),
                    Events::new(),
                    vec![],
                ),
            ],
        )
    }

    fn render_header_controller_menu(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("header-controller-menu")),
            Events::new(),
            vec![
                self.render_header_controller_menu_chat(),
                self.render_header_controller_menu_file(),
            ],
        )
    }

    fn render_header_controller_menu_chat(&self) -> Html {
        let channel_names = self
            .block_arena
            .map(&self.chat_id, |chat: &block::chat::Chat| {
                let mut channels = vec![];

                for channel_id in chat.channels() {
                    let channel_name = self
                        .block_arena
                        .map(channel_id, |channel: &block::chat::channel::Channel| {
                            channel.name().clone()
                        })
                        .unwrap_or(String::new());
                    channels.push((BlockId::clone(&channel_id), channel_name));
                }

                channels
            })
            .unwrap_or(vec![]);

        Dropdown::with_children(
            dropdown::Props {
                text: String::from("チャット"),
                direction: dropdown::Direction::BottomRight,
                variant: btn::Variant::Dark,
                ..Default::default()
            },
            Subscription::none(),
            vec![
                Dropdown::with_children(
                    dropdown::Props {
                        text: String::from("チャンネル"),
                        direction: dropdown::Direction::RightBottom,
                        toggle_type: dropdown::ToggleType::Hover,
                        variant: btn::Variant::Menu,
                        ..Default::default()
                    },
                    Subscription::none(),
                    vec![
                        vec![Btn::with_children(
                            btn::Props {
                                variant: btn::Variant::Menu,
                            },
                            Subscription::new(|sub| match sub {
                                btn::On::Click => Msg::OpenNewChatModeless,
                            }),
                            vec![fa::i("fa-comments"), Html::text(" 全てのチャンネル")],
                        )],
                        channel_names
                            .into_iter()
                            .map(|(channel_id, channel_name)| {
                                Btn::with_children(
                                    btn::Props {
                                        variant: btn::Variant::Menu,
                                    },
                                    Subscription::new(|sub| match sub {
                                        btn::On::Click => Msg::OpenNewModeless {
                                            content: room_modeless::Content::ChatChannel(
                                                channel_id,
                                            ),
                                        },
                                    }),
                                    vec![
                                        fa::i("fa-comment"),
                                        Html::text(format!(" {}", channel_name)),
                                    ],
                                )
                            })
                            .collect(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                ),
                Btn::with_children(
                    btn::Props {
                        variant: btn::Variant::Menu,
                    },
                    Subscription::new(|sub| match sub {
                        btn::On::Click => Msg::OpenNewModal {
                            modal: Modal::NewChannel,
                        },
                    }),
                    vec![fa::i("fa-plus"), Html::text(" 新規チャンネル")],
                ),
                Btn::with_children(
                    btn::Props {
                        variant: btn::Variant::Menu,
                    },
                    Subscription::new(|sub| match sub {
                        btn::On::Click => Msg::OpenNewChatModeless,
                    }),
                    vec![fa::i("fa-cog"), Html::text(" チャンネル設定")],
                ),
            ],
        )
    }

    fn render_header_controller_menu_file(&self) -> Html {
        Dropdown::with_children(
            dropdown::Props {
                text: String::from("ファイル"),
                direction: dropdown::Direction::BottomRight,
                variant: btn::Variant::Dark,
                ..Default::default()
            },
            Subscription::none(),
            vec![Btn::with_children(
                btn::Props {
                    variant: btn::Variant::Menu,
                },
                Subscription::new(|sub| match sub {
                    btn::On::Click => Msg::OpenNewModal {
                        modal: Modal::ImportedFiles,
                    },
                }),
                vec![fa::i("fa-file"), Html::text(" 全てのファイル")],
            )],
        )
    }

    fn render_canvas(&self) -> Html {
        Html::canvas(
            Attributes::new().class(Self::class("canvas")),
            Events::new().rendered(if self.renderer.is_none() {
                Some(|el: web_sys::Element| {
                    let canvas =
                        unwrap_or!(el.dyn_into::<web_sys::HtmlCanvasElement>().ok(); Msg::NoOp);
                    Msg::SetCanvasElement { canvas: canvas }
                })
            } else {
                None
            }),
            vec![],
        )
    }

    fn render_modeless_container(&self) -> Html {
        if let Some(modeless_container_element) = self.modeless_container_element.as_ref() {
            Html::div(
                Attributes::new().class(Self::class("modeless-container")),
                Events::new()
                    .on("dragover", |e| {
                        e.prevent_default();
                        Msg::NoOp
                    })
                    .on("drop", move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                        let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                        let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                        if data == RoomModeless::tag_id() {
                            e.prevent_default();
                            e.stop_propagation();
                            let page_x = e.page_x();
                            let page_y = e.page_y();
                            Msg::DropModelessTab { page_x, page_y }
                        } else {
                            Msg::NoOp
                        }
                    }),
                self.modeless_list
                    .iter()
                    .map(|m| {
                        if let Some((modeless_id, z_index, modeless)) = m.as_ref() {
                            RoomModeless::empty(
                                room_modeless::Props {
                                    z_index: *z_index,
                                    content: modeless.content.as_prop(),
                                    container_element: modeless_container_element.as_prop(),
                                    page_x: modeless.page_x,
                                    page_y: modeless.page_y,
                                    minimized: modeless.minimized,
                                    block_arena: self.block_arena.as_ref(),
                                },
                                Subscription::new({
                                    let modeless_id = U128Id::clone(&modeless_id);
                                    |sub| match sub {
                                        room_modeless::On::Focus => {
                                            Msg::FocusModeless { modeless_id }
                                        }
                                        room_modeless::On::Close => {
                                            Msg::CloseModeless { modeless_id }
                                        }
                                        room_modeless::On::Minimize => {
                                            Msg::MinimizeModeless { modeless_id }
                                        }
                                        room_modeless::On::Restore => {
                                            Msg::RestoreModeless { modeless_id }
                                        }
                                        room_modeless::On::DragTabStart { tab_idx } => {
                                            Msg::SetDraggingModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                        room_modeless::On::DropTab { tab_idx } => {
                                            Msg::MoveModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                        room_modeless::On::SelectTab { tab_idx } => {
                                            Msg::SelectModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                    }
                                }),
                            )
                        } else {
                            Html::div(Attributes::new(), Events::new(), vec![])
                        }
                    })
                    .collect(),
            )
        } else {
            Html::div(
                Attributes::new().class(Self::class("modeless-container")),
                Events::new()
                    .rendered(Some(|element| Msg::SetModelessContainerElement { element })),
                vec![],
            )
        }
    }
}

impl Styled for Implement {
    fn style() -> Style {
        style! {
            "overlay" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "height": "100vh";
                "width": "100vw";
                "z-index": format!("{}", super::super::constant::z_index::overlay);
            }

            "overlay-file-import" {
                "background-color": format!("{}", crate::libs::color::color_system::gray(25, 9));
            }

            "overlay-file-import-text" {
                "position": "absolute";
                "color": format!("{}", crate::libs::color::color_system::gray(100, 0));
                "font-size": "4rem";
                "bottom": "1em";
                "right": "1em";
            }

            "header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            "header-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "column-gap": "0.65em";
            }

            "header-controller-menu" {
                "display": "grid";
                "grid-auto-columns": "max-content";
                "grid-auto-flow": "column";
                "column-gap": "0.65em";
            }

            "body" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
            }

            "side-menu" {
                "z-index": "1";
                "min-height": "max-content";
                "min-width": "max-content";
            }

            "main" {
                "position": "relative";
            }

            "canvas" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
            }

            "modeless-container" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": "0";
                "overflow": "hidden";
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "max-content";
                "justify-content": "start";
                "align-content": "end";
            }
        }
    }
}
