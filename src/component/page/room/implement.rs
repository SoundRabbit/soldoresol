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
    SetTableToolIdx { idx: usize },
    OpenNewModeless { content: room_modeless::Content },
}

pub enum On {}

pub struct Implement {
    peer: Prop<Peer>,
    peer_id: Prop<String>,
    room: Prop<MeshRoom>,
    room_id: Prop<String>,
    element_id: ElementId,

    table_tools: State<SelectList<TableTool>>,
    modeless_list: ModelessList<State<room_modeless::Content>>,
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
            Msg::SetTableToolIdx { idx } => {
                self.table_tools.set_selected_idx(idx);
                Cmd::none()
            }

            Msg::OpenNewModeless { content } => {
                self.modeless_list.push(State::new(content));
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
        Html::div(
            Attributes::new().class(Self::class("modeless-container")),
            Events::new(),
            self.modeless_list
                .iter()
                .map(|m| {
                    if let Some((modeless_id, z_index, content)) = m.as_ref() {
                        RoomModeless::empty(
                            room_modeless::Props {
                                modeless_id: U128Id::clone(&modeless_id),
                                z_index: *z_index,
                                content: content.as_prop(),
                            },
                            Subscription::none(),
                        )
                    } else {
                        Html::div(Attributes::new(), Events::new(), vec![])
                    }
                })
                .collect(),
        )
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
            }
        }
    }
}
