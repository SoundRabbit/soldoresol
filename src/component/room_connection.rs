use super::room;
use crate::{
    config::Config,
    idb,
    skyway::{Peer, Room},
};
use kagura::prelude::*;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

enum RoomConnection {
    UnOpened(Rc<Room>),
    Opened(Rc<Room>),
}

impl Deref for RoomConnection {
    type Target = Rc<Room>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Opened(x) => &x,
            Self::UnOpened(x) => &x,
        }
    }
}

pub struct State {
    peer: Rc<Peer>,
    room: RoomConnection,
    config: Rc<Config>,
    common_database: Rc<web_sys::IdbDatabase>,
    room_database: Option<Rc<web_sys::IdbDatabase>>,
}

pub enum Msg {
    TryToSetRoomDatabase(Rc<web_sys::IdbDatabase>),
    SetConnectionAsOpened,
    DisconnectFromRoom,
}

pub enum Sub {
    DisconnectFromRoom,
}

pub fn new(
    config: Rc<Config>,
    peer: Rc<Peer>,
    room: Rc<Room>,
    common_database: Rc<web_sys::IdbDatabase>,
) -> Component<Msg, State, Sub> {
    let init = {
        let peer = Rc::clone(&peer);
        let room = Rc::clone(&room);
        move || {
            let room_db_name = String::from("") + &config.client.db_prefix + ".room";

            let state = State {
                peer,
                room: RoomConnection::UnOpened(room),
                config: config,
                room_database: None,
                common_database: common_database,
            };
            let task = idb::open_db(&room_db_name, |database| {
                Msg::TryToSetRoomDatabase(Rc::new(database))
            });
            (state, task)
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
        Msg::TryToSetRoomDatabase(room_database) => {
            let names = room_database.object_store_names();
            let mut has_room = false;
            for i in 0..names.length() {
                if let Some(name) = names.item(i) {
                    if name == state.room.id.as_str() {
                        has_room = true;
                    }
                }
            }
            if !has_room {
                idb::create_object_strage(&room_database, state.room.id.as_str(), |database| {
                    Msg::TryToSetRoomDatabase(Rc::new(database))
                })
            } else {
                state.room_database = Some(room_database);
                Cmd::none()
            }
        }
        Msg::SetConnectionAsOpened => {
            if let RoomConnection::UnOpened(room) = &state.room {
                state.room = RoomConnection::Opened(Rc::clone(room));
            }
            Cmd::none()
        }
        Msg::DisconnectFromRoom => Cmd::sub(Sub::DisconnectFromRoom),
    }
}

fn render(state: &State) -> Html {
    if let (RoomConnection::Opened(room), Some(room_database)) = (&state.room, &state.room_database)
    {
        Html::component(
            room::new(
                Rc::clone(&state.peer),
                Rc::clone(room),
                Rc::clone(&state.common_database),
                Rc::clone(room_database),
            )
            .subscribe(|sub| match sub {
                room::Sub::DisconnectFromRoom => Msg::DisconnectFromRoom,
            }),
        )
    } else {
        Html::div(
            Attributes::new()
                .id("app")
                .class("centering")
                .class("fullscreen")
                .class("centering-a"),
            Events::new(),
            vec![Html::text("Loading...")],
        )
    }
}
