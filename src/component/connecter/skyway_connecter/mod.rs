use super::page::room::{self, Room};
use crate::arena::{block, Arena};
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::prelude::*;
use nusa::prelude::*;
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

pub struct SkywayConnecter {
    arena: Arena,
}

impl Component for SkywayConnecter {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for SkywayConnecter {
    fn constructor(_: &Props) -> Self {
        Self {
            arena: Arena::new(),
        }
    }
}

impl Update for SkywayConnecter {}

impl Render for SkywayConnecter {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Room::empty(
            room::Props {
                arena: self.arena.as_mut(),
                client_id: Rc::clone(&props.client_id),
            },
            Sub::none(),
        )
    }
}
