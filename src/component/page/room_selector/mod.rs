use super::atom::{
    btn::{self, Btn},
    card::{self, Card},
    dropdown::{self, Dropdown},
    header::{self, Header},
    heading::{self, Heading},
};
use super::molecule::dialog::{self, Dialog};
use super::organism::{
    modal_notification::{self, ModalNotification},
    modal_sign_in::{self, ModalSignIn},
};
use super::template::{
    basic_app::{self, BasicApp},
    loader::{self, Loader},
};
use crate::libs::gapi::gapi;
use crate::model::config::Config;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use regex::Regex;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod task;

pub struct Props {
    pub common_db: Rc<web_sys::IdbDatabase>,
    pub room_db: Rc<web_sys::IdbDatabase>,
}

pub enum Msg {
    NoOp,
    SetRoomDb(Rc<web_sys::IdbDatabase>),
    SetShowingModal(ShowingModal),
    SetRooms(Vec<RoomData>),
    SetInputingRoomId(String),
    ConnectWithRoomId(String),
    ConnectWithInputingRoomId,
    ConnectWithNewRoomId,
    SetGoogleLoginedState(bool),
    RemoveRoomToCloseModal(String),
}

pub enum On {
    Connect(String),
    SetRoomDb(Rc<web_sys::IdbDatabase>),
}

pub struct RoomSelector {
    rooms: Option<Vec<RoomData>>,
    inputing_annot_room_id: String,
    annot_room_id_validator: Regex,
    showing_modal: ShowingModal,
    common_db: Rc<web_sys::IdbDatabase>,
    room_db: Rc<web_sys::IdbDatabase>,
    google_drive_listener: kagura::util::Batch<Cmd<Self>>,
    is_signed_in_to_google: bool,
    element_id: ElementId,
}

pub enum ShowingModal {
    None,
    Notification,
    SignIn,
    ConfirmingToRemoveRoom { room_id: String, name: String },
}

pub struct RoomData {
    id: String,
    name: String,
    last_access_time: js_sys::Date,
    description: String,
}

ElementId! {
    input_room_id
}

