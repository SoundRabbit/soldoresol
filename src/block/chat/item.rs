use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, resource::ResourceId, JsObject, Promise};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub enum Icon {
    None,
    Resource(ResourceId),
    DefaultUser,
}

#[derive(Clone)]
pub enum Sender {
    System,
    User,
    Character(BlockId),
}

#[derive(Clone)]
pub struct Item {
    peer_id: String,
    display_name: String,
    icon: Icon,
    sender: Sender,
    text: String,
    reply: Option<String>,
}

impl Sender {
    pub fn as_character(&self) -> Option<&BlockId> {
        match self {
            Self::Character(block_id) => Some(block_id),
            _ => None,
        }
    }
}

impl Icon {
    pub fn to_jsobject(&self) -> JsObject {
        match self {
            Self::None => object! {
                type: "None"
            },
            Self::DefaultUser => object! {
                type: "DefaultUser"
            },
            Self::Resource(r_id) => object! {
                type: "Resource",
                payload: r_id.to_jsvalue()
            },
        }
    }

    pub fn from_jsobject(val: JsObject) -> Option<Self> {
        val.get("type")
            .and_then(|t| t.as_string())
            .and_then(|t| match t.as_str() {
                "None" => Some(Self::None),
                "DefaultUser" => Some(Self::DefaultUser),
                "Resource" => val
                    .get("payload")
                    .and_then(|p| U128Id::from_jsvalue(&p))
                    .map(|p| Self::Resource(p)),
                _ => None,
            })
    }
}

impl Sender {
    pub fn to_jsobject(&self) -> JsObject {
        match self {
            Self::System => object! {
                type: "System"
            },
            Self::User => object! {
                type: "User"
            },
            Self::Character(c_id) => object! {
                type: "Character",
                payload: c_id.to_jsvalue()
            },
        }
    }

    pub fn from_jsobject(field: &mut Field, val: JsObject) -> Option<Self> {
        val.get("type")
            .and_then(|t| t.as_string())
            .and_then(|t| match t.as_str() {
                "System" => Some(Self::System),
                "User" => Some(Self::User),
                "Character" => val
                    .get("payload")
                    .and_then(|p| U128Id::from_jsvalue(&p))
                    .map(|p| Self::Character(field.block_id(p))),
                _ => None,
            })
    }
}

impl Item {
    pub fn new(
        peer_id: String,
        display_name: String,
        icon: Icon,
        sender: Sender,
        text: String,
        reply: Option<String>,
    ) -> Self {
        Self {
            peer_id,
            display_name,
            icon,
            sender,
            text,
            reply,
        }
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn peer_id(&self) -> &String {
        &self.peer_id
    }

    pub fn icon(&self) -> &Icon {
        &self.icon
    }

    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn reply(&self) -> Option<&String> {
        self.reply.as_ref()
    }
}

impl Block for Item {
    fn pack(&self) -> Promise<JsValue> {
        let icon: js_sys::Object = self.icon.to_jsobject().into();
        let sender: js_sys::Object = self.sender.to_jsobject().into();

        let data = object! {
            peer_id: &self.peer_id,
            display_name: &self.display_name,
            icon: icon,
            sender: sender,
            text: &self.text,
            reply: self.reply.as_ref()
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(move |resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let peer_id = val.get("peer_id").and_then(|x| x.as_string());
            let display_name = val.get("display_name").and_then(|x| x.as_string());
            let icon = val.get("icon").and_then(|x| Icon::from_jsobject(x));
            let sender = val
                .get("sender")
                .and_then(|x| Sender::from_jsobject(field, x));
            let text = val.get("text").and_then(|x| x.as_string());
            let reply = Some(val.get("reply").and_then(|x| x.as_string()));
            if let (
                Some(peer_id),
                Some(display_name),
                Some(icon),
                Some(sender),
                Some(text),
                Some(reply),
            ) = (peer_id, display_name, icon, sender, text, reply)
            {
                Some(Box::new(Self {
                    peer_id,
                    display_name,
                    icon,
                    sender,
                    text,
                    reply,
                }))
            } else {
                None
            }
        } else {
            None
        };
        Promise::new(move |resolve| resolve(self_))
    }
}
