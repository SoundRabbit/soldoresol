use super::state::{self, Modal};
use super::{Msg, State};
use crate::{block, Resource};
use kagura::prelude::*;

mod chat_log;
mod chat_tab_editor;
mod common;
mod dicebot_selecter;
mod personal_setting;
mod resource;
mod select_character_image;
mod select_player_image;
mod select_table_image;
mod sender_character_selecter;
mod table_mask_color_picker;
mod table_setting;

pub fn render(
    z_index: u64,
    block_field: &block::Field,
    resource: &Resource,
    modals: &Vec<Modal>,
    state: &State,
) -> Html {
    let mut children = vec![];
    for modal in modals {
        let child = match modal {
            Modal::ChatLog(block_id) => {
                if let Some(tab) = block_field.get::<block::chat::Tab>(block_id) {
                    chat_log::render(block_field, resource, tab)
                } else {
                    Html::none()
                }
            }
            Modal::ChatTabEditor => {
                if let Some(chat) = block_field.get::<block::Chat>(state.chat().block_id()) {
                    chat_tab_editor::render(block_field, chat)
                } else {
                    Html::none()
                }
            }
            Modal::PersonalSetting => personal_setting::render(resource, state.personal_data()),
            Modal::Resource => resource::render(resource),
            Modal::SelectCharacterImage(block_id) => {
                select_character_image::render(resource, block_id)
            }
            Modal::SelectPlayerImage => select_player_image::render(resource),
            Modal::SelectTableImage(block_id) => select_table_image::render(resource, block_id),
            Modal::SenderCharacterSelecter => {
                if let Some(world) = block_field.get::<block::World>(state.world()) {
                    sender_character_selecter::render(block_field, resource, world, state.chat())
                } else {
                    Html::none()
                }
            }
            Modal::TablemaskColorPicker(block_id) => table_mask_color_picker::render(block_id),
            Modal::TableSetting => {
                if let Some(world) = block_field.get::<block::World>(state.world()) {
                    table_setting::render(block_field, resource, world)
                } else {
                    Html::none()
                }
            }
            Modal::DicebotSelecter => dicebot_selecter::render(state.dicebot()),
        };
        children.push(child);
    }
    Html::div(
        Attributes::new()
            .style("position", "fixied")
            .style("z-index", z_index.to_string()),
        Events::new(),
        children,
    )
}
