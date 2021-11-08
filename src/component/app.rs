use super::connecter::skyway_connecter::{self, SkywayConnecter};
use super::page::{
    initializer::{self, Initializer},
    room_initializer::{self, RoomInisializer},
    room_selector::{self, RoomSelector},
};
use super::util::router;
use crate::libs::skyway;
use crate::model::config::Config;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {}

#[derive(Clone)]
pub struct CommonPageData {
    config: Rc<Config>,
    common_db: Rc<web_sys::IdbDatabase>,
    client_id: Rc<String>,
    peer: Rc<skyway::Peer>,
    peer_id: Rc<String>,
}

#[derive(Clone)]
pub struct RoomPageData {
    meshroom: Rc<skyway::MeshRoom>,
    room_db: Rc<web_sys::IdbDatabase>,
    table_db: Rc<web_sys::IdbDatabase>,
}

pub enum Msg {
    NoOp,
    SetCommonData(CommonPageData),
    SetRoomData(RoomPageData),
}

pub enum On {}

pub struct App {
    common_data: Option<CommonPageData>,
    room_data: Option<RoomPageData>,
}

impl Component for App {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for App {
    fn constructor(props: &Props) -> Self {
        Self {
            common_data: None,
            room_data: None,
        }
    }
}

impl Update for App {
    fn on_assemble(&mut self, _: &Props) -> Cmd<Self> {
        Cmd::batch(|mut handle| {
            let a = Closure::wrap(Box::new(move |_: web_sys::Event| handle(Msg::NoOp))
                as Box<dyn FnMut(web_sys::Event)>);
            let _ = web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("popstate", a.as_ref().unchecked_ref());
            a.forget();
        })
    }

    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetCommonData(data) => {
                self.common_data = Some(data);
                Cmd::none()
            }
            Msg::SetRoomData(data) => {
                self.room_data = Some(data);
                Cmd::none()
            }
        }
    }
}

impl Render for App {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        router! {
            r"/rooms" => {
                let common_data = unwrap_or!(self.common_data.as_ref(); Self::render_initializer());
                RoomSelector::empty(
                    room_selector::Props {
                        common_db: Rc::clone(&common_data.common_db),
                    },
                    Sub::map(move |sub| match sub {
                        room_selector::On::Connect(room_id) => {
                            router::jump_to(format!("/rooms/skyway/{}", room_id).as_str());
                            Msg::NoOp
                        }
                    })
                )
            },
            r"/rooms/skyway/([A-Za-z0-9@#]{24})" (room_id) => {
                let common_data = unwrap_or!(self.common_data.as_ref(); Self::render_initializer());
                let room_id = Rc::new(String::from(room_id.get(1).unwrap().as_str()));
                let room_data = unwrap_or!(self.room_data.as_ref(); Self::render_room_initializer(&common_data, &room_id));
                SkywayConnecter::empty(
                    skyway_connecter:: Props {
                        peer: Rc::clone(&common_data.peer),
                        peer_id: Rc::clone(&common_data.peer_id),
                        room: Rc::clone(&room_data.meshroom),
                        room_id: room_id,
                        client_id: Rc::clone(&common_data.client_id)
                    },
                    Sub::none()
                )
            },
            r"/rooms/drive/([A-Za-z\-_]+)" (room_id) => {
                let common_data = unwrap_or!(self.common_data.as_ref(); Self::render_initializer());
                let room_id = Rc::new(String::from(room_id.get(1).unwrap().as_str()));
                let room_data = unwrap_or!(self.room_data.as_ref(); Self::render_room_initializer(&common_data, &room_id));
                SkywayConnecter::empty(
                    skyway_connecter:: Props {
                        peer: Rc::clone(&common_data.peer),
                        peer_id: Rc::clone(&common_data.peer_id),
                        room: Rc::clone(&room_data.meshroom),
                        room_id: room_id,
                        client_id: Rc::clone(&common_data.client_id)
                    },
                    Sub::none()
                )
            },
            _ => {
                router::jump_to("/rooms");
                Self::render_initializer()
            }
        }
    }
}

impl App {
    fn render_initializer() -> Html<Self> {
        Initializer::empty(
            initializer::Props {},
            Sub::map(|sub| match sub {
                initializer::On::Load {
                    config,
                    common_db,
                    client_id,
                    peer,
                    peer_id,
                } => Msg::SetCommonData(CommonPageData {
                    config: Rc::new(config),
                    common_db: Rc::new(common_db),
                    client_id: Rc::new(client_id),
                    peer: Rc::new(peer),
                    peer_id: Rc::new(peer_id),
                }),
            }),
        )
    }

    fn render_room_initializer(common: &CommonPageData, room_id: &Rc<String>) -> Html<Self> {
        RoomInisializer::empty(
            room_initializer::Props {
                config: Rc::clone(&common.config),
                common_db: Rc::clone(&common.common_db),
                peer: Rc::clone(&common.peer),
                peer_id: Rc::clone(&common.peer_id),
                client_id: Rc::clone(&common.client_id),
                room_id: Rc::clone(&room_id),
            },
            Sub::map(move |sub| match sub {
                room_initializer::On::Load {
                    room_db,
                    table_db,
                    meshroom,
                } => Msg::SetRoomData(RoomPageData {
                    meshroom,
                    room_db: Rc::new(room_db),
                    table_db: Rc::new(table_db),
                }),
            }),
        )
    }
}

impl Styled for App {
    fn style() -> Style {
        style! {}
    }
}
