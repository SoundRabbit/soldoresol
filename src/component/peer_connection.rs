use super::{awesome, btn, room_connection};
use crate::{
    indexed_db, random_id,
    skyway::{Peer, Room},
    Config,
};
use kagura::prelude::*;
use regex::Regex;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub fn new(
    config: Rc<Config>,
    common_database: Rc<web_sys::IdbDatabase>,
) -> Component<Msg, State, Sub> {
    let peer = Rc::new(Peer::new(&config.skyway.key));
    let init = {
        let peer = Rc::clone(&peer);
        move || {
            let state = State {
                peer_id: None,
                room: None,
                peer: peer,
                inputing_room_id: "".into(),
                error_message: None,
                room_id_regex: Regex::new(r"^[A-Za-z0-9@#]{20}$").unwrap(),
                common_database: Rc::clone(&common_database),
                rooms: vec![],
            };
            let task = request_rooms_task(&common_database);
            (state, task)
        }
    };
    Component::new(init, update, render).batch({
        let peer = Rc::clone(&peer);
        move |mut handler| {
            let a = Closure::wrap(Box::new({
                let peer = Rc::clone(&peer);
                move || handler(Msg::SetPeerId(peer.id()))
            }) as Box<dyn FnMut()>);
            peer.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    })
}

pub struct State {
    peer_id: Option<String>,
    room: Option<Rc<Room>>,
    peer: Rc<Peer>,
    inputing_room_id: String,
    error_message: Option<String>,
    room_id_regex: Regex,
    common_database: Rc<web_sys::IdbDatabase>,
    rooms: Vec<String>,
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
    SetRoomsWithJsValue(JsValue),
}

pub enum Sub {}

fn basic_room_data_object() -> JsValue {
    let obj = object! {
        last_access_time: js_sys::Date::now()
    };
    let obj: js_sys::Object = obj.into();
    obj.into()
}

fn request_rooms_task(database: &web_sys::IdbDatabase) -> Cmd<Msg, Sub> {
    indexed_db::query(
        &database,
        "rooms",
        indexed_db::Query::GetAllKeys,
        |x| Msg::SetRoomsWithJsValue(x),
        |_| Msg::NoOp,
    )
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
                indexed_db::query(
                    &state.common_database,
                    "rooms",
                    indexed_db::Query::Get(&JsValue::from(room_id.as_str())),
                    {
                        let room_id = Rc::clone(&room_id);
                        move |_| Msg::ConnectToRoomAndPutDatabase(room_id)
                    },
                    {
                        let room_id = Rc::clone(&room_id);
                        move |_| Msg::ConnectToRoomAndAddDatabase(room_id)
                    },
                )
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
            state.peer.destroy();
            state.peer.reconnect();
            request_rooms_task(&state.common_database)
        }
        Msg::ConnectToRoomAndPutDatabase(room_id) => indexed_db::query(
            &state.common_database,
            "rooms",
            indexed_db::Query::Put(&JsValue::from(room_id.as_str()), &basic_room_data_object()),
            |_| Msg::ConnectToRoom(room_id),
            |_| Msg::NoOp,
        ),
        Msg::ConnectToRoomAndAddDatabase(room_id) => indexed_db::query(
            &state.common_database,
            "rooms",
            indexed_db::Query::Add(&JsValue::from(room_id.as_str()), &basic_room_data_object()),
            |_| Msg::ConnectToRoom(room_id),
            |_| Msg::NoOp,
        ),
        Msg::ConnectToRoom(room_id) => {
            state.room = Some(Rc::new(Room::new(state.peer.join_room(&room_id), room_id)));
            state.inputing_room_id = "".into();
            Cmd::none()
        }
        Msg::SetRoomsWithJsValue(rooms) => {
            let raw_rooms = js_sys::Array::from(&rooms).to_vec();
            let mut rooms = vec![];

            for raw_room in raw_rooms {
                if let Some(room) = raw_room.as_string() {
                    rooms.push(room);
                }
            }

            state.rooms = rooms;

            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    if let Some(room) = &state.room {
        Html::component(
            room_connection::new(Rc::clone(&state.peer), Rc::clone(room)).subscribe(
                |sub| match sub {
                    room_connection::Sub::DisconnectFromRoom => Msg::DisconnectFromRoom,
                },
            ),
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

fn render_header(room_id: &String) -> Html<Msg> {
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

fn render_body(rooms: &Vec<String>) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("container")
            .class("grid")
            .class("scroll-v"),
        Events::new(),
        vec![Html::div(
            Attributes::new()
                .class("grid-cc-2x6")
                .class("pure-form")
                .class("linear-v"),
            Events::new(),
            vec![
                vec![Html::h3(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("接続履歴")],
                )],
                rooms
                    .iter()
                    .map(|room_id| {
                        Html::input(
                            Attributes::new().value(room_id).flag("readonly"),
                            Events::new(),
                            vec![],
                        )
                    })
                    .collect(),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )],
    )
}
