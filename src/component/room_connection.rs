use super::room;
use crate::skyway::{Peer, Room};
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

enum RoomConnection {
    UnOpened(Rc<Room>),
    Opened(Rc<Room>),
}

pub struct State {
    peer: Rc<Peer>,
    room: RoomConnection,
}

pub enum Msg {
    SetConnectionAsOpened,
    DisconnectFromRoom,
}

pub enum Sub {
    DisconnectFromRoom,
}

pub fn new(peer: Rc<Peer>, room: Rc<Room>) -> Component<Msg, State, Sub> {
    let init = {
        let peer = Rc::clone(&peer);
        let room = Rc::clone(&room);
        move || {
            let state = State {
                peer,
                room: RoomConnection::UnOpened(room),
            };
            (state, Cmd::none())
        }
    };
    Component::new(init, update, render).batch({
        let room = Rc::clone(&room);
        move |mut handler| {
            let a = Closure::wrap(
                Box::new(move || handler(Msg::SetConnectionAsOpened)) as Box<dyn FnMut()>
            );
            room.payload.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    })
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::SetConnectionAsOpened => {
            if let RoomConnection::UnOpened(room) = &state.room {
                state.room = RoomConnection::Opened(Rc::clone(room));
            }
            Cmd::none()
        }
        Msg::DisconnectFromRoom => Cmd::sub(Sub::DisconnectFromRoom),
    }
}

fn render(state: &State) -> Html<Msg> {
    if let RoomConnection::Opened(room) = &state.room {
        Html::component(
            room::new(Rc::clone(&state.peer), Rc::clone(room)).subscribe(|sub| match sub {
                room::Sub::DisconnectFromRoom => Msg::DisconnectFromRoom,
            }),
        )
    } else {
        Html::div(
            Attributes::new().id("app").class("fullscreen"),
            Events::new(),
            vec![],
        )
    }
}
