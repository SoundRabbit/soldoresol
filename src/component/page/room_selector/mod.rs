use super::atom::{
    btn::{self, Btn},
    card::{self, Card},
    dropdown::{self, Dropdown},
    header::{self, Header},
    heading::{self, Heading},
};
use super::organism::{
    modal_notification::{self, ModalNotification},
    modal_sign_in::{self, ModalSignIn},
};
use super::template::{
    basic_app::{self, BasicApp},
    loader::{self, Loader},
};
use crate::libs::gapi::{gapi, GoogleResponse};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use regex::Regex;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod task;

pub struct Props {
    pub common_db: Rc<web_sys::IdbDatabase>,
}

pub enum Msg {
    NoOp,
    SetIsShowingModalSignIn(bool),
    SetRooms(Vec<RoomData>),
    SetInputingRoomId(String),
    ConnectWithRoomId(String),
    ConnectWithInputingRoomId,
    ConnectWithNewRoomId,
    InitializeGoogleDrive,
}

pub enum On {
    Connect(String),
}

pub struct RoomSelector {
    rooms: Option<Vec<RoomData>>,
    inputing_room_id: String,
    room_id_validator: Regex,
    is_showing_modal_sign_in: bool,
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

impl Component for RoomSelector {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomSelector {
    fn constructor(_: &Props) -> Self {
        Self {
            rooms: None,
            inputing_room_id: String::from(""),
            room_id_validator: Regex::new(r"^[A-Za-z0-9@#]{24}$").unwrap(),
            is_showing_modal_sign_in: !gapi.auth2().get_auth_instance().is_signed_in().get(),
            element_id: ElementId {
                input_room_id: format!("{:X}", crate::libs::random_id::u128val()),
            },
        }
    }
}

impl Update for RoomSelector {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        Cmd::list(vec![
            Cmd::task({
                let common_db = props.common_db.clone();
                move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(rooms) = task::get_room_index(&common_db).await {
                            crate::debug::log_1("success to load index of rooms");
                            resolve(Msg::SetRooms(rooms));
                        } else {
                            crate::debug::log_1("faild to load index of rooms");
                        }
                    });
                }
            }),
            Cmd::batch(|mut handle| {
                let a = Closure::wrap(Box::new(move |is_signed_in| {
                    if is_signed_in {
                        wasm_bindgen_futures::spawn_local(async {
                            task::initialize_google_drive().await;
                        });
                    } else {
                        handle(Msg::NoOp);
                    }
                }) as Box<dyn FnMut(bool)>);
                gapi.auth2()
                    .get_auth_instance()
                    .is_signed_in()
                    .listen(a.as_ref().unchecked_ref());
                a.forget();
            }),
            Cmd::task(|resolve| {
                if gapi.auth2().get_auth_instance().is_signed_in().get() {
                    wasm_bindgen_futures::spawn_local(async {
                        task::initialize_google_drive().await;
                    });
                }
            }),
        ])
    }

    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetIsShowingModalSignIn(is_showing) => {
                self.is_showing_modal_sign_in = is_showing;
                Cmd::none()
            }
            Msg::SetRooms(rooms) => {
                self.rooms = Some(rooms);
                Cmd::none()
            }
            Msg::SetInputingRoomId(inputing_room_id) => {
                self.inputing_room_id = inputing_room_id;
                Cmd::none()
            }
            Msg::ConnectWithRoomId(room_id) => Cmd::Sub(On::Connect(room_id)),
            Msg::ConnectWithInputingRoomId => {
                if self.room_id_validator.is_match(&self.inputing_room_id) {
                    Cmd::Sub(On::Connect(self.inputing_room_id.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::ConnectWithNewRoomId => {
                let room_id = crate::libs::random_id::base64url();
                Cmd::Sub(On::Connect(room_id))
            }
            Msg::InitializeGoogleDrive => Cmd::task(|resolve| {}),
        }
    }
}

impl Render for RoomSelector {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(match &self.rooms {
            None => Loader::empty(loader::Props {}, Sub::none()),
            Some(rooms) => BasicApp::with_children(
                basic_app::Props {},
                Sub::none(),
                vec![
                    ModalNotification::empty(
                        modal_notification::Props { is_showing: None },
                        Sub::none(),
                    ),
                    ModalSignIn::empty(
                        modal_sign_in::Props {
                            is_showing: Some(self.is_showing_modal_sign_in),
                        },
                        Sub::map(|sub| match sub {
                            modal_sign_in::On::Close => Msg::SetIsShowingModalSignIn(false),
                        }),
                    ),
                    Header::with_children(
                        header::Props::new(),
                        Sub::none(),
                        vec![self.render_header_row_0(), self.render_header_row_1()],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("body")),
                        Events::new(),
                        vec![
                            Heading::h2(
                                heading::Variant::Light,
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text("自動保存されたルーム")],
                            ),
                            Html::div(
                                Attributes::new().class(Self::class("card-container")),
                                Events::new(),
                                rooms
                                    .iter()
                                    .map(|room| self.render_roomcard(room))
                                    .collect(),
                            ),
                        ],
                    ),
                ],
            ),
        })
    }
}

