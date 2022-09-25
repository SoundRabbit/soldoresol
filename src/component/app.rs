use super::connecter::skyway_connecter::{self, SkywayConnecter};
use super::page::{
    initializer::{self, Initializer},
    room_initializer::{self, RoomInisializer},
    room_selector::{self, RoomSelector},
};
use super::util::router;
use crate::libs::bcdice::js::DynamicLoader;
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
    room_db: Rc<web_sys::IdbDatabase>,
    client_id: Rc<String>,
    peer: Rc<skyway::Peer>,
    peer_id: Rc<String>,
}

#[derive(Clone)]
pub struct RoomPageData {
    meshroom: Rc<skyway::MeshRoom>,
    table_db: Rc<web_sys::IdbDatabase>,
    bcdice_loader: Rc<DynamicLoader>,
}

pub enum Msg {
    NoOp,
    PopState,
    SetRoomDb(Rc<web_sys::IdbDatabase>),
    SetCommonData(CommonPageData),
    SetRoomData(Rc<web_sys::IdbDatabase>, RoomPageData),
}

pub enum On {}

pub struct App {
    common_data: Option<CommonPageData>,
    room_data: Option<RoomPageData>,
    popstate_listener: kagura::util::Batch<Cmd<Self>>,
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
            popstate_listener: kagura::util::Batch::new(|mut resolve| {
                let a = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    resolve(Cmd::chain(Msg::PopState))
                }) as Box<dyn FnMut(web_sys::Event)>);
                let _ = web_sys::window()
                    .unwrap()
                    .add_event_listener_with_callback("popstate", a.as_ref().unchecked_ref());
                a.forget();
            }),
        }
    }
}

impl Update for App {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::task(self.popstate_listener.poll())
    }

    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::PopState => Cmd::task(self.popstate_listener.poll()),
            Msg::SetRoomDb(room_db) => {
                if let Some(common_data) = self.common_data.as_mut() {
                    common_data.room_db = room_db;
                }
                Cmd::none()
            }
            Msg::SetCommonData(data) => {
                self.common_data = Some(data);
                Cmd::none()
            }
            Msg::SetRoomData(room_db, data) => {
                if let Some(common_data) = self.common_data.as_mut() {
                    common_data.room_db = room_db;
                }
                self.room_data = Some(data);
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for App {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        let prefix = if crate::is_dev_mode() {
            ""
        } else {
            "/soldoresol-dev"
        };

        router! {
            (format!(r"{}/rooms", prefix)) => {
                let common_data = unwrap!(self.common_data.as_ref(); self.render_initializer());
                RoomSelector::empty(
                    self,
                    None,
                    room_selector::Props {
                        common_db: Rc::clone(&common_data.common_db),
                        room_db: Rc::clone(&common_data.room_db),
                    },
                    Sub::map(move |sub| match sub {
                        room_selector::On::Connect(annot_room_id) => {
                            router::jump_to(format!("{}/rooms/{}", prefix, annot_room_id).as_str());
                            Msg::NoOp
                        }
                        room_selector::On::SetRoomDb(room_db) => Msg::SetRoomDb(room_db),
                    })
                )
            },
            (format!(r"{}/rooms/skyway/([A-Za-z0-9@#]{{24}})", prefix)) (room_id) => {
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
                        client_id: Rc::clone(&common_data.client_id),
                        bcdice_loader: Rc::clone(&room_data.bcdice_loader)
                    },
                    Sub::none()
                )
            },
            (format!(r"{}/rooms/drive/([A-Za-z\-_]+)", prefix)) (room_id) => {
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
                        client_id: Rc::clone(&common_data.client_id),
                        bcdice_loader: Rc::clone(&room_data.bcdice_loader)
                    },
                    Sub::none()
                )
            },
            _ => {
                router::jump_to(format!(r"{}/rooms", prefix).as_str());
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
                    room_db,
                    client_id,
                    peer,
                    peer_id,
                } => Msg::SetCommonData(CommonPageData {
                    config: Rc::new(config),
                    common_db: Rc::new(common_db),
                    room_db: Rc::new(room_db),
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
                room_db: Rc::clone(&common.room_db),
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
                } => Msg::SetRoomData(
                    room_db,
                    RoomPageData {
                        meshroom,
                        table_db: Rc::new(table_db),
                        bcdice_loader: Rc::new(DynamicLoader::new()),
                    },
                ),
            }),
        )
    }
}
