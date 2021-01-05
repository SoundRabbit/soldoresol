use super::state::chat;
use super::Msg;
use crate::{
    block::{self, chat::item::Sender},
    Resource,
};
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    world: &block::World,
    chat: &chat::State,
) -> Html {
    common::character_selecter(
        block_field,
        resource,
        world,
        &chat
            .senders()
            .iter()
            .filter_map(|sender| {
                if let Sender::Character(block_id) = sender {
                    Some(block_id.clone())
                } else {
                    None
                }
            })
            .collect(),
        |character_id, is_selected| {
            if is_selected {
                Msg::AddChatSender(character_id)
            } else {
                Msg::RemoveChatSender(character_id)
            }
        },
    )
}
