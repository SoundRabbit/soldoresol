use super::template::loader::{self, Loader};
use crate::skyway::Peer;
use crate::Config;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

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

pub struct Initializer {
    config: Option<Config>,
    common_database: Option<web_sys::IdbDatabase>,
    client_id: Option<String>,
    peer: Option<Rc<Peer>>,
}

impl Constructor for Initializer {
    fn constructor(_: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        builder.set_cmd(Cmd::task(move |resolve| {
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
        }));

        Self {
            config: None,
            common_database: None,
            client_id: None,
            peer: None,
        }
    }
}

impl Component for Initializer {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Initialized(config, common_db, client_id, peer, peer_id) => Cmd::sub(On::Load {
                config,
                common_db,
                client_id,
                peer,
                peer_id,
            }),
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Loader::empty(loader::Props {}, Subscription::none())
    }
}
