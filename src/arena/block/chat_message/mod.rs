uses! {}

use super::super::resource::ImageData;
use super::util::Pack;
use super::BlockRef;
use std::collections::HashMap;

pub mod map;
pub mod parse;

pub use parse::Message;
pub use parse::MessageCommand;
pub use parse::MessageToken;

pub fn map(
    mut refs: impl FnMut(&String) -> Message,
    message: Message,
) -> (Message, Vec<(String, String)>) {
    let mut descriptions = vec![];
    let mut var_nums = HashMap::new();
    let message = map::map_message(&mut refs, &mut var_nums, &mut descriptions, message);
    (message, descriptions)
}

#[async_trait(?Send)]
impl Pack for Message {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let data = array![];

        for token in self.iter() {
            data.push(&token.pack(is_deep).await);
        }

        data.into()
    }
}

#[async_trait(?Send)]
impl Pack for MessageToken {
    async fn pack(&self, is_deep: bool) -> JsValue {
        match self {
            Self::Text(x) => (object! {"Text": JsValue::from(x)}).into(),
            Self::Refer(x) => (object! {"Refer": x.pack(is_deep).await}).into(),
            Self::CommandBlock(c, m) => {
                (object! {"CommandBlock": array![c.pack(is_deep).await, m.pack(is_deep).await]})
                    .into()
            }
        }
    }
}

#[async_trait(?Send)]
impl Pack for MessageCommand {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let name = self.name.pack(is_deep).await;

        let args = array![];
        for arg in &self.args {
            args.push(&arg.pack(is_deep).await);
        }

        (object! {
            "name": name,
            "args": args
        })
        .into()
    }
}

block! {
    [pub Sender(constructor, pack)]
    (client_id): Rc<String>;
    (icon): Option<BlockRef<ImageData>>;
    (name): String;
}

impl Sender {
    pub fn client_id(&self) -> &Rc<String> {
        &self.client_id
    }

    pub fn icon(&self) -> Option<&BlockRef<ImageData>> {
        self.icon.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

block! {
    [pub ChatMessage(constructor, pack)]
    (sender): Sender;
    (timestamp): chrono::DateTime<chrono::Utc>;
    (message): Message;
    reference: Option<BlockRef<Self>> = None;
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
