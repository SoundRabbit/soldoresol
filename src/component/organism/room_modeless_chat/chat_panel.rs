use super::*;

impl RoomModelessChat {
    pub fn render_chat_panel(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("chatpallet-container")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new()
                        .string("data-is-showing", self.is_showing_chat_pallet.to_string())
                        .class(Self::class("chatpallet")),
                    Events::new(),
                    vec![
                        Dropdown::with_children(
                            dropdown::Props {
                                text: dropdown::Text::Text(
                                    self.test_chatpallet
                                        .index()
                                        .get(self.test_chatpallet_selected_index)
                                        .map(|(name, _)| name.clone())
                                        .unwrap_or(String::from("")),
                                ),
                                direction: dropdown::Direction::Bottom,
                                toggle_type: dropdown::ToggleType::Click,
                                variant: btn::Variant::DarkLikeMenu,
                            },
                            Sub::none(),
                            self.test_chatpallet
                                .index()
                                .iter()
                                .enumerate()
                                .map(|(idx, (name, _))| {
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(move |_| {
                                            Msg::SetTestChatPalletSelectedIndex(idx)
                                        }),
                                        vec![Html::text(name)],
                                    )
                                })
                                .collect(),
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("chatpallet-index")),
                            Events::new(),
                            self.test_chatpallet
                                .index()
                                .get(self.test_chatpallet_selected_index)
                                .map(|(_, items)| {
                                    items
                                        .iter()
                                        .enumerate()
                                        .map(|(idx, item)| {
                                            Btn::with_variant(
                                                btn::Variant::LightLikeMenu,
                                                Attributes::new()
                                                    .class(Self::class("chatpallet-item")),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTestChatPalletSelectedItem(idx)
                                                }),
                                                vec![Html::text(item)],
                                            )
                                        })
                                        .collect()
                                })
                                .unwrap_or(vec![]),
                        ),
                    ],
                ),
                Btn::light(
                    Attributes::new().title(if self.is_showing_chat_pallet {
                        "チャットパレットをしまう"
                    } else {
                        "チャットパレットを表示"
                    }),
                    Events::new().on_click({
                        let is_showing = self.is_showing_chat_pallet;
                        move |_| Msg::SetIsShowingChatPallet(!is_showing)
                    }),
                    vec![fa::i(if self.is_showing_chat_pallet {
                        "fa-caret-left"
                    } else {
                        "fa-caret-right"
                    })],
                ),
            ],
        )
    }
}
