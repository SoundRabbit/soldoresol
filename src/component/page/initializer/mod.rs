use super::template::loader::{self, Loader};
use crate::libs::skyway::Peer;
use crate::model::config::Config;
use component::{Cmd, Sub};
use kagura::prelude::*;

mod task;

pub struct Props {}

pub enum Msg {
    Initialized(Config, web_sys::IdbDatabase, String, Peer, String),
}

pub enum On {
    Load {
        config: Config,
        common_db: web_sys::IdbDatabase,
        client_id: String,
        peer: Peer,
        peer_id: String,
    },
}

pub struct Initializer {}

impl Component for Initializer {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Initializer {
    fn constructor(_: &Self::Props) -> Self {
        crate::debug::log_1("on_construct");
        Self {}
    }
}

impl Update for Initializer {
    fn on_assemble(&mut self, _: &Self::Props) -> Cmd<Self> {
        crate::debug::log_1("on_assemble");
        Cmd::task(move |resolve| {
            wasm_bindgen_futures::spawn_local(async {
                if let Some((config, common_db, client_id, peer, peer_id)) =
                    task::initialize().await
                {
                    crate::debug::log_1("success to initialize");
                    resolve(Msg::Initialized(
                        config, common_db, client_id, peer, peer_id,
                    ));
                } else {
                    crate::debug::log_1("faild to initialize");
                }
            });
        })
    }

    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Initialized(config, common_db, client_id, peer, peer_id) => Cmd::Sub(On::Load {
                config,
                common_db,
                client_id,
                peer,
                peer_id,
            }),
        }
    }
}

impl Render for Initializer {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Loader::empty(loader::Props {}, Sub::none())
    }
}
