use super::template::loader::{self, Loader};
use crate::libs::skyway::Peer;
use crate::model::config::Config;
use kagura::prelude::*;
use nusa::prelude::*;

mod task;

pub struct Props {}

pub enum Msg {}

pub enum On {
    Load {
        config: Config,
        common_db: web_sys::IdbDatabase,
        room_db: web_sys::IdbDatabase,
        client_id: String,
        peer: Peer,
        peer_id: String,
    },
}

pub struct Initializer {}

impl Component for Initializer {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Initializer {}

impl Constructor for Initializer {
    fn constructor(_: Self::Props) -> Self {
        Self {}
    }
}

impl Update for Initializer {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::task(async {
            if let Some((config, common_db, room_db, client_id, peer, peer_id)) =
                task::initialize().await
            {
                Cmd::submit(On::Load {
                    config,
                    common_db,
                    room_db,
                    client_id,
                    peer,
                    peer_id,
                })
            } else {
                Cmd::none()
            }
        })
    }
}

impl Render<Html> for Initializer {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Loader::empty(self, None, loader::Props {}, Sub::none())
    }
}
