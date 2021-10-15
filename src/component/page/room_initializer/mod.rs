use super::template::loader::{self, Loader};
use crate::libs::skyway::{MeshRoom, Peer};
use crate::model::config::Config;
use component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;

mod task;

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
        meshroom: Rc<MeshRoom>,
    },
}

pub enum On {
    Load {
        room_db: web_sys::IdbDatabase,
        table_db: web_sys::IdbDatabase,
        meshroom: Rc<MeshRoom>,
    },
}

pub struct RoomInisializer {}

impl Component for RoomInisializer {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomInisializer {
    fn constructor(_: &Props) -> Self {
        Self {}
    }
}

impl Update for RoomInisializer {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        Cmd::task({
            let config = Rc::clone(&props.config);
            let common_db = Rc::clone(&props.common_db);
            let meshroom = props.peer.join_room(&props.room_id);
            let room_id = Rc::clone(&props.room_id);
            move |resolve| {
                wasm_bindgen_futures::spawn_local(async {
                    if let Some((room_db, table_db, meshroom)) =
                        task::initialize(config, common_db, meshroom, room_id).await
                    {
                        resolve(Msg::Initialized {
                            room_db,
                            table_db,
                            meshroom,
                        });
                    }
                })
            }
        })
    }

    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Initialized {
                room_db,
                table_db,
                meshroom,
            } => Cmd::Sub(On::Load {
                room_db,
                table_db,
                meshroom,
            }),
        }
    }
}

impl Render for RoomInisializer {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Loader::empty(loader::Props {}, Sub::none())
    }
}
