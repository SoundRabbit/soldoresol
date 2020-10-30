use super::atom::{
    btn::{self, Btn},
    card::{self, Card},
    header::{self, Header},
};
use super::organism::modal_notification::{self, ModalNotification};
use super::template::{
    basic_app::{self, BasicApp},
    loader::{self, Loader},
};
use super::util::styled::{Style, Styled};
use super::util::Prop;
use kagura::prelude::*;
mod task;

pub struct Props {
    pub common_database: Prop<web_sys::IdbDatabase>,
}

pub enum Msg {
    SetRooms(Vec<RoomData>),
}

pub enum On {}

pub struct RoomSelector {
    rooms: Option<Vec<RoomData>>,
    element_id: ElementId,
}

pub struct RoomData {
    id: String,
    name: String,
    last_access_time: js_sys::Date,
    description: String,
}

struct ElementId {
    input_room_id: String,
}

impl Constructor for RoomSelector {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        crate::debug::log_1(format!("construct {}", std::any::type_name::<Self>()));

        builder.set_cmd(Cmd::task({
            let common_database = props.common_database.clone();
            move |resolve| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(rooms) = task::get_room_index(&common_database).await {
                        crate::debug::log_1("success to load index of rooms");
                        resolve(Msg::SetRooms(rooms));
                    } else {
                        crate::debug::log_1("faild to load index of rooms");
                    }
                });
            }
        }));

        Self {
            rooms: None,
            element_id: ElementId {
                input_room_id: format!("{:X}", crate::random_id::u128val()),
            },
        }
    }
}

impl Component for RoomSelector {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        crate::debug::log_1(format!("init {}", std::any::type_name::<Self>()));
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetRooms(rooms) => {
                self.rooms = Some(rooms);
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(match &self.rooms {
            None => Loader::empty(loader::Props {}, Subscription::none()),
            Some(rooms) => BasicApp::with_children(
                basic_app::Props {},
                Subscription::none(),
                vec![
                    ModalNotification::empty(
                        modal_notification::Props { state: None },
                        Subscription::none(),
                    ),
                    Header::with_children(
                        header::Props::new(),
                        Subscription::none(),
                        vec![Html::div(
                            Attributes::new()
                                .class(Self::class("header-row"))
                                .class("pure-form"),
                            Events::new(),
                            vec![
                                Html::div(
                                    Attributes::new().class(Self::class("input-room-id")),
                                    Events::new(),
                                    vec![
                                        Html::label(
                                            Attributes::new()
                                                .class(Self::class("label"))
                                                .string("for", &self.element_id.input_room_id),
                                            Events::new(),
                                            vec![Html::text("接続先のルームID")],
                                        ),
                                        Html::input(
                                            Attributes::new()
                                                .class("pure-input")
                                                .id(&self.element_id.input_room_id),
                                            Events::new(),
                                            vec![],
                                        ),
                                        Btn::with_children(
                                            btn::Props {
                                                variant: btn::Variant::Primary,
                                            },
                                            Subscription::none(),
                                            vec![Html::text("接続")],
                                        ),
                                    ],
                                ),
                                Html::div(
                                    Attributes::new().class(Self::class("right")),
                                    Events::new(),
                                    vec![Btn::with_children(
                                        btn::Props {
                                            variant: btn::Variant::Primary,
                                        },
                                        Subscription::none(),
                                        vec![Html::text("新規ルームを作成")],
                                    )],
                                ),
                            ],
                        )],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("body")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::h1(
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text("履歴")],
                                )],
                            ),
                            Html::div(
                                Attributes::new(),
                                Events::new(),
                                rooms
                                    .iter()
                                    .map(|room| {
                                        Card::with_children(
                                            card::Props {},
                                            Subscription::none(),
                                            vec![
                                                Html::h2(
                                                    Attributes::new()
                                                        .class(Self::class("room-name")),
                                                    Events::new(),
                                                    vec![Html::text(&room.name)],
                                                ),
                                                Html::aside(
                                                    Attributes::new().class(Self::class("room-id")),
                                                    Events::new(),
                                                    vec![Html::text(&room.id)],
                                                ),
                                                Html::dl(
                                                    Attributes::new(),
                                                    Events::new(),
                                                    vec![
                                                        Html::dt(
                                                            Attributes::new(),
                                                            Events::new(),
                                                            vec![Html::text("最終使用")],
                                                        ),
                                                        Html::dd(
                                                            Attributes::new(),
                                                            Events::new(),
                                                            vec![Html::text(
                                                                room.last_access_time
                                                                    .to_locale_string(
                                                                        "ja-JP",
                                                                        object! {}.as_ref(),
                                                                    )
                                                                    .as_string()
                                                                    .unwrap_or(String::from("")),
                                                            )],
                                                        ),
                                                        Html::dt(
                                                            Attributes::new(),
                                                            Events::new(),
                                                            vec![Html::text("メモ")],
                                                        ),
                                                        Html::dd(
                                                            Attributes::new(),
                                                            Events::new(),
                                                            vec![Html::text(&room.description)],
                                                        ),
                                                    ],
                                                ),
                                            ],
                                        )
                                    })
                                    .collect(),
                            ),
                        ],
                    ),
                ],
            ),
        })
    }
}

impl Styled for RoomSelector {
    fn style() -> Style {
        style! {
            "header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            "input-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "column-gap": "0.65em";
            }

            "label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            "right" {
                "display": "flex";
                "justify-content": "flex-end";
            }

            "room-name" {
                "border-bottom": format!("0.1em solid {}", crate::color_system::gray(255, 9));
            }

            "room-id" {
                "background-color": crate::color_system::gray(255, 2).to_string();
                "padding": "0.25em";
                "border-radius": "2px";
            }
        }
    }
}
