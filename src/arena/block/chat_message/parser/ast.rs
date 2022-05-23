#[derive(Debug, Clone)]
pub struct Message(Vec<MessageToken>);

#[derive(Debug, Clone)]
pub enum MessageToken {
    Text(String),
    Reference(Reference),
    Command(Command),
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub name: Vec<Message>,
    pub args: Vec<Argument>,
    pub option: Option<Message>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: Message,
    pub args: Vec<Argument>,
    pub text: Message,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub value: Message,
    pub option: Option<Message>,
}

impl Message {
    pub fn new(msg_tokens: Vec<MessageToken>) -> Self {
        Self(msg_tokens)
    }

    pub fn from_str(text: &str) -> Self {
        super::message_parser::message(text).unwrap()
    }

    pub fn map(self, f: impl FnMut(MessageToken) -> Message) -> Self {
        Self::new(self.0.into_iter().map(f).map(|m| m.0).flatten().collect())
    }

    pub fn flatten(self) -> Self {
        let mut flatten = vec![];

        for m_token in self.0 {
            match m_token {
                MessageToken::Command(Command { name, args, text }) => {
                    let name = name.flatten();
                    let args = args
                        .into_iter()
                        .map(|arg| Argument {
                            value: arg.value.flatten(),
                            option: arg.option.map(|arg_option| arg_option.flatten()),
                        })
                        .collect();
                    let text = text.flatten();

                    flatten.push(MessageToken::Command(Command { name, args, text }));
                }
                MessageToken::Reference(Reference { name, args, option }) => {
                    let name = name.into_iter().map(|name| name.flatten()).collect();
                    let args = args
                        .into_iter()
                        .map(|arg| Argument {
                            value: arg.value.flatten(),
                            option: arg.option.map(|arg_option| arg_option.flatten()),
                        })
                        .collect();
                    let option = option.map(|option| option.flatten());

                    flatten.push(MessageToken::Reference(Reference { name, args, option }));
                }
                MessageToken::Text(text) => {
                    if let Some(MessageToken::Text(f_text)) = flatten.last_mut() {
                        *f_text += &text;
                    } else {
                        flatten.push(MessageToken::Text(text));
                    }
                }
            }
        }

        Self(flatten)
    }
}

impl std::ops::Deref for Message {
    type Target = Vec<MessageToken>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::convert::Into<Vec<MessageToken>> for Message {
    fn into(self) -> Vec<MessageToken> {
        self.0
    }
}

impl std::convert::From<Vec<MessageToken>> for Message {
    fn from(m_tokens: Vec<MessageToken>) -> Self {
        Self(m_tokens)
    }
}

impl Reference {
    pub fn to_ref_text(&self) -> String {
        let name = self
            .name
            .iter()
            .map(|a_name| format!("{}", a_name))
            .collect::<Vec<_>>()
            .join("::");

        let args = if self.args.len() > 0 {
            format!(
                "[{}]",
                self.args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        } else {
            String::from("")
        };

        let option = if let Some(option) = &self.option {
            format!(".{}", option)
        } else {
            String::from("")
        };

        format!("{}{}{}", name, args, option)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .iter()
                .map(|m_token| format!("{}", m_token))
                .collect::<Vec<_>>()
                .join(""),
        )
    }
}

impl std::fmt::Display for MessageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text),
            Self::Command(command) => write!(f, "{}", command),
            Self::Reference(reference) => write!(f, "{}", reference),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.args.len() > 0 {
            write!(
                f,
                "{{\\{}[{}]{}}}",
                self.name,
                self.args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(","),
                self.text
            )
        } else {
            let text = format!("{}", self.text);
            if text.len() > 0 {
                write!(f, "{{\\{} {}}}", self.name, text)
            } else {
                write!(f, "{{\\{}}}", self.name)
            }
        }
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.to_ref_text())
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(option) = &self.option {
            write!(f, "{}={}", self.value, option)
        } else {
            write!(f, "{}", self.value)
        }
    }
}
