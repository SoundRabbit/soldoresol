uses! {
    super::BlockRef;
    super::util::Pack;
}

pub mod parse;

pub use parse::MapCommand;
pub use parse::MapMessage;
pub use parse::MapToken;
pub use parse::Message;
pub use parse::MessageCommand;
pub use parse::MessageToken;

#[async_trait(?Send)]
impl Pack for Message {
    async fn pack(&self) -> JsValue {
        let data = array![];

        for token in self.iter() {
            data.push(&token.pack().await);
        }

        data.into()
    }
}

#[async_trait(?Send)]
impl Pack for MessageToken {
    async fn pack(&self) -> JsValue {
        match self {
            Self::Text(x) => (object! {"Text": JsValue::from(x)}).into(),
            Self::Refer(x) => (object! {"Refer": x.pack().await}).into(),
            Self::CommandBlock(c, m) => {
                (object! {"CommandBlock": array![c.pack().await, m.pack().await]}).into()
            }
        }
    }
}

#[async_trait(?Send)]
impl Pack for MessageCommand {
    async fn pack(&self) -> JsValue {
        let name = self.name.pack().await;

        let args = array![];
        for arg in &self.args {
            args.push(&arg.pack().await);
        }

        (object! {
            "name": name,
            "args": args
        })
        .into()
    }
}

block! {
    [pub Sender]
    (client_id): Rc<String>;
    (icon): Option<BlockRef>;
    (name): String;
}

impl Sender {
    pub fn client_id(&self) -> &Rc<String> {
        &self.client_id
    }

    pub fn icon(&self) -> Option<&BlockRef> {
        self.icon.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

block! {
    [pub ChatMessage]
    (sender): Sender;
    (timestamp): chrono::DateTime<chrono::Utc>;
    (message): Message;
    reference: Option<BlockRef> = None;
}

impl ChatMessage {
    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.timestamp
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}
