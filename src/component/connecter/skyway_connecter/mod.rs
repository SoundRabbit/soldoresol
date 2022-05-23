use super::page::room::{self, Room};
use crate::arena::Arena;
use crate::libs::bcdice::js::DynamicLoader;
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
    pub bcdice_loader: Rc<DynamicLoader>,
}

pub enum Msg {}

pub enum On {}

pub struct SkywayConnecter {
    arena: Arena,
    client_id: Rc<String>,
    bcdice_loader: Rc<DynamicLoader>,
}

impl Component for SkywayConnecter {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for SkywayConnecter {}

impl Constructor for SkywayConnecter {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: Arena::new(),
            client_id: props.client_id,
            bcdice_loader: props.bcdice_loader,
        }
    }
}

impl Update for SkywayConnecter {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.client_id = props.client_id;
        Cmd::none()
    }
}

impl Render<Html> for SkywayConnecter {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Room::empty(
            self,
            None,
            room::Props {
                arena: self.arena.as_mut(),
                client_id: Rc::clone(&self.client_id),
                bcdice_loader: Rc::clone(&self.bcdice_loader),
            },
            Sub::none(),
        )
    }
}
