use super::state::chat;
use super::Msg;
use crate::{block, Resource};
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    world: &block::World,
    chat: &chat::State,
) -> Html<Msg> {
    common::character_selecter(
        block_field,
        resource,
        world,
        &chat
            .senders()
            .iter()
            .filter_map(|sender| {
                if let chat::Sender::Character(block_id) = sender {
                    Some(block_id.clone())
                } else {
                    None
                }
            })
            .collect(),
        |_| Msg::NoOp,
    )
}
