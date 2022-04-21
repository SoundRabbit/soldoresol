use super::connecter::skyway_connecter::{self, SkywayConnecter};
use super::page::{
    initializer::{self, Initializer},
    room_initializer::{self, RoomInisializer},
    room_selector::{self, RoomSelector},
};
use super::util::router;
use crate::libs::skyway;
use crate::model::config::Config;
use kagura::prelude::*;
use nusa::prelude::*;
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
    type Event = On;
}

impl HtmlComponent for App {}

impl Constructor for App {
    fn constructor(_: Self::Props) -> Self {
        Self {
            common_data: None,
            room_data: None,
        }
    }
}

impl Update for App {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        let a =
            Closure::wrap(Box::new(move |_: web_sys::Event| {}) as Box<dyn FnMut(web_sys::Event)>);
        let _ = web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("popstate", a.as_ref().unchecked_ref());
        a.forget();

        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
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

impl Render<Html> for App {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        router! {
            r"/rooms" => {
                let common_data = unwrap!(self.common_data.as_ref(); self.render_initializer());
                RoomSelector::empty(
                    self,
                    None,
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
                let common_data = unwrap!(self.common_data.as_ref(); self.render_initializer());
                let room_id = Rc::new(String::from(room_id.get(1).unwrap().as_str()));
                let room_data = unwrap!(self.room_data.as_ref(); self.render_room_initializer(&common_data, &room_id));
                SkywayConnecter::empty(
                    self,
                    None,
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
                let common_data = unwrap!(self.common_data.as_ref(); self.render_initializer());
                let room_id = Rc::new(String::from(room_id.get(1).unwrap().as_str()));
                let room_data = unwrap!(self.room_data.as_ref(); self.render_room_initializer(&common_data, &room_id));
                SkywayConnecter::empty(
                    self,
                    None,
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
                self.render_initializer()
            }
        }
    }
}

impl App {
    fn render_initializer(&self) -> Html {
        Initializer::empty(
            self,
            None,
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

    fn render_room_initializer(&self, common: &CommonPageData, room_id: &Rc<String>) -> Html {
        RoomInisializer::empty(
            self,
            None,
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
