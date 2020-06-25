use super::Block;
use super::BlockId;

pub enum Icon {
    None,
    Resource(u128),
    DefaultUser,
}

pub struct Item {
    display_name: String,
    peer_id: String,
    character_id: Option<u128>,
    icon: Icon,
    payload: String,
}
