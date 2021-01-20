use super::template::loader::{self, Loader};
use super::util::{Prop, State};
use crate::libs::skyway::{MeshRoom, Peer};
use crate::model::config::Config;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod children;
mod implement;
mod model;
mod task;

use implement::Implement;

pub struct Props {
    pub config: Rc<Config>,
    pub common_db: Rc<web_sys::IdbDatabase>,
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
}

pub enum Msg {
    Initialized {
        room_db: web_sys::IdbDatabase,
        table_db: web_sys::IdbDatabase,
        room: Rc<MeshRoom>,
    },
}

pub enum On {}

pub struct Room {
    config: Rc<Config>,
    common_db: Rc<web_sys::IdbDatabase>,
    room_db: Option<Rc<web_sys::IdbDatabase>>,
    table_db: Option<Rc<web_sys::IdbDatabase>>,
    peer: Rc<Peer>,
    peer_id: Rc<String>,
    room: Option<Rc<MeshRoom>>,
    room_id: Rc<String>,
    client_id: Rc<String>,
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
            client_id: props.client_id,
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
                self.room_db = Some(Rc::new(room_db));
                self.table_db = Some(Rc::new(table_db));
                self.room = Some(room);
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        if let Some(room) = &self.room {
            Implement::empty(
                implement::Props {
                    peer: Rc::clone(&self.peer),
                    peer_id: Rc::clone(&self.peer_id),
                    room: Rc::clone(&room),
                    room_id: Rc::clone(&self.room_id),
                    client_id: Rc::clone(&self.client_id),
                },
                Subscription::none(),
            )
        } else {
            Loader::empty(loader::Props {}, Subscription::none())
        }
    }
}
