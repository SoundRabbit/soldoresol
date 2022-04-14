use super::super::atom::{
    btn::{self, Btn},
    dropdown::{self, Dropdown},
    fa, text,
};
use super::{
    ChatPalletIndex, ChatPalletSectionIndex, ChatUser, InputingMessage, SharedState, ShowingModal,
};
use crate::arena::block;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props {
    chat_user: ChatUser,
    shared_state: Rc<RefCell<SharedState>>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetIsShowing(bool),
    SetSelectedSection(Option<usize>),
    SetSelectedItem(ChatPalletIndex),
}

pub enum On {
    OpenModal(ShowingModal),
    SendInputingChatMessage,
}

pub struct ChatPallet {
    chat_user: ChatUser,
    is_showing: bool,
    selected_section: Option<usize>,
    shared_state: Rc<RefCell<SharedState>>,
}

impl Component for ChatPallet {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ChatPallet {}

impl Constructor for ChatPallet {
    fn constructor(props: Self::Props) -> Self {
        Self {
            chat_user: props.chat_user,
            is_showing: true,
            selected_section: None,
            shared_state: props.shared_state,
        }
    }
}

impl Update for ChatPallet {
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.shared_state = props.shared_state;
        if self.chat_user != props.chat_user {
            self.chat_user = props.chat_user;
            self.selected_section = None;
            let text = if let InputingMessage::ChatPallet { text, .. } =
                &self.shared_state.borrow().inputing_message
            {
                Some(text.clone())
            } else {
                None
            };
            if let Some(text) = text {
                self.shared_state.borrow_mut().inputing_message = InputingMessage::Text(text);
            }
        }
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(e) => Cmd::submit(e),
            Msg::SetIsShowing(is_showing) => {
                self.is_showing = is_showing;
                Cmd::none()
            }
            Msg::SetSelectedSection(selected_section) => {
                self.selected_section = selected_section;
                Cmd::none()
            }
            Msg::SetSelectedItem(item) => {
                if let InputingMessage::ChatPallet { index, .. } =
                    &self.shared_state.borrow().inputing_message
                {
                    if item == *index {
                        return Cmd::submit(On::SendInputingChatMessage);
                    }
                }

                if let ChatUser::Character(character) = &self.chat_user {
                    character.map(|character| {
                        if let Some(text) =
                            super::get_chatpallet_item(character.chatpallet(), &item)
                        {
                            self.shared_state.borrow_mut().inputing_message =
                                InputingMessage::ChatPallet { text, index: item }
                        }
                    });
                }

                Cmd::none()
            }
        }
    }
}