impl Component for RoomSelector {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomSelector {}

impl Constructor for RoomSelector {
    fn constructor(props: Self::Props) -> Self {
        Self {
            rooms: None,
            inputing_annot_room_id: String::from(""),
            annot_room_id_validator: Regex::new(r"^(skyway/[A-Za-z0-9@#]{24}|drive/[A-Za-z\-_]+)$")
                .unwrap(),
            showing_modal: ShowingModal::Notification,
            common_db: props.common_db,
            room_db: props.room_db,
            google_drive_listener: kagura::util::Batch::new(|mut resolve| {
                let a = Closure::wrap(Box::new(move |is_signed_in| {
                    resolve(Cmd::chain(Msg::SetGoogleLoginedState(is_signed_in)));
                }) as Box<dyn FnMut(bool)>);
                gapi.auth2()
                    .get_auth_instance()
                    .is_signed_in()
                    .listen(a.as_ref().unchecked_ref());
                a.forget();
            }),
            is_signed_in_to_google: gapi.auth2().get_auth_instance().is_signed_in().get(),
            element_id: ElementId::new(),
        }
    }
}

impl Update for RoomSelector {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::list(vec![
            Cmd::task({
                let common_db = Rc::clone(&self.common_db);
                async move {
                    if let Some(rooms) = task::get_room_index(&common_db).await {
                        crate::debug::log_1("success to load index of rooms");
                        Cmd::chain(Msg::SetRooms(rooms))
                    } else {
                        crate::debug::log_1("faild to load index of rooms");
                        Cmd::none()
                    }
                }
            }),
            Cmd::chain(Msg::SetGoogleLoginedState(
                gapi.auth2().get_auth_instance().is_signed_in().get(),
            )),
        ])
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetRoomDb(room_db) => {
                self.room_db = Rc::clone(&room_db);
                Cmd::submit(On::SetRoomDb(room_db))
            }
            Msg::SetShowingModal(showing_modal) => {
                crate::debug::log_1("SetShowingModal");
                self.showing_modal = showing_modal;
                Cmd::none()
            }
            Msg::SetRooms(rooms) => {
                self.rooms = Some(rooms);
                Cmd::none()
            }
            Msg::SetInputingRoomId(inputing_room_id) => {
                self.inputing_annot_room_id = inputing_room_id;
                Cmd::none()
            }
            Msg::ConnectWithRoomId(room_id) => Cmd::submit(On::Connect(room_id)),
            Msg::ConnectWithInputingRoomId => {
                if self
                    .annot_room_id_validator
                    .is_match(&self.inputing_annot_room_id)
                {
                    Cmd::submit(On::Connect(self.inputing_annot_room_id.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::ConnectWithNewRoomId => {
                let room_id = crate::libs::random_id::base64url();
                Cmd::submit(On::Connect(room_id))
            }
            Msg::SetGoogleLoginedState(is_signed_in) => {
                self.is_signed_in_to_google = is_signed_in;
                Cmd::list(vec![
                    Cmd::task(async {
                        if gapi.auth2().get_auth_instance().is_signed_in().get() {
                            task::initialize_google_drive().await;
                            Cmd::none()
                        } else {
                            Cmd::none()
                        }
                    }),
                    Cmd::task(self.google_drive_listener.poll()),
                ])
            }
            Msg::RemoveRoomToCloseModal(room_id) => {
                self.showing_modal = ShowingModal::None;
                Cmd::task({
                    let common_db = Rc::clone(&self.common_db);
                    let room_db = Rc::clone(&self.room_db);
                    async move {
                        let mut cmds = vec![];
                        if let Some(room_db) =
                            task::remove_room(&room_id, &common_db, &room_db).await
                        {
                            cmds.push(Cmd::chain(Msg::SetRoomDb(Rc::new(room_db))));
                        }
                        if let Some(rooms) = task::get_room_index(&common_db).await {
                            cmds.push(Cmd::chain(Msg::SetRooms(rooms)));
                        }
                        Cmd::list(cmds)
                    }
                })
            }
        }
    }
}

impl Render<Html> for RoomSelector {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        crate::debug::log_1("render RoomSelector");
        Self::styled(match &self.rooms {
            None => Loader::empty(self, None, loader::Props {}, Sub::none()),
            Some(rooms) => BasicApp::new(
                self,
                None,
                basic_app::Props {},
                Sub::none(),
                vec![
                    Header::new(
                        self,
                        None,
                        header::Props {},
                        Sub::none(),
                        (
                            Attributes::new(),
                            Events::new(),
                            vec![self.render_header_row_0(), self.render_header_row_1()],
                        ),
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
                    self.render_modal(),
                ],
            ),
        })
    }
}

impl RoomSelector {
    fn render_modal(&self) -> Html {
        match &self.showing_modal {
            ShowingModal::None => {
                crate::debug::log_1("ShowingModal::None");
                super::atom::common::Common::none()
            }
            ShowingModal::Notification => {
                crate::debug::log_1("ShowingModal::Notification");
                ModalNotification::empty(
                    self,
                    None,
                    modal_notification::Props {},
                    Sub::map({
                        let is_signed_in = self.is_signed_in_to_google;
                        move |sub| {
                            crate::debug::log_1("modal_notification::On::Close");
                            match sub {
                                modal_notification::On::Close => {
                                    if is_signed_in {
                                        Msg::SetShowingModal(ShowingModal::None)
                                    } else {
                                        Msg::SetShowingModal(ShowingModal::SignIn)
                                    }
                                }
                            }
                        }
                    }),
                )
            }
            ShowingModal::SignIn => {
                crate::debug::log_1("ShowingModal::SignIn");
                ModalSignIn::empty(
                    self,
                    None,
                    modal_sign_in::Props {},
                    Sub::map(|sub| match sub {
                        modal_sign_in::On::Close => Msg::SetShowingModal(ShowingModal::None),
                    }),
                )
            }
            ShowingModal::ConfirmingToRemoveRoom { room_id, name } => Dialog::new(
                self,
                None,
                dialog::Props {},
                Sub::none(),
                (
                    String::from("ルームの削除"),
                    format!(
                        "ルームデータ\n\tID\t{}\n\t名前\t{}\nを削除します。",
                        room_id, name
                    ),
                    vec![
                        dialog::Button::Yes(Events::new().on_click(self, {
                            let room_id = room_id.clone();
                            move |_| Msg::RemoveRoomToCloseModal(room_id)
                        })),
                        dialog::Button::No(
                            Events::new()
                                .on_click(self, |_| Msg::SetShowingModal(ShowingModal::None)),
                        ),
                    ],
                ),
            ),
        }
    }

    fn render_header_row_0(&self) -> Html {
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
                        Events::new().on_click(self, |_| Msg::ConnectWithNewRoomId),
                        vec![Html::text("新規ルームを作成")],
                    )],
                ),
            ],
        )
    }

    fn render_header_row_0_left(&self) -> Html {
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
                        .value(&self.inputing_annot_room_id),
                    Events::new().on_input(self, |room_id| Msg::SetInputingRoomId(room_id)),
                    vec![],
                ),
                Btn::with_variant(
                    if self
                        .annot_room_id_validator
                        .is_match(&self.inputing_annot_room_id)
                    {
                        btn::Variant::Primary
                    } else {
                        btn::Variant::Disable
                    },
                    Attributes::new(),
                    Events::new().on_click(self, |_| Msg::ConnectWithInputingRoomId),
                    vec![Html::text("接続")],
                ),
            ],
        )
    }

    fn render_header_row_1(&self) -> Html {
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
                    vec![if self.is_signed_in_to_google {
                        Btn::danger(
                            Attributes::new(),
                            Events::new().on_click(self, |_| {
                                gapi.auth2().get_auth_instance().sign_out();
                                Msg::NoOp
                            }),
                            vec![Html::text("サインアウト")],
                        )
                    } else {
                        Btn::success(
                            Attributes::new(),
                            Events::new()
                                .on_click(self, |_| Msg::SetShowingModal(ShowingModal::SignIn)),
                            vec![Html::text("サインイン")],
                        )
                    }],
                ),
            ],
        )
    }

    fn render_roomcard(&self, room: &RoomData) -> Html {
        Html::div(
            Attributes::new().class(Self::class("card")),
            Events::new(),
            vec![Card::new(
                self,
                None,
                card::Props {},
                Sub::none(),
                vec![
                    Dropdown::new(
                        self,
                        None,
                        dropdown::Props {
                            direction: dropdown::Direction::BottomLeft,
                            variant: btn::Variant::Menu,
                            ..Default::default()
                        },
                        Sub::none(),
                        (
                            vec![Html::text(&room.name)],
                            vec![
                                Btn::menu(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let room_id = format!("skyway/{}", room.id);
                                        move |_| Msg::ConnectWithRoomId(room_id)
                                    }),
                                    vec![Html::text("開く")],
                                ),
                                Btn::menu(
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text("ダウンロード")],
                                ),
                                Btn::menu(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let room_id = room.id.clone();
                                        let name = room.name.clone();
                                        move |_| {
                                            Msg::SetShowingModal(
                                                ShowingModal::ConfirmingToRemoveRoom {
                                                    room_id,
                                                    name,
                                                },
                                            )
                                        }
                                    }),
                                    vec![Html::text("削除")],
                                ),
                            ],
                        ),
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
