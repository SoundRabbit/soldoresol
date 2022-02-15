use super::*;

impl RoomModelessChat {
    pub fn send_chat_message(
        &mut self,
        props: &Props,
        sender: block::chat_message::Sender,
        mut channel: BlockMut<block::ChatChannel>,
        message: &String,
    ) -> Cmd<Self> {
        let message = block::chat_message::Message::new(message);
        let (message, descriptions) = if let ChatUser::Character(character) = &props.user {
            block::chat_message::map(character.chat_ref(), message)
        } else {
            block::chat_message::map(Self::ref_none(), message)
        };

        if descriptions.len() > 0 {
            self.showing_modal = ShowingModal::ChatCapture(WaitingChatMessage {
                channel: channel,
                message: message,
                descriptions: Rc::new(descriptions),
                sender: sender,
            });
            return Cmd::none();
        }

        let chat_message = block::ChatMessage::new(sender, chrono::Utc::now(), message);
        let chat_message = self.arena.insert(chat_message);
        let chat_message_id = chat_message.id();
        channel.update(|channel: &mut block::ChatChannel| {
            channel.messages_push(chat_message);
        });
        let channel_id = channel.id();
        Cmd::sub(On::UpdateBlocks {
            insert: set! { chat_message_id },
            update: set! { channel_id },
        })
    }

    pub fn send_waitng_chat_message(&mut self, captured: &Vec<String>) -> Cmd<Self> {
        let mut showing_modal = ShowingModal::None;
        std::mem::swap(&mut self.showing_modal, &mut showing_modal);
        if let ShowingModal::ChatCapture(WaitingChatMessage {
            mut channel,
            message,
            sender,
            ..
        }) = showing_modal
        {
            let message = Self::capture_message(&captured, message);
            let chat_message = block::ChatMessage::new(sender, chrono::Utc::now(), message);
            let chat_message = self.arena.insert(chat_message);
            let chat_message_id = chat_message.id();
            channel.update(|channel: &mut block::ChatChannel| {
                channel.messages_push(chat_message);
            });
            let channel_id = channel.id();
            Cmd::sub(On::UpdateBlocks {
                insert: set! { chat_message_id },
                update: set! { channel_id },
            })
        } else {
            Cmd::none()
        }
    }

    fn ref_none<'a>() -> impl FnMut(&String) -> block::chat_message::Message + 'a {
        |_ref_name: &String| block::chat_message::Message::from(vec![])
    }

    fn capture_message(
        captured: &Vec<String>,
        message: block::chat_message::Message,
    ) -> block::chat_message::Message {
        message.map(|token| Self::capture_message_token(captured, token))
    }

    fn capture_message_token(
        captured: &Vec<String>,
        token: block::chat_message::MessageToken,
    ) -> block::chat_message::Message {
        match token {
            block::chat_message::MessageToken::Text(text) => block::chat_message::Message::from(
                vec![block::chat_message::MessageToken::Text(text)],
            ),
            block::chat_message::MessageToken::Refer(refer) => {
                block::chat_message::Message::from(vec![block::chat_message::MessageToken::Refer(
                    Self::capture_message(captured, refer),
                )])
            }
            block::chat_message::MessageToken::CommandBlock(cmd, text) => {
                let cmd_name = Self::capture_message(captured, cmd.name);
                let cmd_args: Vec<_> = cmd
                    .args
                    .into_iter()
                    .map(|x| Self::capture_message(captured, x))
                    .collect();

                if cmd_name.to_string() == "ref" {
                    let cap_name = Self::capture_message(captured, text).to_string();
                    let text = cap_name
                        .parse()
                        .ok()
                        .and_then(|x: usize| captured.get(x - 1).map(|x: &String| x.clone()))
                        .unwrap_or(String::from(""));

                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::Text(text),
                    ])
                } else {
                    let text = Self::capture_message(captured, text);
                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::CommandBlock(
                            block::chat_message::MessageCommand {
                                name: cmd_name,
                                args: cmd_args,
                            },
                            text,
                        ),
                    ])
                }
            }
        }
    }
}
