use super::atom::craftboard::{self, Craftboard};
use super::organism::room_modeless::{self, RoomModeless};
use super::organism::tab_modeless_container::{self, TabModelessContainer};
use crate::arena::{block, Arena, ArenaMut};
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::prelude::*;
use std::rc::Rc;

mod constructor;
mod render;
mod update;

pub struct Props {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
}

pub enum Msg {
    NoOp,
}

pub enum On {}

pub struct Room {
    arena: ArenaMut,
    local_arena: Arena,

    craftboard: PrepackedComponent<Craftboard>,
    modeless_container:
        PrepackedComponent<TabModelessContainer<RoomModeless, room_modeless::TabName>>,
}

impl Component for Room {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}
