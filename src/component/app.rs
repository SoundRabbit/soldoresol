use super::page::{
    initializer::{self, Initializer},
    room::{self, Room},
    room_selector::{self, RoomSelector},
};
use super::util::State;
use crate::libs::skyway::Peer;
use crate::model::config::Config;
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {
    Init {
        config: Config,
        common_db: web_sys::IdbDatabase,
        client_id: String,
        peer: Peer,
        peer_id: String,
    },
    SetRoomId(String),
}

pub enum Sub {}

pub struct App {
    config: Option<State<Config>>,
    common_db: Option<State<web_sys::IdbDatabase>>,
    client_id: Option<State<String>>,
    peer: Option<State<Peer>>,
    peer_id: Option<State<String>>,
    room_id: Option<State<String>>,
}

impl Constructor for App {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            config: None,
            common_db: None,
            client_id: None,
            peer: None,
            peer_id: None,
            room_id: None,
        }
    }
}

impl Component for App {
    type Props = Props;
    type Msg = Msg;
    type Sub = Sub;

    fn init(&mut self, _: Props, _: &mut ComponentBuilder<Msg, Sub>) {}

    fn update(&mut self, msg: Msg) -> Cmd<Msg, Sub> {
        match msg {
            Msg::Init {
                config,
                common_db,
                client_id,
                peer,
                peer_id,
            } => {
                self.config = Some(State::new(config));
                self.common_db = Some(State::new(common_db));
                self.client_id = Some(State::new(client_id));
                self.peer = Some(State::new(peer));
                self.peer_id = Some(State::new(peer_id));
                Cmd::none()
            }
            Msg::SetRoomId(room_id) => {
                self.room_id = Some(State::new(room_id));
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        if let (Some(config), Some(common_db), Some(client_id), Some(peer), Some(peer_id)) = (
            &self.config,
            &self.common_db,
            &self.client_id,
            &self.peer,
            &self.peer_id,
        ) {
            if let Some(room_id) = &self.room_id {
                Room::empty(
                    room::Props {
                        config: config.as_prop(),
                        common_db: common_db.as_prop(),
                        peer: peer.as_prop(),
                        peer_id: peer_id.as_prop(),
                        room_id: room_id.as_prop(),
                        client_id: client_id.as_prop(),
                    },
                    Subscription::none(),
                )
            } else {
                RoomSelector::empty(
                    room_selector::Props {
                        common_db: common_db.as_prop(),
                    },
                    Subscription::new(|sub| match sub {
                        room_selector::On::Connect(room_id) => Msg::SetRoomId(room_id),
                    }),
                )
            }
        } else {
            Initializer::empty(
                initializer::Props {},
                Subscription::new(|sub| match sub {
                    initializer::On::Load {
                        config,
                        common_db,
                        client_id,
                        peer,
                        peer_id,
                    } => Msg::Init {
                        config,
                        common_db,
                        client_id,
                        peer,
                        peer_id,
                    },
                }),
            )
        }
    }
}

impl App {}
