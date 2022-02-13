use super::*;

impl RoomModelessChat {
    pub fn render_chat_pallet(&self) -> Html<Self> {
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
                                    self.test_chatpallet_selected_section
                                        .and_then(|section_idx| {
                                            self.test_chatpallet
                                                .sections()
                                                .get(section_idx)
                                                .map(|sec| sec.name().clone())
                                        })
                                        .unwrap_or(String::from("")),
                                ),
                                direction: dropdown::Direction::Bottom,
                                toggle_type: dropdown::ToggleType::Click,
                                variant: btn::Variant::DarkLikeMenu,
                            },
                            Sub::none(),
                            {
                                let mut a = if self.test_chatpallet.children().is_empty()
                                    && self.test_chatpallet.sub_sections().is_empty()
                                {
                                    vec![]
                                } else {
                                    vec![Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(move |_| {
                                            Msg::SetTestChatPalletSelectedSection(None)
                                        }),
                                        vec![Html::text("")],
                                    )]
                                };
                                for (idx, sec) in self.test_chatpallet.sections().iter().enumerate()
                                {
                                    a.push(Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(move |_| {
                                            Msg::SetTestChatPalletSelectedSection(Some(idx))
                                        }),
                                        vec![Html::text(sec.name())],
                                    ));
                                }
                                a
                            },
                        ),
                        self.test_chatpallet_selected_section
                            .and_then(|sec_idx| {
                                self.test_chatpallet.sections().get(sec_idx).map(|sec| {
                                    self.render_chat_pallet_section(
                                        sec.children(),
                                        sec.sub_sections(),
                                        Some(sec_idx),
                                    )
                                })
                            })
                            .unwrap_or_else(|| {
                                self.render_chat_pallet_section(
                                    self.test_chatpallet.children(),
                                    self.test_chatpallet.sub_sections(),
                                    None,
                                )
                            }),
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

    fn render_chat_pallet_section(
        &self,
        children: &Vec<String>,
        sub_sections: &Vec<block::character::ChatPalletSubSection>,
        sec_idx: Option<usize>,
    ) -> Html<Self> {
        let mut items = vec![];

        Self::render_chat_pallet_children(&mut items, children, sec_idx, None);

        let mut ssec_idx = 0;
        for ssec in sub_sections {
            items.push(text::div(ssec.name()));
            Self::render_chat_pallet_children(&mut items, ssec.children(), sec_idx, Some(ssec_idx));
            ssec_idx += 1;
        }

        Html::div(
            Attributes::new().class(Self::class("chatpallet-section")),
            Events::new(),
            items,
        )
    }

    fn render_chat_pallet_children(
        items: &mut Vec<Html<Self>>,
        children: &Vec<String>,
        sec_idx: Option<usize>,
        ssec_idx: Option<usize>,
    ) {
        let mut item_idx = 0;
        for child in children {
            let item = match sec_idx {
                Some(sec_idx) => match ssec_idx {
                    Some(ssec_idx) => ChatPalletItem::Section(
                        sec_idx,
                        ChatPalletSectionItem::SubSection(ssec_idx, item_idx),
                    ),
                    None => {
                        ChatPalletItem::Section(sec_idx, ChatPalletSectionItem::Children(item_idx))
                    }
                },
                None => match ssec_idx {
                    Some(ssec_idx) => ChatPalletItem::SubSection(ssec_idx, item_idx),
                    None => ChatPalletItem::Children(item_idx),
                },
            };

            items.push(Btn::with_variant(
                btn::Variant::LightLikeMenu,
                Attributes::new().class(Self::class("chatpallet-item")),
                Events::new().on_click(move |_| Msg::SetTestChatPalletSelectedItem(item)),
                vec![Html::text(child)],
            ));

            item_idx += 1;
        }
    }
}
