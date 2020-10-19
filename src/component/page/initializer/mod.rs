use super::super::template::{loader, Loader};
use crate::Config;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;

mod task;

pub struct Props {}

pub enum Msg {
    SetConfig(Config),
    SetCommonDatabase(Config, web_sys::IdbDatabase, String),
}

pub enum On {
    Load(Config, web_sys::IdbDatabase, String),
}

pub struct Initializer {}

impl Constructor for Initializer {
    fn constructor(_: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        builder.set_cmd(Cmd::task(move |resolve| {
            wasm_bindgen_futures::spawn_local(async {
                if let Some(config) = task::load_config().await {
                    crate::debug::log_1("success to load config");
                    resolve(Msg::SetConfig(config));
                } else {
                    crate::debug::log_1("faild to load config");
                }
            });
        }));

        Self {}
    }
}

impl Component for Initializer {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetConfig(config) => {
                let db_name = format!("{}.common", config.client.db_prefix);
                Cmd::task(move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some((common_database, client_id)) =
                            task::initialize_common_db(&db_name).await
                        {
                            crate::debug::log_1("success to initialize common_db");
                            resolve(Msg::SetCommonDatabase(config, common_database, client_id))
                        }
                    });
                })
            }
            Msg::SetCommonDatabase(config, common_database, client_id) => {
                Cmd::Sub(On::Load(config, common_database, client_id))
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Loader::view(loader::Props {}, SubMap::empty(), vec![])
    }
}
