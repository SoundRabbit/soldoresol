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
                    ShowingContextmenuData::Craftboard(block) => {
                        self.render_contextmenu_craftboard(block)
                    }
                },
            )],
        )
    }

    fn render_contextmenu_boxblock(&self, boxblock: &BlockMut<block::Boxblock>) -> Vec<Html<Self>> {
        vec![
            Marker::light(
                Attributes::new(),
                Events::new(),
                vec![Html::text(
                    boxblock
                        .map(|boxblock| boxblock.name().clone())
                        .unwrap_or(String::from("")),
                )],
            ),
            Btn::menu(
                Attributes::new(),
                Events::new().on_click({
                    let block_id = boxblock.id();
                    move |_| Msg::OpenBoxblockModeless(block_id)
                }),
                vec![Html::text("詳細を表示")],
            ),
            Self::render_is_fixed_position(
                boxblock
                    .map(|boxblock| boxblock.is_fixed_position())
                    .unwrap_or(false),
                BlockMut::clone(&boxblock).untyped(),
            ),
            Self::render_is_bind_to_grid(
                boxblock
                    .map(|boxblock| boxblock.is_bind_to_grid())
                    .unwrap_or(false),
                BlockMut::clone(&boxblock).untyped(),
            ),
        ]
    }

    fn render_contextmenu_character(
        &self,
        character: &BlockMut<block::Character>,
    ) -> Vec<Html<Self>> {
        vec![
            Marker::light(
                Attributes::new(),
                Events::new(),
                vec![Html::text(
                    character
                        .map(|character| character.name().clone())
                        .unwrap_or(String::from("")),
                )],
            ),
            Btn::menu(
                Attributes::new(),
                Events::new().on_click({
                    let block_id = character.id();
                    move |_| Msg::OpenCharacterModeless(block_id)
                }),
                vec![Html::text("詳細を表示")],
            ),
            Btn::menu(
                Attributes::new(),
                Events::new().on_click({
                    let user = ChatUser::Character(BlockMut::clone(&character));
                    move |_| Msg::OpenChatModeless(user)
                }),
                vec![Html::text("チャットを表示")],
            ),
            Self::render_is_fixed_position(
                character
                    .map(|character| character.is_fixed_position())
                    .unwrap_or(false),
                BlockMut::clone(&character).untyped(),
            ),
            Self::render_is_bind_to_grid(
                character
                    .map(|character| character.is_bind_to_grid())
                    .unwrap_or(false),
                BlockMut::clone(&character).untyped(),
            ),
        ]
    }

    fn render_contextmenu_craftboard(
        &self,
        craftboard: &BlockMut<block::Craftboard>,
    ) -> Vec<Html<Self>> {
        vec![
            Marker::light(
                Attributes::new(),
                Events::new(),
                vec![Html::text(
                    craftboard
                        .map(|craftboard| craftboard.name().clone())
                        .unwrap_or(String::from("")),
                )],
            ),
            Btn::menu(
                Attributes::new(),
                Events::new().on_click({
                    let block_id = craftboard.id();
                    move |_| Msg::OpenCraftboardModeless(block_id)
                }),
                vec![Html::text("詳細を表示")],
            ),
            Self::render_is_fixed_position(
                craftboard
                    .map(|craftboard| craftboard.is_fixed_position())
                    .unwrap_or(false),
                BlockMut::clone(&craftboard).untyped(),
            ),
            Self::render_is_bind_to_grid(
                craftboard
                    .map(|craftboard| craftboard.is_bind_to_grid())
                    .unwrap_or(false),
                BlockMut::clone(&craftboard).untyped(),
            ),
        ]
    }

    fn render_is_fixed_position(is_fixed_position: bool, block: BlockMut<Untyped>) -> Html<Self> {
        Btn::menu(
            Attributes::new(),
            Events::new()
                .on_click(move |_| Msg::SetBlockIsFixedPosition(block, !is_fixed_position)),
            vec![if is_fixed_position {
                Html::text("固定解除")
            } else {
                Html::text("場所を固定")
            }],
        )
    }

    fn render_is_bind_to_grid(is_bind_to_grid: bool, block: BlockMut<Untyped>) -> Html<Self> {
        Btn::menu(
            Attributes::new(),
            Events::new().on_click(move |_| Msg::SetBlockIsBindToGrid(block, !is_bind_to_grid)),
            vec![if is_bind_to_grid {
                Html::text("グリッドにスナップしない")
            } else {
                Html::text("グリッドにスナップする")
            }],
        )
    }
}
