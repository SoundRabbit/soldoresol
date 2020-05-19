use super::room;
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
}

pub enum Msg {
    SetPeerId(String),
    ConnectToRoom(String),
}

pub enum Sub {}

fn init(peer: Rc<Peer>) -> impl FnOnce() -> (State, Cmd<Msg, Sub>) {
    || {
        let state = State {
            peer_id: None,
            room: None,
            peer: peer,
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
        Msg::ConnectToRoom(room_id) => {
            web_sys::console::log_1(&JsValue::from("ConnectToRoom"));
            state.room = Some(Rc::new(Room::new(state.peer.join_room(&room_id), room_id)));
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
            Attributes::new().class("app").id("app"),
            Events::new(),
            vec![Html::div(
                Attributes::new().class("app__modal-bg--dark"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("app__modal-bg--centering"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("modal"),
                        Events::new(),
                        vec![
                            Html::h2(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text("ルームに接続")],
                            ),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                        ],
                    )],
                )],
            )],
        )
    }
}
