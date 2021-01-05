use super::room;
use crate::{
    config::Config,
    idb,
    skyway::{self, Peer},
};
use kagura::prelude::*;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub type RoomConnection = Component<Props, Sub>;

pub struct Props {
    pub config: Rc<Config>,
    pub peer: Rc<Peer>,
    pub room: Rc<skyway::Room>,
    pub client_id: Rc<String>,
    pub common_database: Rc<web_sys::IdbDatabase>,
}

enum Room {
    UnOpened(Rc<skyway::Room>),
    Opened(Rc<skyway::Room>),
}

impl Deref for Room {
    type Target = Rc<skyway::Room>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Opened(x) => &x,
            Self::UnOpened(x) => &x,
        }
    }
}

pub struct State {
    peer: Rc<Peer>,
    room: Room,
    config: Rc<Config>,
    client_id: Rc<String>,
    common_database: Rc<web_sys::IdbDatabase>,
    room_database: Option<Rc<web_sys::IdbDatabase>>,
    table_database: Option<Rc<web_sys::IdbDatabase>>,
    room_component: room::Room,
}

pub enum Msg {
    TryToSetRoomDatabase(Rc<web_sys::IdbDatabase>),
    GetTableDatabase,
    SetTableDatabase(Rc<web_sys::IdbDatabase>),
    SetConnectionAsOpened,
    DisconnectFromRoom,
}

pub enum Sub {
    DisconnectFromRoom,
}

pub fn new() -> RoomConnection {
    Component::new(init, update, render)
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let room_db_name = String::from("") + &props.config.client.db_prefix + ".room";

    let state = State {
        peer: props.peer,
        room: Room::UnOpened(Rc::clone(&props.room)),
        client_id: props.client_id,
        config: props.config,
        room_database: None,
        table_database: None,
        common_database: props.common_database,
        room_component: room::new(),
    };

    let task = Cmd::task(move |resolve| {
        idb::open_db(&room_db_name).then(move |database| {
            if let Some(database) = database {
                resolve(Msg::TryToSetRoomDatabase(Rc::new(database)))
            }
        })
    });

    let batch = vec![Batch::new({
        let room = Rc::clone(&props.room);
        move |mut handler| {
            let a = Closure::wrap(
                Box::new(move || handler(Msg::SetConnectionAsOpened)) as Box<dyn FnMut()>
            );
            room.payload.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    })];

    (state, task, batch)
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
                let promise = idb::create_object_strage(&room_database, state.room.id.as_str());
                Cmd::task(move |resolve| {
                    promise.then(move |database| {
                        if let Some(database) = database {
                            resolve(Msg::TryToSetRoomDatabase(Rc::new(database)))
                        }
                    })
                })
            } else {
                state.room_database = Some(room_database);
                update(state, Msg::GetTableDatabase)
            }
        }
        Msg::GetTableDatabase => {
            crate::debug::log_1("GetTableDatabase");
            let table_db_name = String::from("") + &state.config.client.db_prefix + ".table";
            Cmd::task(move |resolve| {
                idb::open_db(&table_db_name).then(move |database| {
                    if let Some(database) = database {
                        resolve(Msg::SetTableDatabase(Rc::new(database)))
                    }
                })
            })
        }
        Msg::SetTableDatabase(table_database) => {
            crate::debug::log_1("SetTableDatabase");
            state.table_database = Some(table_database);
            Cmd::none()
        }
        Msg::SetConnectionAsOpened => {
            if let Room::UnOpened(room) = &state.room {
                state.room = Room::Opened(Rc::clone(room));
            }
            Cmd::none()
        }
        Msg::DisconnectFromRoom => Cmd::sub(Sub::DisconnectFromRoom),
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    if let (Room::Opened(room), Some(room_database), Some(table_database)) =
        (&state.room, &state.room_database, &state.table_database)
    {
        Html::component(
            state
                .room_component
                .with(room::Props {
                    peer: Rc::clone(&state.peer),
                    room: Rc::clone(room),
                    client_id: Rc::clone(&state.client_id),
                    common_database: Rc::clone(&state.common_database),
                    room_database: Rc::clone(room_database),
                    table_database: Rc::clone(table_database),
                })
                .subscribe(|sub| match sub {
                    room::Sub::DisconnectFromRoom => Msg::DisconnectFromRoom,
                    room::Sub::UpdateTableDatabase(table_db) => Msg::SetTableDatabase(table_db),
                }),
            vec![],
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
