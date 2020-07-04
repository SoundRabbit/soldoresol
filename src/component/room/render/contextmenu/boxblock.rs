use super::super::super::super::{btn, contextmenu};
use super::state::{self};
use super::Msg;
use crate::block::BlockId;
use kagura::prelude::*;

pub fn render(
    z_index: u64,
    contextmenu: &state::contextmenu::State,
    block_id: &BlockId,
) -> Html<Msg> {
    contextmenu::div(
        z_index,
        || Msg::CloseContextmenu,
        contextmenu.grobal_position(),
        Attributes::new(),
        Events::new(),
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![btn::contextmenu_text(
                Attributes::new(),
                Events::new().on_click({
                    let block_id = block_id.clone();
                    move |_| Msg::RemoveBoxblockToCloseContextmenu(block_id)
                }),
                "削除",
            )],
        )],
    )
}
