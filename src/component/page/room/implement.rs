use super::{
    super::atom::btn::{self, Btn},
    super::atom::fa,
    super::atom::header::{self, Header},
    super::template::basic_app::{self, BasicApp},
    super::util::styled::{Style, Styled},
    super::util::{Prop, State},
    children::room_modeless::{self, RoomModeless},
    children::side_menu::{self, SideMenu},
    model::table::TableTool,
};
use crate::libs::modeless_list::ModelessList;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::prelude::*;

pub struct Props {
    pub peer: Prop<Peer>,
    pub peer_id: Prop<String>,
    pub room: Prop<MeshRoom>,
    pub room_id: Prop<String>,
}

pub enum Msg {
    NoOp,
    SetTableToolIdx { idx: usize },
    OpenNewModeless { content: room_modeless::Content },
    FocusModeless { modeless_id: U128Id },
    CloseModeless { modeless_id: U128Id },
    SetModelessContainerElement { element: web_sys::Element },
    SetDraggingModelessTab { modeless_id: U128Id, tab_idx: usize },
    MoveModelessTab { modeless_id: Option<U128Id> },
}

pub enum On {}

pub struct Implement {
    peer: Prop<Peer>,
    peer_id: Prop<String>,
    room: Prop<MeshRoom>,
    room_id: Prop<String>,
    element_id: ElementId,

    table_tools: State<SelectList<TableTool>>,
    modeless_list: ModelessList<State<SelectList<room_modeless::Content>>>,
    modeless_container_element: Option<State<web_sys::Element>>,
    dragging_modeless_tab: Option<(U128Id, usize)>,
}

struct ElementId {
    header_room_id: String,
}

impl Constructor for Implement {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            peer: props.peer,
            peer_id: props.peer_id,
            room: props.room,
            room_id: props.room_id,
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

            Msg::SetTableToolIdx { idx } => {
                self.table_tools.set_selected_idx(idx);
                Cmd::none()
            }

            Msg::OpenNewModeless { content } => {
                self.modeless_list
                    .push(State::new(SelectList::new(vec![content], 0)));
                Cmd::none()
            }

            Msg::CloseModeless { modeless_id } => {
                self.modeless_list.remove(&modeless_id);
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

            Msg::MoveModelessTab { modeless_id: to_id } => {
                if let Some((from_id, from_idx)) = self.dragging_modeless_tab.take() {
                    let tab = if let Some(tab) = self
                        .modeless_list
                        .get_mut(&from_id)
                        .and_then(|c| c.remove(from_idx))
                    {
                        Some(tab)
                    } else {
                        None
                    };
                    if let Some(tab) = tab {
                        if let Some(to_content) =
                            to_id.and_then(|to_id| self.modeless_list.get_mut(&to_id))
                        {
                            to_content.push(tab);
                        } else {
                            self.modeless_list
                                .push(State::new(SelectList::new(vec![tab], 0)));
                        }
                    }
                    if let Some(from_content) = self.modeless_list.get(&from_id) {
                        if from_content.len() < 1 {
                            self.modeless_list.remove(&from_id);
                        }
                    }
                }
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(BasicApp::with_children(
            basic_app::Props {},
            Subscription::none(),
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
                            vec![self.render_modeless_container()],
                        ),
                    ],
                ),
            ],
        ))
    }
}

impl Implement {
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
            vec![Btn::with_children(
                btn::Props {
                    variant: btn::Variant::Primary,
                },
                Subscription::new(|sub| match sub {
                    btn::On::Click => Msg::OpenNewModeless {
                        content: room_modeless::Content::ChatPanel,
                    },
                }),
                vec![fa::i("fa-comment"), Html::text("チャットパネル")],
            )],
        )
    }

    fn render_modeless_container(&self) -> Html {
        if let Some(modeless_container_element) = self.modeless_container_element.as_ref() {
            Html::div(
                Attributes::new().class(Self::class("modeless-container")),
                Events::new(),
                self.modeless_list
                    .iter()
                    .map(|m| {
                        if let Some((modeless_id, z_index, content)) = m.as_ref() {
                            RoomModeless::empty(
                                room_modeless::Props {
                                    z_index: *z_index,
                                    content: content.as_prop(),
                                    container_element: modeless_container_element.as_prop(),
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
                                        room_modeless::On::DragTabStart { tab_idx } => {
                                            crate::debug::log_1("room_modeless::On::DragTabStart");
                                            Msg::SetDraggingModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                        room_modeless::On::DropTab => Msg::MoveModelessTab {
                                            modeless_id: Some(modeless_id),
                                        },
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
            "header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            "header-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
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

            "modeless-container" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": "0";
                "overflow": "hidden";
            }
        }
    }
}