impl RoomSelector {
    fn render_header_row_0(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_header_row_0_left(),
                Html::div(
                    Attributes::new().class(Self::class("right")),
                    Events::new(),
                    vec![Btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::ConnectWithNewRoomId),
                        vec![Html::text("新規ルームを作成")],
                    )],
                ),
            ],
        )
    }

    fn render_header_row_0_left(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("input-room-id")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(Self::class("label"))
                        .class(Self::class("input-room-id-label"))
                        .string("for", &self.element_id.input_room_id),
                    Events::new(),
                    vec![Html::text("接続先のルームID")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_room_id)
                        .value(&self.inputing_room_id),
                    Events::new().on_input(Msg::SetInputingRoomId),
                    vec![],
                ),
                Btn::with_variant(
                    if self.room_id_validator.is_match(&self.inputing_room_id) {
                        btn::Variant::Primary
                    } else {
                        btn::Variant::Disable
                    },
                    Attributes::new(),
                    Events::new().on_click(|_| Msg::ConnectWithInputingRoomId),
                    vec![Html::text("接続")],
                ),
            ],
        )
    }

    fn render_header_row_1(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![
                Html::div(Attributes::new(), Events::new(), vec![]),
                Html::div(
                    Attributes::new().class(Self::class("right")),
                    Events::new(),
                    vec![if gapi.auth2().get_auth_instance().is_signed_in().get() {
                        Btn::danger(
                            Attributes::new(),
                            Events::new().on_click(|_| {
                                gapi.auth2().get_auth_instance().sign_out();
                                Msg::NoOp
                            }),
                            vec![Html::text("サインアウト")],
                        )
                    } else {
                        Btn::success(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::SetIsShowingModalSignIn(true)),
                            vec![Html::text("サインイン")],
                        )
                    }],
                ),
            ],
        )
    }

    fn render_roomcard(&self, room: &RoomData) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("card")),
            Events::new(),
            vec![Card::with_children(
                card::Props {},
                Sub::none(),
                vec![
                    Dropdown::with_children(
                        dropdown::Props {
                            direction: dropdown::Direction::BottomLeft,
                            text: dropdown::Text::Text(room.name.clone()),
                            variant: btn::Variant::Menu,
                            ..Default::default()
                        },
                        Sub::none(),
                        vec![
                            Btn::menu(
                                Attributes::new(),
                                Events::new().on_click({
                                    let room_id = room.id.clone();
                                    move |_| Msg::ConnectWithRoomId(room_id)
                                }),
                                vec![Html::text("開く")],
                            ),
                            Btn::menu(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text("ダウンロード")],
                            ),
                            Btn::menu(Attributes::new(), Events::new(), vec![Html::text("削除")]),
                        ],
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
                                Attributes::new()
                                    .class(Heading::class_name(6, &heading::Variant::Light)),
                                Events::new(),
                                vec![Html::text("最終使用")],
                            ),
                            Html::dd(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text(
                                    room.last_access_time
                                        .to_locale_string("ja-JP", object! {}.as_ref())
                                        .as_string()
                                        .unwrap_or(String::from("")),
                                )],
                            ),
                            Html::dt(
                                Attributes::new()
                                    .class(Heading::class_name(6, &heading::Variant::Light)),
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
            )],
        )
    }
}

impl Styled for RoomSelector {
    fn style() -> Style {
        style! {
            ".header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            ".input-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "column-gap": "0.65em";
            }

            ".label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            ".right" {
                "display": "flex";
                "justify-content": "flex-end";
            }

            ".body" {
                "padding": "0.65em";
            }

            ".card-container" {
                "display": "flex";
                "flex-wrap": "wrap";
            }

            ".card" {
                "max-width": "max-content";
                "max-height": "max-content";
            }

            ".room-id" {
                "background-color": crate::libs::color::color_system::gray(100, 2).to_string();
                "padding": "0.25em";
                "border-radius": "2px";
            }

            @media "(max-width: 40rem)" {
                ".header-row" {
                    "display": "flex";
                    "flex-direction": "column-reverse";
                }

                ".input-room-id" {
                    "grid-template-columns": "1fr max-content";
                    "grid-auto-rows": "max-content";
                }

                ".input-room-id-label" {
                    "grid-column": "1 / -1";
                }

                ".card-container" {
                    "justify-content": "center";
                }
            }
        }
    }
}