impl Render<Html> for ChatPallet {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("container")),
            Events::new(),
            if let ChatUser::Character(character) = &self.chat_user {
                character.map(|character| {
                    vec![
                        self.render_container(character.chatpallet()),
                        Btn::light(
                            Attributes::new().title(if self.is_showing {
                                "チャットパレットをしまう"
                            } else {
                                "チャットパレットを表示"
                            }),
                            Events::new().on_click(self, {
                                let is_showing = self.is_showing;
                                move |_| Msg::SetIsShowing(!is_showing)
                            }),
                            vec![fa::fas_i(if self.is_showing {
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
        ))
    }
}

impl ChatPallet {
    fn render_container(&self, chatpallet: &block::character::ChatPallet) -> Html {
        Html::div(
            Attributes::new()
                .string("data-is-showing", self.is_showing.to_string())
                .class(Self::class("base")),
            Events::new(),
            vec![
                Dropdown::new(
                    self,
                    None,
                    dropdown::Props {
                        direction: dropdown::Direction::Bottom,
                        toggle_type: dropdown::ToggleType::Click,
                        variant: btn::Variant::DarkLikeMenu,
                    },
                    Sub::none(),
                    (
                        vec![Html::text(
                            self.selected_section
                                .and_then(|section_idx| {
                                    chatpallet
                                        .sections()
                                        .get(section_idx)
                                        .map(|sec| sec.name().clone())
                                })
                                .unwrap_or(String::from("[チャットパレット]")),
                        )],
                        {
                            let mut a = if chatpallet.children().is_empty()
                                && chatpallet.sub_sections().is_empty()
                            {
                                vec![]
                            } else {
                                vec![Btn::menu(
                                    Attributes::new(),
                                    Events::new()
                                        .on_click(self, move |_| Msg::SetSelectedSection(None)),
                                    vec![Html::text("[チャットパレット]")],
                                )]
                            };
                            for (idx, sec) in chatpallet.sections().iter().enumerate() {
                                a.push(Btn::menu(
                                    Attributes::new(),
                                    Events::new().on_click(self, move |_| {
                                        Msg::SetSelectedSection(Some(idx))
                                    }),
                                    vec![Html::text(sec.name())],
                                ));
                            }
                            a
                        },
                    ),
                ),
                self.selected_section
                    .and_then(|sec_idx| {
                        chatpallet.sections().get(sec_idx).map(|sec| {
                            self.render_section(sec.children(), sec.sub_sections(), Some(sec_idx))
                        })
                    })
                    .unwrap_or_else(|| {
                        self.render_section(chatpallet.children(), chatpallet.sub_sections(), None)
                    }),
                Btn::dark(
                    Attributes::new(),
                    Events::new()
                        .on_click(self, |_| Msg::Sub(On::OpenModal(ShowingModal::Chatpallet))),
                    vec![Html::text("編集")],
                ),
            ],
        )
    }

    fn render_section(
        &self,
        children: &Vec<String>,
        sub_sections: &Vec<block::character::ChatPalletSubSection>,
        sec_idx: Option<usize>,
    ) -> Html {
        let mut items = vec![];

        self.render_children(&mut items, children, sec_idx, None);

        let mut ssec_idx = 0;
        for ssec in sub_sections {
            items.push(text::div(ssec.name()));
            self.render_children(&mut items, ssec.children(), sec_idx, Some(ssec_idx));
            ssec_idx += 1;
        }

        Html::div(
            Attributes::new().class(Self::class("section")),
            Events::new(),
            items,
        )
    }

    fn render_children(
        &self,
        items: &mut Vec<Html>,
        children: &Vec<String>,
        sec_idx: Option<usize>,
        ssec_idx: Option<usize>,
    ) {
        let mut item_idx = 0;
        for child in children {
            if !child.is_empty() {
                let item = match sec_idx {
                    Some(sec_idx) => match ssec_idx {
                        Some(ssec_idx) => ChatPalletIndex::Section(
                            sec_idx,
                            ChatPalletSectionIndex::SubSection(ssec_idx, item_idx),
                        ),
                        None => ChatPalletIndex::Section(
                            sec_idx,
                            ChatPalletSectionIndex::Children(item_idx),
                        ),
                    },
                    None => match ssec_idx {
                        Some(ssec_idx) => ChatPalletIndex::SubSection(ssec_idx, item_idx),
                        None => ChatPalletIndex::Children(item_idx),
                    },
                };

                items.push(Btn::with_variant(
                    btn::Variant::LightLikeMenu,
                    Attributes::new().class(Self::class("item")),
                    Events::new().on_click(self, move |_| Msg::SetSelectedItem(item)),
                    vec![Html::text(child)],
                ));
            }
            item_idx += 1;
        }
    }
}

impl Styled for ChatPallet {
    fn style() -> Style {
        style! {
            ".container" {
                "grid-column": "1 / 2";
                "grid-row": "1 / 3";
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "grid-template-rows": "1fr";
            }

            ".container > button" {
                "padding-left": ".35em";
                "padding-right": ".35em";
            }

            ".base" {
                "overflow": "hidden";
                "display": "grid";
                "grid-template-rows": "max-content 1fr max-content";
                "row-gap": ".65rem";
            }

            ".base[data-is-showing='false']" {
                "width": "0";
            }

            ".base[data-is-showing='true']" {
                "min-width": "30ch";
                "max-width": "30ch";
                "padding-left": ".65rem";
            }

            ".section" {
                "overflow-y": "scroll";
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": ".65rem";
            }

            ".item" {
                "white-space": "pre-wrap";
                "font-size": "0.8rem";
            }
        }
    }
}
