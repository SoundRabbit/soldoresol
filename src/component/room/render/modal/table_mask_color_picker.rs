use super::Msg;
use crate::block::BlockId;
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(block_id: &BlockId) -> Html<Msg> {
    common::color_picker({
        let block_id = block_id.clone();
        move |resource_id| Msg::NoOp
    })
}
