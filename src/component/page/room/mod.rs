use super::atom::craftboard::{self, Craftboard};
use super::organism::room_modeless::{self, RoomModeless};
use super::organism::tab_modeless_container::{self, TabModelessContainer};
use crate::arena::block;
use crate::arena::player;
use crate::arena::resource;
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::prelude::*;
use std::rc::Rc;

mod constructor;
mod render;
mod update;

pub struct Props {
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub room: Rc<MeshRoom>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
}

pub enum Msg {
    NoOp,
}

pub enum On {}

pub struct Room {
    block_arena: block::Arena,
    local_block_arena: block::Arena,
    player_arena: player::Arena,
    resource_arena: resource::Arena,

    craftboard: PrepackedComponent<Craftboard>,
    modeless_container:
        PrepackedComponent<TabModelessContainer<RoomModeless, room_modeless::TabName>>,
}

impl Component for Room {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}
