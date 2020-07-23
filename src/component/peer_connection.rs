use super::{btn, room_connection};
use crate::{
    idb, random_id,
    skyway::{Peer, Room},
    Config, JsObject,
};
use kagura::prelude::*;
use regex::Regex;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

type PeerConnection = Component<Msg, Props, State, Sub>;

pub struct Props {
    pub config: Rc<Config>,
    pub client_id: Rc<String>,
    pub common_database: Rc<web_sys::IdbDatabase>,
}

pub struct State {
    peer_id: Option<String>,
    room: Option<Rc<Room>>,
    peer: Rc<Peer>,
    client_id: Rc<String>,
    inputing_room_id: String,
    error_message: Option<String>,
    room_id_regex: Regex,
    common_database: Rc<web_sys::IdbDatabase>,
    room_ids_buf: Vec<String>,
    rooms: Vec<RoomData>,
    config: Rc<Config>,
}

struct RoomData {
    room_id: String,
    last_access_time: js_sys::Date,
}

pub enum Msg {
    NoOp,
    SetPeerId(String),
    SetRoomId(String),
    TryToConnectToRoom(String),
    DisconnectFromRoom,
    ConnectToRoomAndPutDatabase(Rc<String>),
    ConnectToRoomAndAddDatabase(Rc<String>),
    ConnectToRoom(Rc<String>),
    SetRoomIdsWithJsValue(JsValue),
    SetRoomWithJsValue(String, JsValue),
}

pub enum Sub {
    Reconnect,
}

pub fn new() -> PeerConnection {
    Component::new(init, update, render)
}

