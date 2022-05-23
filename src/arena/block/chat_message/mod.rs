#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::resource::ImageData;
use super::util::Pack;
use super::Property;
use super::{BlockMut, BlockRef};
use std::collections::HashMap;

pub mod map;
pub mod parser;

pub use parser::Argument;
pub use parser::Command;
pub use parser::Message;
pub use parser::MessageToken;
pub use parser::Reference;

pub fn map(
    props: &Vec<BlockMut<Property>>,
    mut refs: impl FnMut(&String) -> Message,
    message: Message,
) -> (Message, Vec<(String, String)>) {
    let mut descriptions = vec![];
    let mut var_nums = HashMap::new();
    let message = map::map_message(props, &mut refs, &mut var_nums, &mut descriptions, message);
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
            Self::Text(x) => (object! {
                "_tag": "Text",
                "_val": JsValue::from(x)
            })
            .into(),
            Self::Reference(reference) => (object! {
                "_tag": "Refer",
                "_val": reference.pack(is_deep).await
            })
            .into(),
            Self::Command(command) => (object! {
                "_tag": "CommandBlock",
                "_val": command.pack(is_deep).await
            })
            .into(),
        }
    }
}

#[async_trait(?Send)]
impl Pack for Command {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let name = self.name.pack(is_deep).await;

        let args = array![];
        for arg in &self.args {
            args.push(&arg.pack(is_deep).await);
        }

        (object! {
            "name": name,
            "args": args,
            "text": self.text.pack(is_deep).await
        })
        .into()
    }
}

#[async_trait(?Send)]
impl Pack for Reference {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let pakced_name = js_sys::Array::new();
        for a_name in &self.name {
            pakced_name.push(&a_name.pack(is_deep).await);
        }

        let packed_args = array![];
        for arg in &self.args {
            packed_args.push(&arg.pack(is_deep).await);
        }

        let packed_option;
        if let Some(option) = &self.option {
            packed_option = option.pack(is_deep).await;
        } else {
            packed_option = JsValue::null();
        }

        (object! {
            "name": pakced_name,
            "args": packed_args,
            "option": packed_option
        })
        .into()
    }
}

#[async_trait(?Send)]
impl Pack for Argument {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let packed = js_sys::Array::new();

        packed.push(&self.value.pack(is_deep).await);
        if let Some(option) = &self.option {
            packed.push(&option.pack(is_deep).await);
        }

        packed.into()
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
