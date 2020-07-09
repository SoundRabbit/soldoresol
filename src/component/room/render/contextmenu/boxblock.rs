use super::super::super::super::{btn, contextmenu};
use super::super::state::Modeless;
use super::state::{self};
use super::Msg;
use crate::block::{self, BlockId};
use kagura::prelude::*;

pub fn render(
    z_index: u64,
    contextmenu: &state::contextmenu::State,
    block_id: &BlockId,
    boxblock: &block::table_object::Boxblock,
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
            vec![
                btn::contextmenu_text_window(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| {
                            Msg::OpenModeless(Modeless::Object {
                                tabs: vec![block_id],
                                focused: 0,
                                outlined: None,
                            })
                        }
                    }),
                    "編集",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        let is_fixed = boxblock.is_fixed();
                        move |_| Msg::SetBoxblockIsFixed(block_id, !is_fixed)
                    }),
                    String::from("固定")
                        + if boxblock.is_fixed() {
                            "解除"
                        } else {
                            "する"
                        },
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| Msg::RemoveBoxblockToCloseContextmenu(block_id)
                    }),
                    "削除",
                ),
            ],
        )],
    )
}
