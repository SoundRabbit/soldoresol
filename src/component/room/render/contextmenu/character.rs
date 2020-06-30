use super::super::super::super::{btn, contextmenu};
use super::state::{self, Modeless};
use super::Msg;
use crate::block::BlockId;
use kagura::prelude::*;

pub fn render(contextmenu: &state::contextmenu::State, block_id: &BlockId) -> Html<Msg> {
    contextmenu::div(
        || Msg::CloseContextmenu,
        contextmenu.grobal_position(),
        Attributes::new(),
        Events::new(),
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![
                btn::contextmenu_text(
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
                        move |_| Msg::CloneCharacterToCloseContextmenu(block_id)
                    }),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| Msg::RemoveCharacterToCloseContextmenu(block_id)
                    }),
                    "削除",
                ),
            ],
        )],
    )
}
