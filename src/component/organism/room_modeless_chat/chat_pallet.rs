use super::*;

impl RoomModelessChat {
    pub fn render_chat_pallet(&self, props: &Props) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("chatpallet-container")),
            Events::new(),
            if let ChatUser::Character(character) = &props.user {
                character.map(|character| {
                    vec![
                        self.render_chat_pallet_container(character.chatpallet()),
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
                    ]
                })
            } else {
                None
            }
            .unwrap_or(vec![]),
        )
    }

    fn render_chat_pallet_container(
        &self,
        chatpallet: &block::character::ChatPallet,
    ) -> Html<Self> {
        Html::div(
            Attributes::new()
                .string("data-is-showing", self.is_showing_chat_pallet.to_string())
                .class(Self::class("chatpallet")),
            Events::new(),
            vec![
                Dropdown::with_children(
                    dropdown::Props {
                        text: dropdown::Text::Text(
                            self.chatpallet_selected_section
                                .and_then(|section_idx| {
                                    chatpallet
                                        .sections()
                                        .get(section_idx)
                                        .map(|sec| sec.name().clone())
                                })
                                .unwrap_or(String::from("[チャットパレット]")),
                        ),
                        direction: dropdown::Direction::Bottom,
                        toggle_type: dropdown::ToggleType::Click,
                        variant: btn::Variant::DarkLikeMenu,
                    },
                    Sub::none(),
                    {
                        let mut a = if chatpallet.children().is_empty()
                            && chatpallet.sub_sections().is_empty()
                        {
                            vec![]
                        } else {
                            vec![Btn::menu(
                                Attributes::new(),
                                Events::new()
                                    .on_click(move |_| Msg::SetChatPalletSelectedSection(None)),
                                vec![Html::text("[チャットパレット]")],
                            )]
                        };
                        for (idx, sec) in chatpallet.sections().iter().enumerate() {
                            a.push(Btn::menu(
                                Attributes::new(),
                                Events::new().on_click(move |_| {
                                    Msg::SetChatPalletSelectedSection(Some(idx))
                                }),
                                vec![Html::text(sec.name())],
                            ));
                        }
                        a
                    },
                ),
                self.chatpallet_selected_section
                    .and_then(|sec_idx| {
                        chatpallet.sections().get(sec_idx).map(|sec| {
                            self.render_chat_pallet_section(
                                sec.children(),
                                sec.sub_sections(),
                                Some(sec_idx),
                            )
                        })
                    })
                    .unwrap_or_else(|| {
                        self.render_chat_pallet_section(
                            chatpallet.children(),
                            chatpallet.sub_sections(),
                            None,
                        )
                    }),
                Btn::dark(
                    Attributes::new(),
                    Events::new().on_click(|_| Msg::SetShowingModal(ShowingModal::Chatpallet)),
                    vec![Html::text("編集")],
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
            if !child.is_empty() {
                let item = match sec_idx {
                    Some(sec_idx) => match ssec_idx {
                        Some(ssec_idx) => ChatPalletItem::Section(
                            sec_idx,
                            ChatPalletSectionItem::SubSection(ssec_idx, item_idx),
                        ),
                        None => ChatPalletItem::Section(
                            sec_idx,
                            ChatPalletSectionItem::Children(item_idx),
                        ),
                    },
                    None => match ssec_idx {
                        Some(ssec_idx) => ChatPalletItem::SubSection(ssec_idx, item_idx),
                        None => ChatPalletItem::Children(item_idx),
                    },
                };

                items.push(Btn::with_variant(
                    btn::Variant::LightLikeMenu,
                    Attributes::new().class(Self::class("chatpallet-item")),
                    Events::new().on_click(move |_| Msg::SetChatPalletSelectedItem(item)),
                    vec![Html::text(child)],
                ));
            }
            item_idx += 1;
        }
    }
}
