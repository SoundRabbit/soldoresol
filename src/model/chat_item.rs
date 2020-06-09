use super::Icon;
use crate::JsObject;

pub struct ChatItem {
    display_name: String,
    peer_id: String,
    character_id: Option<u128>,
    icon: Icon,
    payload: String,
}

impl ChatItem {
    pub fn new(
        display_name: String,
        peer_id: String,
        character_id: Option<u128>,
        icon: Icon,
        payload: String,
    ) -> Self {
        Self {
            display_name: display_name,
            peer_id: peer_id,
            character_id,
            icon: icon,
            payload: payload,
        }
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn peer_id(&self) -> &String {
        &self.peer_id
    }

    pub fn character_id(&self) -> Option<u128> {
        self.character_id.clone()
    }

    pub fn icon(&self) -> &Icon {
        &self.icon
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }

    pub fn as_object(&self) -> JsObject {
        object! {
            display_name: &self.display_name,
            peer_id: &self.peer_id,
            character_id: self.character_id.map(|x| x.to_string()),
            icon: self.icon.as_object(),
            payload: &self.payload
        }
    }
}

impl From<JsObject> for ChatItem {
    fn from(object: JsObject) -> Self {
        let display_name = object
            .get("display_name")
            .and_then(|x| x.as_string())
            .unwrap_or(String::from(""));
        let peer_id = object
            .get("peer_id")
            .and_then(|x| x.as_string())
            .unwrap_or(String::from(""));
        let character_id = object
            .get("character_id")
            .and_then(|x| x.as_string())
            .and_then(|x| x.parse().ok());
        let icon = object
            .get("icon")
            .map(|x| Icon::from(x))
            .unwrap_or(Icon::None);
        let payload = object
            .get("payload")
            .and_then(|x| x.as_string())
            .unwrap_or(String::from(""));

        Self {
            display_name,
            peer_id,
            character_id,
            icon,
            payload,
        }
    }
}
