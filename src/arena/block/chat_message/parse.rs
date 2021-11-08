use peg;

pub struct Message(Vec<MessageToken>);

impl Message {
    fn evalute(
        self,
        refer: &mut dyn FnMut(EvalutedMessage) -> EvalutedMessage,
        command: &mut dyn FnMut(EvalutedCommand, EvalutedMessage) -> EvalutedMessage,
    ) -> EvalutedMessage {
        let msg: Vec<_> = self.into();
        let mut evaluted_msg = vec![];

        for token in msg {
            let mut evaluted_tokens = token.evalute(refer, command);
            evaluted_msg.append(&mut evaluted_tokens);
        }

        EvalutedMessage(evaluted_msg)
    }

    fn compress(self) -> Self {
        let mut text = String::new();
        let mut mapped = vec![];
        let msg: Vec<_> = self.into();

        for token in msg {
            match token {
                MessageToken::Text(mut t) => {
                    text.push_str(t.drain(..).as_str());
                }
                MessageToken::CommandBlock(c, m) => {
                    if !text.is_empty() {
                        mapped.push(MessageToken::Text(text.drain(..).collect()));
                    }
                    let command = Command {
                        name: c.name.compress(),
                        args: c.args.into_iter().map(Message::compress).collect(),
                    };
                    mapped.push(MessageToken::CommandBlock(command, m.compress()));
                }
                MessageToken::Refer(refer) => {
                    if !text.is_empty() {
                        mapped.push(MessageToken::Text(text.drain(..).collect()));
                    }
                    mapped.push(MessageToken::Refer(refer.compress()));
                }
            }
        }

        if !text.is_empty() {
            mapped.push(MessageToken::Text(text.drain(..).collect()));
        }

        Message(mapped)
    }
}

impl Into<Vec<MessageToken>> for Message {
    fn into(self) -> Vec<MessageToken> {
        self.0
    }
}

enum MessageToken {
    Text(String),
    Refer(Message),
    CommandBlock(Command, Message),
}

impl MessageToken {
    fn evalute(
        self,
        refer: &mut dyn FnMut(EvalutedMessage) -> EvalutedMessage,
        command: &mut dyn FnMut(EvalutedCommand, EvalutedMessage) -> EvalutedMessage,
    ) -> Vec<EvalutedMessageToken> {
        match self {
            Self::Text(x) => vec![EvalutedMessageToken::Text(x)],
            Self::Refer(x) => {
                let x = x.evalute(refer, command);
                refer(x).into()
            }
            Self::CommandBlock(c, m) => {
                let c = c.evalute(refer, command);
                let m = m.evalute(refer, command);
                command(c, m).into()
            }
        }
    }
}

struct Command {
    name: Message,
    args: Vec<Message>,
}

impl Command {
    fn evalute(
        self,
        refer: &mut dyn FnMut(EvalutedMessage) -> EvalutedMessage,
        command: &mut dyn FnMut(EvalutedCommand, EvalutedMessage) -> EvalutedMessage,
    ) -> EvalutedCommand {
        let name = self.name.evalute(refer, command);
        let mut args = vec![];

        for arg in self.args {
            args.push(arg.evalute(refer, command));
        }

        EvalutedCommand { name, args }
    }
}

pub struct EvalutedMessage(Vec<EvalutedMessageToken>);

impl EvalutedMessage {
    pub fn new(
        msg: &String,
        refer: impl FnMut(EvalutedMessage) -> EvalutedMessage,
        command: impl FnMut(EvalutedCommand, EvalutedMessage) -> EvalutedMessage,
    ) -> Self {
        match message_parser::message(&msg) {
            Ok(msg) => msg.evalute(Box::leak(Box::new(refer)), Box::leak(Box::new(command))),
            Err(err) => {
                crate::debug::log_1(err.to_string());
                Self(vec![EvalutedMessageToken::Text(msg.clone())])
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut msg = String::from("");

        for token in &self.0 {
            match token {
                EvalutedMessageToken::Text(x) => {
                    msg += x;
                }
                EvalutedMessageToken::CommandBlock(_, x) => {
                    msg += &x.to_string();
                }
            }
        }

        msg
    }
}

impl Into<Vec<EvalutedMessageToken>> for EvalutedMessage {
    fn into(self) -> Vec<EvalutedMessageToken> {
        self.0
    }
}

impl From<Vec<EvalutedMessageToken>> for EvalutedMessage {
    fn from(tokens: Vec<EvalutedMessageToken>) -> Self {
        Self(tokens)
    }
}

impl std::ops::Deref for EvalutedMessage {
    type Target = Vec<EvalutedMessageToken>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EvalutedMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum EvalutedMessageToken {
    Text(String),
    CommandBlock(EvalutedCommand, EvalutedMessage),
}

pub struct EvalutedCommand {
    pub name: EvalutedMessage,
    pub args: Vec<EvalutedMessage>,
}

peg::parser! {
    grammar message_parser() for str {
        pub rule message() -> Message
            = tokens:message_token()* { Message(tokens).compress() }

        rule message_token() -> MessageToken = precedence! {
            r"\{" { MessageToken::Text(String::from(r"{")) }
            r"\}" { MessageToken::Text(String::from(r"}")) }
            r"\\" { MessageToken::Text(String::from(r"\")) }
            r"\n" { MessageToken::Text(String::from("\n")) }
            --
            r"{\" command:command() text:block_text_token()* "}" { MessageToken::CommandBlock(command, Message(text)) }
            --
            "{" text:block_text_token()* "}" { MessageToken::Refer(Message(text))}
            --
            t:$([_]) { MessageToken::Text(String::from(t)) }
        }

        rule block_text_token() -> MessageToken
            = !['}'] token:message_token() { token }

        rule command() -> Command = precedence! {
            name:command_name_token_with_args()* "[" args:(command_arg_token()*) ** "," "]" {
                Command { name: Message(name), args: args.into_iter().map(Message).collect()}
            }
            --
            name:command_name_token_with_no_args()* " "? {
                Command { name: Message(name), args: vec![] }
            }
        }

        rule command_name_token_with_args() -> MessageToken
            = !['['] token:message_token() { token }

        rule command_arg_token() -> MessageToken
            = ![',' | ']'] token:message_token() { token }

        rule command_name_token_with_no_args() -> MessageToken
            = ![' ' | '}'] token:message_token() { token }
    }
}
