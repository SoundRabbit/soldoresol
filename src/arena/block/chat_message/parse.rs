use peg;
use std::rc::Rc;

#[derive(Clone)]
pub struct Message(Vec<MessageToken>);

impl Message {
    pub fn new(msg: &str) -> Self {
        if let Ok(msg) = message_parser::message(msg) {
            msg
        } else {
            Self(vec![MessageToken::Text(String::from(msg))])
        }
    }

    pub fn map(self, map: impl Fn(MapToken) -> Message + 'static) -> Self {
        MapMessage::wrap(self, Rc::new(map)).get()
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

impl From<Vec<MessageToken>> for Message {
    fn from(data: Vec<MessageToken>) -> Self {
        Self(data)
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

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(MessageToken::to_string)
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Clone)]
pub enum Token<T: Clone> {
    Text(String),
    Refer(T),
    CommandBlock(Command<T>, T),
}

pub type MessageToken = Token<Message>;
pub type MapToken = Token<MapMessage>;

impl MessageToken {
    fn wrap(self, map: Rc<dyn Fn(MapToken) -> Message>) -> MapToken {
        match self {
            Self::Text(x) => Token::Text(x),
            Self::Refer(x) => Token::Refer(MapMessage::wrap(x, map)),
            Self::CommandBlock(x, t) => Token::CommandBlock(
                Command::wrap(x, Rc::clone(&map)),
                MapMessage::wrap(t, Rc::clone(&map)),
            ),
        }
    }
}

impl std::fmt::Display for MessageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(x) => write!(f, "{}", x),
            Self::Refer(x) => write!(f, "{}{}{}", "{", x, "}"),
            Self::CommandBlock(x, t) => write!(f, r"{}\{}{}{}", r"{", x.to_string(), t, "}"),
        }
    }
}

#[derive(Clone)]
pub struct Command<T: Clone> {
    pub name: T,
    pub args: Vec<T>,
}

pub type MessageCommand = Command<Message>;
pub type MapCommand = Command<MapMessage>;

impl MessageCommand {
    fn wrap(self, map: Rc<dyn Fn(MapToken) -> Message>) -> Command<MapMessage> {
        let name = MapMessage::wrap(self.name, Rc::clone(&map));
        let args = self
            .args
            .into_iter()
            .map(|arg| MapMessage::wrap(arg, Rc::clone(&map)))
            .collect();

        Command { name, args }
    }
}

impl std::fmt::Display for MessageCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"{}[{}]",
            self.name,
            self.args
                .iter()
                .map(Message::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Clone)]
pub struct MapMessage {
    data: Message,
    map: Rc<dyn Fn(MapToken) -> Message>,
}

impl MapMessage {
    fn wrap(data: Message, map: Rc<dyn Fn(MapToken) -> Message>) -> Self {
        Self { data, map }
    }
    pub fn get(self) -> Message {
        let mut mapped = vec![];

        for token in self.data.0 {
            let map = Rc::clone(&self.map);
            let mapped_tokens = (self.map)(token.wrap(map)).0;
            for mapped_token in mapped_tokens {
                mapped.push(mapped_token);
            }
        }

        Message::from(mapped)
    }
}

peg::parser! {
    grammar message_parser() for str {
        pub rule message() -> Message
            = tokens:message_token()* { Message(tokens).compress() }

        rule message_token() -> MessageToken = precedence! {
            r"{\" command:command() text:block_text_token()* "}" { MessageToken::CommandBlock(command, Message(text)) }
            --
            "{" text:block_text_token()* "}" { MessageToken::Refer(Message(text))}
            --
            r"\{" { MessageToken::Text(String::from(r"{")) }
            r"\}" { MessageToken::Text(String::from(r"}")) }
            r"\\" { MessageToken::Text(String::from(r"\")) }
            r"\n" { MessageToken::Text(String::from("\n")) }
            "\\\n" { MessageToken::Text(String::from("")) }
            --
            t:$([_]) { MessageToken::Text(String::from(t)) }
        }

        rule block_text_token() -> MessageToken
            = !['}'] token:message_token() { token }

        rule command() -> Command<Message> = precedence! {
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
