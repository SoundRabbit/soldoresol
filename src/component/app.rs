use super::page::{
    initializer::{self, Initializer},
    room::{self, Room},
    room_selector::{self, RoomSelector},
};
use crate::libs::skyway::Peer;
use crate::model::config::Config;
use kagura::prelude::*;
use std::rc::Rc;

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
    config: Option<Rc<Config>>,
    common_db: Option<Rc<web_sys::IdbDatabase>>,
    client_id: Option<Rc<String>>,
    peer: Option<Rc<Peer>>,
    peer_id: Option<Rc<String>>,
    room_id: Option<Rc<String>>,
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
                self.config = Some(Rc::new(config));
                self.common_db = Some(Rc::new(common_db));
                self.client_id = Some(Rc::new(client_id));
                self.peer = Some(Rc::new(peer));
                self.peer_id = Some(Rc::new(peer_id));
                Cmd::none()
            }
            Msg::SetRoomId(room_id) => {
                self.room_id = Some(Rc::new(room_id));
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
                        config: Rc::clone(&config),
                        common_db: Rc::clone(&common_db),
                        peer: Rc::clone(&peer),
                        peer_id: Rc::clone(&peer_id),
                        room_id: Rc::clone(&room_id),
                        client_id: Rc::clone(&client_id),
                    },
                    Subscription::none(),
                )
            } else {
                RoomSelector::empty(
                    room_selector::Props {
                        common_db: Rc::clone(&common_db),
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
