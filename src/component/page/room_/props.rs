use crate::libs::skyway::{MeshRoom, Peer};
use std::rc::Rc;

pub struct Props {
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub meshroom: Rc<MeshRoom>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
}
