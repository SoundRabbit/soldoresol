use super::template::loader::{self, Loader};
use crate::libs::skyway::{MeshRoom, Peer};
use crate::model::config::Config;
use kagura::prelude::*;
use nusa::prelude::*;
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

pub enum Msg {}

pub enum On {
    Load {
        room_db: web_sys::IdbDatabase,
        table_db: web_sys::IdbDatabase,
        meshroom: Rc<MeshRoom>,
    },
}

pub struct RoomInisializer {
    config: Rc<Config>,
    common_db: Rc<web_sys::IdbDatabase>,
    peer: Rc<Peer>,
    peer_id: Rc<String>,
    room_id: Rc<String>,
    client_id: Rc<String>,
}

impl Component for RoomInisializer {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomInisializer {}

impl Constructor for RoomInisializer {
    fn constructor(props: Self::Props) -> Self {
        Self {
            config: props.config,
            common_db: props.common_db,
            peer: props.peer,
            peer_id: props.peer_id,
            room_id: props.room_id,
            client_id: props.client_id,
        }
    }
}

impl Update for RoomInisializer {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::task({
            let config = Rc::clone(&self.config);
            let common_db = Rc::clone(&self.common_db);
            let meshroom = self.peer.join_room(&self.room_id);
            let room_id = Rc::clone(&self.room_id);
            async {
                if let Some((room_db, table_db, meshroom)) =
                    task::initialize(config, common_db, meshroom, room_id).await
                {
                    Cmd::submit(On::Load {
                        room_db,
                        table_db,
                        meshroom,
                    })
                } else {
                    Cmd::none()
                }
            }
        })
    }
}

impl Render<Html> for RoomInisializer {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Loader::empty(self, None, loader::Props {}, Sub::none())
    }
}
