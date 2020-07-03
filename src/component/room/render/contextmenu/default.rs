use super::super::super::super::{btn, contextmenu};
use super::state;
use super::Msg;
use kagura::prelude::*;

pub fn render(z_index: u64, contextmenu: &state::contextmenu::State) -> Html<Msg> {
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
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let [x, y] = contextmenu.canvas_position();
                        let position = [*x as f32, *y as f32];
                        move |_| Msg::AddChracaterWithMousePositionToCloseContextmenu(position)
                    }),
                    "キャラクターを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let [x, y] = contextmenu.canvas_position();
                        let position = [*x as f32, *y as f32];
                        move |_| Msg::AddTablemaskWithMousePositionToCloseContextmenu(position)
                    }),
                    "マップマスクを作成",
                ),
            ],
        )],
    )
}
