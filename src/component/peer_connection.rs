use super::btn;
use super::room_connection;
use crate::random_id;
use crate::skyway::{Peer, Room};
use crate::Config;
use kagura::prelude::*;
use regex::Regex;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn new(config: Rc<Config>) -> Component<Msg, State, Sub> {
    let peer = Rc::new(Peer::new(&config.skyway.key));
    Component::new(init(Rc::clone(&peer)), update, render).batch({
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
}

pub enum Msg {
    SetPeerId(String),
    SetRoomId(String),
    ConnectToRoom(String),
    DisconnectFromRoom,
}

pub enum Sub {}

fn init(peer: Rc<Peer>) -> impl FnOnce() -> (State, Cmd<Msg, Sub>) {
    || {
        let state = State {
            peer_id: None,
            room: None,
            peer: peer,
            inputing_room_id: "".into(),
            error_message: None,
            room_id_regex: Regex::new(r"^[A-Za-z0-9-_]{20}$").unwrap(),
        };
        (state, Cmd::none())
    }
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::SetPeerId(peer_id) => {
            state.peer_id = Some(peer_id);
            Cmd::none()
        }
        Msg::SetRoomId(room_id) => {
            state.inputing_room_id = room_id;
            Cmd::none()
        }
        Msg::ConnectToRoom(room_id) => {
            if state.room_id_regex.is_match(&room_id) {
                state.room = Some(Rc::new(Room::new(state.peer.join_room(&room_id), room_id)));
                state.inputing_room_id = "".into();
            } else if room_id.len() > 20 {
                state.error_message = Some("ルームIDの文字数が多すぎます。".into());
            } else if room_id.len() < 20 {
                state.error_message = Some("ルームIDの文字数が少なすぎます。".into());
            } else {
                state.error_message = Some("ルームIDに不正な文字が含まれています。".into());
            }
            Cmd::none()
        }
        Msg::DisconnectFromRoom => {
            if let Some(room) = &mut state.room {
                room.payload.close();
                state.inputing_room_id = room.id.clone();
            }
            state.room = None;
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
            Attributes::new()
                .id("app")
                .class("fullscreen centering-v grid"),
            Events::new(),
            vec![Html::div(
                Attributes::new().class("grid-cc-2x5 frame"),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class("frame-header"),
                        Events::new(),
                        vec![Html::text("Soldoresol")],
                    ),
                    Html::div(
                        Attributes::new().class("frame-body grid"),
                        Events::new(),
                        vec![
                            if let Some(error_message) = &state.error_message {
                                Html::div(
                                    Attributes::new()
                                        .class("grid-w-f container-a text-color-danger-l"),
                                    Events::new(),
                                    vec![Html::text(error_message)],
                                )
                            } else {
                                Html::none()
                            },
                            Html::div(
                                Attributes::new().class("grid-w-10 centering-a pure-form"),
                                Events::new(),
                                vec![
                                    Html::fieldset(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::input(
                                            Attributes::new()
                                                .type_("text")
                                                .value(&state.inputing_room_id)
                                                .class("pure-input-1")
                                                .placeholder("ルームID"),
                                            Events::new().on_input(|s| Msg::SetRoomId(s)),
                                            vec![],
                                        )],
                                    ),
                                    btn::primary(
                                        Attributes::new(),
                                        Events::new().on_click({
                                            let room_id = state.inputing_room_id.clone();
                                            move |_| Msg::ConnectToRoom(room_id)
                                        }),
                                        vec![Html::text("接続")],
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new().class("grid-w-4 centering-a"),
                                Events::new(),
                                vec![Html::text("または")],
                            ),
                            Html::div(
                                Attributes::new().class("grid-w-10 centering-a pure-form"),
                                Events::new(),
                                vec![btn::primary(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let room_id = random_id::base64url();
                                        move |_| Msg::ConnectToRoom(room_id)
                                    }),
                                    vec![Html::text("新規ルームを開く")],
                                )],
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class("frame-footer"),
                        Events::new(),
                        vec![Html::text("開発者twitter：@SoundRabbit_")],
                    ),
                ],
            )],
        )
    }
}
