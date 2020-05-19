use super::btn;
use super::room;
use crate::random_id;
use crate::skyway::{Peer, Room};
use crate::Config;
use kagura::prelude::*;
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
}

pub enum Msg {
    SetPeerId(String),
    SetRoomId(String),
    ConnectToRoom(String),
}

pub enum Sub {}

fn init(peer: Rc<Peer>) -> impl FnOnce() -> (State, Cmd<Msg, Sub>) {
    || {
        let state = State {
            peer_id: None,
            room: None,
            peer: peer,
            inputing_room_id: "".into(),
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
            if room_id != "" {
                state.room = Some(Rc::new(Room::new(state.peer.join_room(&room_id), room_id)));
                state.inputing_room_id = "".into();
            }
            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    if let Some(room) = &state.room {
        Html::component(room::new(Rc::clone(room)).subscribe(|sub| match sub {
            room::Sub::ConnectToRoom(room_id) => Msg::ConnectToRoom(room_id),
        }))
    } else {
        Html::div(
            Attributes::new()
                .id("app")
                .class("fullscreen centering-v grid"),
            Events::new(),
            vec![
                Html::div(Attributes::new().class("grid-w-7"), Events::new(), vec![]),
                Html::div(
                    Attributes::new().class("grid-w-10 frame"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class("frame-header"),
                            Events::new(),
                            vec![Html::text("ルームに接続していません。")],
                        ),
                        Html::div(
                            Attributes::new().class("frame-body grid"),
                            Events::new(),
                            vec![
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
                                            let room_id = random_id::base64();
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
                            vec![Html::text("※ルームIDは20文字の英数字と記号です。")],
                        ),
                    ],
                ),
            ],
        )
    }
}
