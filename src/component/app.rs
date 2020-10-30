use super::batch::peer_connection::{self, PeerConnection};
use super::page::{
    initializer::{self, Initializer},
    room_selector::{self, RoomSelector},
};
use super::util::State;
use crate::skyway::{Peer, Room};
use crate::Config;
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {
    Init(Config, web_sys::IdbDatabase, String),
}

pub enum Sub {}

pub struct App {
    config: Option<State<Config>>,
    common_database: Option<State<web_sys::IdbDatabase>>,
    client_id: Option<State<String>>,
    peer: Option<State<Peer>>,
    peer_id: Option<State<String>>,
}

impl Constructor for App {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        crate::debug::log_1(format!("construct {}", std::any::type_name::<Self>()));

        Self {
            config: None,
            common_database: None,
            client_id: None,
            peer: None,
            peer_id: None,
        }
    }
}

impl Component for App {
    type Props = Props;
    type Msg = Msg;
    type Sub = Sub;

    fn init(&mut self, _: Props, _: &mut ComponentBuilder<Msg, Sub>) {
        crate::debug::log_1(format!("init {}", std::any::type_name::<Self>()));
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg, Sub> {
        match msg {
            Msg::Init(config, common_database, client_id) => {
                self.config = Some(State::new(config));
                self.common_database = Some(State::new(common_database));
                self.client_id = Some(State::new(client_id));
                self.peer = Some(State::new(Peer::new(
                    &self.config.as_ref().unwrap().skyway.key,
                )));
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        if let (Some(config), Some(common_database), Some(client_id), Some(peer)) = (
            &self.config,
            &self.common_database,
            &self.client_id,
            &self.peer,
        ) {
            PeerConnection::with_child(
                peer_connection::Props {
                    peer: peer.as_prop(),
                },
                Subscription::none(),
                RoomSelector::empty(
                    room_selector::Props {
                        common_database: common_database.as_prop(),
                    },
                    Subscription::none(),
                ),
            )
        } else {
            Initializer::empty(
                initializer::Props {},
                Subscription::new(|sub| match sub {
                    initializer::On::Load(config, common_database, client_id) => {
                        Msg::Init(config, common_database, client_id)
                    }
                }),
            )
        }
    }
}

impl App {}
