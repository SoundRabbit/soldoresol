#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::resource::ImageData;
use super::util::{Pack, PackDepth};
use super::Property;
use super::{BlockMut, BlockRef};
use crate::libs::bcdice::js::{CommandResult, GameSystemClass};
use std::collections::HashMap;

pub mod map;
pub mod parser;
pub mod roll;

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

pub fn roll(game_system_class: &GameSystemClass, message: &Message) -> Vec<CommandResult> {
    let mut command_results = vec![];
    roll::roll_message(game_system_class, &mut command_results, message);
    command_results
}

#[async_trait(?Send)]
impl Pack for Message {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let data = array![];

        for token in self.iter() {
            data.push(&token.pack(pack_depth).await);
        }

        data.into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(&data).to_vec();
        let mut tokens = vec![];

        for token in data {
            if let Some(token) = MessageToken::unpack(&token, ArenaMut::clone(&arena)).await {
                tokens.push(*token);
            }
        }

        Some(Box::new(Message::from(tokens)))
    }
}

#[async_trait(?Send)]
impl Pack for MessageToken {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        match self {
            Self::Text(x) => (object! {
                "_tag": "Text",
                "_val": x.as_str()
            })
            .into(),
            Self::Reference(reference) => (object! {
                "_tag": "Refer",
                "_val": reference.pack(pack_depth).await
            })
            .into(),
            Self::Command(command) => (object! {
                "_tag": "CommandBlock",
                "_val": command.pack(pack_depth).await
            })
            .into(),
        }
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let tag = data.get("_tag")?.as_string()?;
        let val = data.get("_val")?;

        match tag.as_str() {
            "Text" => Some(Box::new(Self::Text(val.as_string()?))),
            "Refer" => Some(Box::new(Self::Reference(
                *Reference::unpack(&val, ArenaMut::clone(&arena)).await?,
            ))),
            "CommandBlock" => Some(Box::new(Self::Command(
                *Command::unpack(&val, ArenaMut::clone(&arena)).await?,
            ))),
            _ => None,
        }
    }
}

#[async_trait(?Send)]
impl Pack for Command {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let name = self.name.pack(pack_depth).await;

        let args = array![];
        for arg in &self.args {
            args.push(&arg.pack(pack_depth).await);
        }

        (object! {
            "name": name,
            "args": args,
            "text": self.text.pack(pack_depth).await
        })
        .into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let name = data.get("name")?;
        let args = data.get("args")?;
        let text = data.get("text")?;

        let name = *Message::unpack(&name, ArenaMut::clone(&arena)).await?;
        let text = *Message::unpack(&text, ArenaMut::clone(&arena)).await?;

        let raw_args = js_sys::Array::from(&args).to_vec();
        let mut args = vec![];
        for raw_arg in raw_args {
            if let Some(arg) = Argument::unpack(&raw_arg, ArenaMut::clone(&arena)).await {
                args.push(*arg);
            }
        }

        Some(Box::new(Self { name, args, text }))
    }
}

#[async_trait(?Send)]
impl Pack for Reference {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let pakced_name = js_sys::Array::new();
        for a_name in &self.name {
            pakced_name.push(&a_name.pack(pack_depth).await);
        }

        let packed_args = array![];
        for arg in &self.args {
            packed_args.push(&arg.pack(pack_depth).await);
        }

        let packed_option;
        if let Some(option) = &self.option {
            packed_option = option.pack(pack_depth).await;
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

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let name = data.get("name")?;
        let args = data.get("args")?;
        let option = data.get("option")?;

        let raw_name = js_sys::Array::from(&name).to_vec();
        let mut name = vec![];
        for a_name in raw_name {
            if let Some(a_name) = Message::unpack(&a_name, ArenaMut::clone(&arena)).await {
                name.push(*a_name);
            }
        }

        let raw_args = js_sys::Array::from(&args).to_vec();
        let mut args = vec![];
        for raw_arg in raw_args {
            if let Some(arg) = Argument::unpack(&raw_arg, ArenaMut::clone(&arena)).await {
                args.push(*arg);
            }
        }

        let option = if option.is_null() {
            None
        } else {
            Some(*Message::unpack(&option, ArenaMut::clone(&arena)).await?)
        };

        Some(Box::new(Self { name, args, option }))
    }
}

#[async_trait(?Send)]
impl Pack for Argument {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let packed = js_sys::Array::new();

        packed.push(&self.value.pack(pack_depth).await);
        if let Some(option) = &self.option {
            packed.push(&option.pack(pack_depth).await);
        }

        packed.into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(&data).to_vec();

        let value = *Message::unpack(&data[0], ArenaMut::clone(&arena)).await?;
        let option = if data.len() > 1 {
            Some(*Message::unpack(&data[1], ArenaMut::clone(&arena)).await?)
        } else {
            None
        };

        Some(Box::new(Self { value, option }))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SenderKind {
    Normal,
    System,
}

#[async_trait(?Send)]
impl Pack for SenderKind {
    async fn pack(&self, _: PackDepth) -> JsValue {
        match self {
            Self::Normal => JsValue::from("Normal"),
            Self::System => JsValue::from("System"),
        }
    }

    async fn unpack(data: &JsValue, _: ArenaMut) -> Option<Box<Self>> {
        match data.as_string()?.as_str() {
            "Normal" => Some(Box::new(Self::Normal)),
            "System" => Some(Box::new(Self::System)),
            _ => None,
        }
    }
}

block! {
    [pub Sender(constructor, pack)]
    (client_id): Rc<String>;
    (icon): Option<BlockRef<ImageData>>;
    (name): String;
    (kind): SenderKind;
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

    pub fn kind(&self) -> &SenderKind {
        &self.kind
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
