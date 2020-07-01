use super::Msg;
use crate::{block::BlockId, Resource};
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(resource: &Resource, block_id: &BlockId) -> Html<Msg> {
    common::select_image(resource, {
        let block_id = block_id.clone();
        move |resource_id| Msg::SetCharacterTextrureToCloseModal(block_id, Some(resource_id))
    })
}
