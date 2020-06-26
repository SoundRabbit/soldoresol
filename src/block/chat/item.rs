use super::{Block, BlockId, Field};

#[derive(Clone)]
pub enum Icon {
    None,
    Resource(u128),
    DefaultUser,
}

#[derive(Clone)]
pub struct Item {
    display_name: String,
    peer_id: String,
    character_id: Option<BlockId>,
    icon: Icon,
    payload: String,
}
