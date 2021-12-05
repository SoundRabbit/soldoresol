use super::*;

impl Room {
    pub(super) fn render_contextmenu(&self, contextmenu: &ShowingContextmenu) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("contextmenu-mask")),
            Events::new()
                .on_click(|e| Msg::SetShowingContextmenu(None))
                .on_contextmenu(Msg::OnTableContextmenu),
            vec![Html::div(
                Attributes::new()
                    .class(Self::class("contextmenu"))
                    .style("left", format!("{}px", contextmenu.page_x))
                    .style("top", format!("{}px", contextmenu.page_y)),
                Events::new(),
                match &contextmenu.data {
                    ShowingContextmenuData::Boxblock(block) => {
                        self.render_contextmenu_boxblock(block)
                    }
                    ShowingContextmenuData::Character(block) => {
                        self.render_contextmenu_character(block)
                    }
                },
            )],
        )
    }

    fn render_contextmenu_boxblock(&self, boxblock: &BlockMut<block::Boxblock>) -> Vec<Html<Self>> {
        vec![Btn::menu(
            Attributes::new(),
            Events::new().on_click({
                let block_id = boxblock.id();
                move |_| Msg::OpenBoxblockModeless(block_id)
            }),
            vec![Html::text("詳細を表示")],
        )]
    }

    fn render_contextmenu_character(
        &self,
        character: &BlockMut<block::Character>,
    ) -> Vec<Html<Self>> {
        vec![Btn::menu(
            Attributes::new(),
            Events::new().on_click({
                let block_id = character.id();
                move |_| Msg::OpenCharacterModeless(block_id)
            }),
            vec![Html::text("詳細を表示")],
        )]
    }
}
