use super::{
    super::atom::btn::{self, Btn},
    super::atom::fa,
    super::atom::header::{self, Header},
    super::template::basic_app::{self, BasicApp},
    super::util::styled::{Style, Styled},
    super::util::{Prop, State},
    children::side_menu::{self, SideMenu},
    model::table::TableTool,
};
use crate::libs::select_list::SelectList;
use crate::skyway::{MeshRoom, Peer};
use kagura::prelude::*;

pub struct Props {
    pub peer: Prop<Peer>,
    pub peer_id: Prop<String>,
    pub room: Prop<MeshRoom>,
    pub room_id: Prop<String>,
}

pub enum Msg {
    SetTableToolIdx { idx: usize },
}

pub enum On {}

pub struct Implement {
    peer: Prop<Peer>,
    peer_id: Prop<String>,
    room: Prop<MeshRoom>,
    room_id: Prop<String>,
    element_id: ElementId,

    table_tools: State<SelectList<TableTool>>,
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
                header_room_id: format!("{:X}", crate::random_id::u128val()),
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
                    vec![self.render_header_row_0()],
                ),
                Html::div(
                    Attributes::new().class(Self::class("body")),
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
        }
    }
}
