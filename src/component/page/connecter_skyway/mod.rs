use super::template::room::{self, Room};
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub room: Rc<MeshRoom>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
}

pub enum Msg {}

pub enum On {}

pub struct ConnecterSkyway {}

impl Component for ConnecterSkyway {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ConnecterSkyway {
    fn constructor(_: &Props) -> Self {
        Self {}
    }
}

impl Update for ConnecterSkyway {}

impl Render for ConnecterSkyway {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Room::empty(room::Props {}, Sub::none())
    }
}
