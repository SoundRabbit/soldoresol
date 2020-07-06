use super::state;
use super::Msg;
use crate::block;
use kagura::prelude::*;

mod area;
mod boxblock;
mod character;
mod default;
mod tablemask;

pub fn render(
    z_index: u64,
    block_field: &block::Field,
    contextmenu: &state::contextmenu::State,
) -> Html<Msg> {
    match &contextmenu as &state::Contextmenu {
        state::Contextmenu::Default => default::render(z_index, contextmenu),
        state::Contextmenu::Area(block_id) => area::render(z_index, contextmenu, block_id),
        state::Contextmenu::Boxblock(block_id) => {
            if let Some(boxblock) = block_field.get::<block::table_object::Boxblock>(block_id) {
                boxblock::render(z_index, contextmenu, block_id, boxblock)
            } else {
                Html::none()
            }
        }
        state::Contextmenu::Character(block_id) => {
            character::render(z_index, contextmenu, block_id)
        }
        state::Contextmenu::Tablemask(block_id) => {
            if let Some(tablemask) = block_field.get::<block::table_object::Tablemask>(block_id) {
                tablemask::render(z_index, contextmenu, block_id, tablemask)
            } else {
                Html::none()
            }
        }
    }
}
