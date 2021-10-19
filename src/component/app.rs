use super::page::{
    initializer::{self, Initializer},
    // room::{self, Room},
    room_initializer::{self, RoomInisializer},
    room_selector::{self, RoomSelector},
};
use crate::libs::skyway;
use crate::model::config::Config;
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {}

#[derive(Clone)]
pub struct CommonPageData {
    config: Rc<Config>,
    common_db: Rc<web_sys::IdbDatabase>,
    client_id: Rc<String>,
    peer: Rc<skyway::Peer>,
    peer_id: Rc<String>,
}

pub enum Page {
    Initializer {},
    RoomSelector {
        common: CommonPageData,
    },
    RoomInisializer {
        common: CommonPageData,
        room_id: Rc<String>,
    },
    Room {
        common: CommonPageData,
        meshroom: Rc<skyway::MeshRoom>,
        room_id: Rc<String>,
        room_db: Rc<web_sys::IdbDatabase>,
        table_db: Rc<web_sys::IdbDatabase>,
    },
}

pub enum Msg {
    Show(Page),
}

pub enum On {}

pub struct App {
    page: Page,
}

impl Component for App {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for App {
    fn constructor(props: &Props) -> Self {
        Self {
            page: Page::Initializer {},
        }
    }
}

impl Update for App {
    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Show(page) => {
                self.page = page;
                Cmd::none()
            }
        }
    }
}

impl Render for App {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        match &self.page {
            Page::Initializer {} => Initializer::empty(
                initializer::Props {},
                Sub::map(|sub| match sub {
                    initializer::On::Load {
                        config,
                        common_db,
                        client_id,
                        peer,
                        peer_id,
                    } => Msg::Show(Page::RoomSelector {
                        common: CommonPageData {
                            config: Rc::new(config),
                            common_db: Rc::new(common_db),
                            client_id: Rc::new(client_id),
                            peer: Rc::new(peer),
                            peer_id: Rc::new(peer_id),
                        },
                    }),
                }),
            ),
            Page::RoomSelector { common } => RoomSelector::empty(
                room_selector::Props {
                    common_db: Rc::clone(&common.common_db),
                },
                Sub::once({
                    let common = CommonPageData::clone(&common);
                    move |sub| match sub {
                        room_selector::On::Connect(room_id) => Msg::Show(Page::RoomInisializer {
                            common: common,
                            room_id: Rc::new(room_id),
                        }),
                    }
                }),
            ),
            Page::RoomInisializer { common, room_id } => RoomInisializer::empty(
                room_initializer::Props {
                    config: Rc::clone(&common.config),
                    common_db: Rc::clone(&common.common_db),
                    peer: Rc::clone(&common.peer),
                    peer_id: Rc::clone(&common.peer_id),
                    client_id: Rc::clone(&common.client_id),
                    room_id: Rc::clone(&room_id),
                },
                Sub::map({
                    let common = CommonPageData::clone(&common);
                    let room_id = Rc::clone(&room_id);
                    move |sub| match sub {
                        room_initializer::On::Load {
                            room_db,
                            table_db,
                            meshroom,
                        } => Msg::Show(Page::Room {
                            common: CommonPageData::clone(&common),
                            room_id: Rc::clone(&room_id),
                            room_db: Rc::new(room_db),
                            table_db: Rc::new(table_db),
                            meshroom: meshroom,
                        }),
                    }
                }),
            ),
            _ => Html::none()
            // Page::Room {
            //     common,
            //     room_id,
            //     room_db,
            //     table_db,
            //     meshroom,
            // } => Room::empty(
            //     room::Props {
            //         peer: Rc::clone(&common.peer),
            //         peer_id: Rc::clone(&common.peer_id),
            //         meshroom: Rc::clone(&meshroom),
            //         client_id: Rc::clone(&common.client_id),
            //         room_id: Rc::clone(&room_id),
            //     },
            //     Subscription::none(),
            // ),
        }
    }
}