fn init(this: &mut PeerConnection, state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>) {
    let peer = Rc::new(Peer::new(&props.config.skyway.key));
    let state = State {
        peer_id: None,
        room: None,
        peer: Rc::clone(&peer),
        client_id: props.client_id,
        inputing_room_id: "".into(),
        error_message: None,
        room_id_regex: Regex::new(r"^[A-Za-z0-9@#]{24}$").unwrap(),
        common_database: Rc::clone(&props.common_database),
        room_ids_buf: vec![],
        rooms: vec![],
        config: props.config,
    };
    let task = request_room_ids_task(&props.common_database);

    this.batch({
        let peer = Rc::clone(&peer);
        move |mut handler| {
            let a = Closure::wrap(Box::new({
                let peer = Rc::clone(&peer);
                move || handler(Msg::SetPeerId(peer.id()))
            }) as Box<dyn FnMut()>);
            peer.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    });

    (state, task)
}

fn basic_room_data_object() -> JsValue {
    let obj = object! {
        last_access_time: js_sys::Date::now()
    };
    let obj: js_sys::Object = obj.into();
    obj.into()
}

fn request_room_ids_task(database: &web_sys::IdbDatabase) -> Cmd<Msg, Sub> {
    let promise = idb::query(&database, "rooms", idb::Query::GetAllKeys);
    Cmd::task(move |resolve| {
        promise.then(move |x| {
            if let Some(x) = x {
                resolve(Msg::SetRoomIdsWithJsValue(x))
            }
        })
    })
}

fn request_room_task(database: &web_sys::IdbDatabase, room_id: Option<String>) -> Cmd<Msg, Sub> {
    if let Some(room_id) = room_id {
        let promise = idb::query(
            &database,
            "rooms",
            idb::Query::Get(&JsValue::from(&room_id)),
        );
        Cmd::task(move |resolve| {
            promise.then(move |x| {
                if let Some(x) = x {
                    resolve(Msg::SetRoomWithJsValue(room_id, x))
                }
            })
        })
    } else {
        Cmd::none()
    }
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetPeerId(peer_id) => {
            state.peer_id = Some(peer_id);
            Cmd::none()
        }
        Msg::SetRoomId(room_id) => {
            state.inputing_room_id = room_id;
            Cmd::none()
        }
        Msg::TryToConnectToRoom(room_id) => {
            if state.room_id_regex.is_match(&room_id) {
                let room_id = Rc::new(room_id);
                let promise = idb::query(
                    &state.common_database,
                    "rooms",
                    idb::Query::Get(&JsValue::from(room_id.as_str())),
                );
                Cmd::task(move |resolve| {
                    promise.then(move |x| {
                        if x.is_some() {
                            resolve(Msg::ConnectToRoomAndPutDatabase(room_id))
                        } else {
                            resolve(Msg::ConnectToRoomAndAddDatabase(room_id))
                        }
                    })
                })
            } else if room_id.len() > 20 {
                state.error_message = Some("ルームIDの文字数が多すぎます。".into());
                Cmd::none()
            } else if room_id.len() < 20 {
                state.error_message = Some("ルームIDの文字数が少なすぎます。".into());
                Cmd::none()
            } else {
                state.error_message = Some("ルームIDに不正な文字が含まれています。".into());
                Cmd::none()
            }
        }
        Msg::DisconnectFromRoom => {
            if let Some(room) = &mut state.room {
                room.payload.close();
                state.inputing_room_id = room.id.as_ref().clone();
            }
            state.room = None;
            Cmd::sub(Sub::Reconnect)
        }
        Msg::ConnectToRoomAndPutDatabase(room_id) => {
            let promise = idb::query(
                &state.common_database,
                "rooms",
                idb::Query::Put(&JsValue::from(room_id.as_str()), &basic_room_data_object()),
            );
            Cmd::task(move |resolve| promise.then(move |_| resolve(Msg::ConnectToRoom(room_id))))
        }
        Msg::ConnectToRoomAndAddDatabase(room_id) => {
            let promise = idb::query(
                &state.common_database,
                "rooms",
                idb::Query::Add(&JsValue::from(room_id.as_str()), &basic_room_data_object()),
            );
            Cmd::task(move |resolve| promise.then(move |_| resolve(Msg::ConnectToRoom(room_id))))
        }
        Msg::ConnectToRoom(room_id) => {
            state.room = Some(Rc::new(Room::new(state.peer.join_room(&room_id), room_id)));
            state.inputing_room_id = "".into();
            Cmd::none()
        }
        Msg::SetRoomIdsWithJsValue(rooms) => {
            let raw_rooms = js_sys::Array::from(&rooms).to_vec();
            let mut room_ids = vec![];

            for raw_room in raw_rooms {
                if let Some(room) = raw_room.as_string() {
                    room_ids.push(room);
                }
            }

            state.room_ids_buf = room_ids;
            request_room_task(&state.common_database, state.room_ids_buf.pop())
        }
        Msg::SetRoomWithJsValue(room_id, room) => {
            if let Ok(room) = room.dyn_into::<JsObject>() {
                let last_access_time = room.get("last_access_time").unwrap().as_f64().unwrap();
                let last_access_time = js_sys::Date::new(&JsValue::from(last_access_time));
                state.rooms.push(RoomData {
                    room_id: room_id,
                    last_access_time: last_access_time,
                });
            }
            request_room_task(&state.common_database, state.room_ids_buf.pop())
        }
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    if let Some(room) = &state.room {
        Html::component(
            room_connection::new()
                .with(room_connection::Props {
                    config: Rc::clone(&state.config),
                    peer: Rc::clone(&state.peer),
                    room: Rc::clone(room),
                    client_id: Rc::clone(&state.client_id),
                    common_database: Rc::clone(&state.common_database),
                })
                .subscribe(|sub| match sub {
                    room_connection::Sub::DisconnectFromRoom => Msg::DisconnectFromRoom,
                }),
            vec![],
        )
    } else {
        Html::div(
            Attributes::new().id("app").class("fullscreen").class("app"),
            Events::new(),
            vec![
                render_header(&state.inputing_room_id),
                render_body(&state.rooms),
            ],
        )
    }
}

fn render_header(room_id: &String) -> Html {
    Html::div(
        Attributes::new().class("panel grid"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("grid-w-12 keyvalueoption pure-form"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new().string("for", "roomid"),
                        Events::new(),
                        vec![Html::text("接続先のルームID")],
                    ),
                    Html::input(
                        Attributes::new().value(room_id).id("roomid"),
                        Events::new().on_input(|x| Msg::SetRoomId(x)),
                        vec![],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click({
                            let room_id = room_id.clone();
                            move |_| Msg::TryToConnectToRoom(room_id)
                        }),
                        vec![Html::text("接続")],
                    ),
                ],
            ),
            Html::div(
                Attributes::new()
                    .class("grid-w-12")
                    .class("justify-r")
                    .class("centering-h"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| {
                            let room_id = random_id::base64url();
                            Msg::TryToConnectToRoom(room_id)
                        }),
                        vec![Html::text("新規ルームを作成")],
                    )],
                )],
            ),
        ],
    )
}

fn render_body(rooms: &Vec<RoomData>) -> Html {
    Html::div(
        Attributes::new()
            .class("scroll-v")
            .class("grid")
            .class("scroll-v")
            .class("container"),
        Events::new(),
        vec![Html::div(
            Attributes::new().class("grid-cc-2x6").class("linear-v"),
            Events::new(),
            vec![
                Html::h3(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("接続履歴")],
                ),
                Html::div(
                    Attributes::new()
                        .class("pure-form")
                        .class("keyvalue")
                        .class("keyvalue-rev"),
                    Events::new(),
                    rooms
                        .iter()
                        .map(|room| {
                            vec![
                                Html::input(
                                    Attributes::new().value(&room.room_id).flag("readonly"),
                                    Events::new(),
                                    vec![],
                                ),
                                Html::div(
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text(
                                        room.last_access_time
                                            .to_locale_string("ja-JP", object! {}.as_ref())
                                            .as_string()
                                            .unwrap_or(String::from("")),
                                    )],
                                ),
                            ]
                        })
                        .flatten()
                        .collect(),
                ),
            ],
        )],
    )
}
