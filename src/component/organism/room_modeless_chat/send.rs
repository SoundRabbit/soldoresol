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

        let mut descriptions = vec![];
        let mut var_nums = HashMap::new();
        let message = self.map_message(props, &mut var_nums, &mut descriptions, message);

        if descriptions.len() > 0 {
            self.waiting_chat_message = Some(WaitingChatMessage {
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
        if let Some(WaitingChatMessage {
            mut channel,
            message,
            sender,
            ..
        }) = self.waiting_chat_message.take()
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

    fn ref_def(&self, refer: &String, props: &Props) -> block::chat_message::Message {
        if let ChatUser::Character(character) = &props.user {
            if let Some(message) = character
                .map(|character| {
                    for (pat, text) in character.chatpallet().defs() {
                        if pat.is_match(refer) {
                            let message = block::chat_message::Message::new(
                                pat.replace(refer, text).as_ref(),
                            );
                            return Some(message);
                        }
                    }
                    None
                })
                .unwrap_or(None)
            {
                return message;
            }
        }
        block::chat_message::Message::from(vec![])
    }

    fn map_message(
        &self,
        props: &Props,
        var_nums: &mut HashMap<String, Vec<usize>>,
        descriptions: &mut Vec<(String, String)>,
        message: block::chat_message::Message,
    ) -> block::chat_message::Message {
        message
            .map(|token| self.map_message_token(props, var_nums, descriptions, token))
            .compress()
    }

    fn map_message_token(
        &self,
        props: &Props,
        var_nums: &mut HashMap<String, Vec<usize>>,
        descriptions: &mut Vec<(String, String)>,
        token: block::chat_message::MessageToken,
    ) -> block::chat_message::Message {
        match token {
            block::chat_message::MessageToken::Text(text) => block::chat_message::Message::from(
                vec![block::chat_message::MessageToken::Text(text)],
            ),
            block::chat_message::MessageToken::Refer(refer) => {
                let refer = self.map_message(props, var_nums, descriptions, refer);
                let refer = self.ref_def(&refer.to_string(), props);
                let message = self.map_message(props, var_nums, descriptions, refer);
                message
            }
            block::chat_message::MessageToken::CommandBlock(cmd, text) => {
                let cmd_name = self.map_message(props, var_nums, descriptions, cmd.name);
                let cmd_args: Vec<_> = cmd
                    .args
                    .into_iter()
                    .map(|x| self.map_message(props, var_nums, descriptions, x))
                    .collect();

                if cmd_name.to_string() == "capture" {
                    let mut cap_names = vec![];

                    for args in cmd_args {
                        let args: Vec<_> = args.into();
                        for arg in args {
                            if let block::chat_message::MessageToken::CommandBlock(cap, desc) = arg
                            {
                                for cap_name in cap.args {
                                    let cap_name = cap_name.to_string();
                                    descriptions.push((cap.name.to_string(), desc.to_string()));
                                    let num = descriptions.len();
                                    if let Some(vars) = var_nums.get_mut(&cap_name) {
                                        vars.push(num);
                                    } else {
                                        var_nums.insert(cap_name.clone(), vec![num]);
                                    }
                                    cap_names.push(cap_name);
                                }
                            }
                        }
                    }

                    let text = self.map_message(props, var_nums, descriptions, text);

                    for cap_name in cap_names {
                        if let Some(vars) = var_nums.get_mut(&cap_name) {
                            vars.pop();
                        }
                    }

                    text
                } else if cmd_name.to_string() == "ref" {
                    let cap_name = self
                        .map_message(props, var_nums, descriptions, text)
                        .to_string();
                    let text = if let Some(num) = var_nums.get(&cap_name).and_then(|x| x.last()) {
                        block::chat_message::Message::from(vec![
                            block::chat_message::MessageToken::Text(num.to_string()),
                        ])
                    } else {
                        block::chat_message::Message::from(vec![
                            block::chat_message::MessageToken::Text(cap_name),
                        ])
                    };
                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::CommandBlock(
                            block::chat_message::MessageCommand {
                                name: cmd_name,
                                args: cmd_args,
                            },
                            text,
                        ),
                    ])
                } else {
                    let text = self.map_message(props, var_nums, descriptions, text);
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
