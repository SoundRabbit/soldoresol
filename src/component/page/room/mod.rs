use super::template::loader::{self, Loader};
use super::util::{Prop, State};
use crate::libs::skyway::{MeshRoom, Peer};
use crate::model::config::Config;
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

mod children;
mod implement;
mod model;
mod task;

use implement::Implement;

pub struct Props {
    pub config: Prop<Config>,
    pub common_db: Prop<web_sys::IdbDatabase>,
    pub peer: Prop<Peer>,
    pub peer_id: Prop<String>,
    pub room_id: Prop<String>,
}

pub enum Msg {
    Initialized {
        room_db: web_sys::IdbDatabase,
        table_db: web_sys::IdbDatabase,
        room: State<MeshRoom>,
    },
}

pub enum On {}

pub struct Room {
    config: Prop<Config>,
    common_db: Prop<web_sys::IdbDatabase>,
    room_db: Option<State<web_sys::IdbDatabase>>,
    table_db: Option<State<web_sys::IdbDatabase>>,
    peer: Prop<Peer>,
    peer_id: Prop<String>,
    room: Option<State<MeshRoom>>,
    room_id: Prop<String>,
}

impl Constructor for Room {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        builder.set_cmd(Cmd::task({
            let config = props.config.clone();
            let common_db = props.common_db.clone();
            let room = props.peer.join_room(&props.room_id);
            let room_id = props.room_id.clone();
            move |resolve| {
                wasm_bindgen_futures::spawn_local(async {
                    if let Some((room_db, table_db, room)) =
                        task::initialize(config, common_db, room, room_id).await
                    {
                        resolve(Msg::Initialized {
                            room_db,
                            table_db,
                            room,
                        });
                    }
                })
            }
        }));

        Self {
            config: props.config,
            common_db: props.common_db,
            room_db: None,
            table_db: None,
            peer: props.peer,
            peer_id: props.peer_id,
            room: None,
            room_id: props.room_id,
        }
    }
}

impl Component for Room {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Initialized {
                room_db,
                table_db,
                room,
            } => {
                self.room_db = Some(State::new(room_db));
                self.table_db = Some(State::new(table_db));
                self.room = Some(room);
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        if let Some(room) = &self.room {
            Implement::empty(
                implement::Props {
                    peer: self.peer.clone(),
                    peer_id: self.peer_id.clone(),
                    room: room.as_prop(),
                    room_id: self.room_id.clone(),
                },
                Subscription::none(),
            )
        } else {
            Loader::empty(loader::Props {}, Subscription::none())
        }
    }
}
