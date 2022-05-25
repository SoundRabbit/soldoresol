use super::*;

impl RoomModelessChat {
    pub fn send_chat_message(
        mut self: Pin<&mut Self>,
        sender: block::chat_message::Sender,
        mut channel: BlockMut<block::ChatChannel>,
        message: &String,
    ) -> Cmd<Self> {
        let message = block::chat_message::Message::from_str(message);
        let (message, descriptions) = if let ChatUser::Character(character) = &self.chat_user {
            if let Some(res) = character.map(|character| {
                block::chat_message::map(character.properties(), character.chat_ref(), message)
            }) {
                res
            } else {
                return Cmd::none();
            }
        } else {
            block::chat_message::map(&vec![], Self::ref_none(), message)
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

        let command_results =
            if let Some(game_system_class) = self.game_system_class.borrow().as_ref() {
                block::chat_message::roll(game_system_class, &message)
            } else {
                crate::debug::log_1("no dicebot");
                vec![]
            };

        let mut message_ids = set! {};
        let now = chrono::Utc::now();

        let chat_message = block::ChatMessage::new(sender, now.clone(), message);
        let chat_message = self.arena.insert(chat_message);
        message_ids.insert(chat_message.id());
        channel.update(|channel: &mut block::ChatChannel| {
            channel.messages_push(chat_message);
        });

        for command_result in command_results {
            let message = block::chat_message::Message::from(&command_result);
            let sender = block::chat_message::Sender::new(
                Rc::clone(&self.client_id),
                None,
                String::from("System"),
            );
            let chat_message = block::ChatMessage::new(sender, now.clone(), message);
            let chat_message = self.arena.insert(chat_message);
            message_ids.insert(chat_message.id());
            channel.update(|channel: &mut block::ChatChannel| {
                channel.messages_push(chat_message);
            });
        }

        let channel_id = channel.id();
        Cmd::submit(On::UpdateBlocks {
            insert: message_ids,
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
            Cmd::submit(On::UpdateBlocks {
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
            block::chat_message::MessageToken::Reference(reference) => {
                block::chat_message::Message::from(vec![
                    block::chat_message::MessageToken::Reference(block::chat_message::Reference {
                        name: reference
                            .name
                            .into_iter()
                            .map(|a_name| Self::capture_message(captured, a_name))
                            .collect(),
                        args: reference
                            .args
                            .into_iter()
                            .map(|arg| Self::capture_argument(captured, arg))
                            .collect(),
                        option: reference
                            .option
                            .map(|option| Self::capture_message(captured, option)),
                    }),
                ])
            }
            block::chat_message::MessageToken::Command(cmd) => {
                let cmd_name = Self::capture_message(captured, cmd.name);
                let cmd_args: Vec<_> = cmd
                    .args
                    .into_iter()
                    .map(|x| Self::capture_argument(captured, x))
                    .collect();

                if cmd_name.to_string() == "ref" {
                    let cap_name = Self::capture_message(captured, cmd.text).to_string();
                    let text = cap_name
                        .parse()
                        .ok()
                        .and_then(|x: usize| captured.get(x - 1).map(|x: &String| x.clone()))
                        .unwrap_or(String::from(""));

                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::Text(text),
                    ])
                } else {
                    let text = Self::capture_message(captured, cmd.text);
                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::Command(block::chat_message::Command {
                            name: cmd_name,
                            args: cmd_args,
                            text,
                        }),
                    ])
                }
            }
        }
    }

    fn capture_argument(
        captured: &Vec<String>,
        arg: block::chat_message::Argument,
    ) -> block::chat_message::Argument {
        let value = Self::capture_message(captured, arg.value);
        let option = arg
            .option
            .map(|option| Self::capture_message(captured, option));

        block::chat_message::Argument { value, option }
    }
}
