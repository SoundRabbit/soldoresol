use crate::JsObject;

pub enum Icon {
    None,
    Resource(u128),
    DefaultUser,
}

impl Icon {
    pub fn as_object(&self) -> JsObject {
        match self {
            Icon::None => object! {type: "None"},
            Icon::Resource(resource_id) => {
                object! {type: "Resource", payload: resource_id.to_string()}
            }
            Icon::DefaultUser => object! {type: "DefaultUser"},
        }
    }
}

impl From<JsObject> for Icon {
    fn from(object: JsObject) -> Self {
        let icon_type = object
            .get("type")
            .and_then(|x| x.as_string())
            .unwrap_or(String::from(""));
        match icon_type.as_str() {
            "Resource" => {
                if let Some(resource_id) = object
                    .get("payload")
                    .and_then(|x| x.as_string())
                    .and_then(|x| x.parse().ok())
                {
                    Icon::Resource(resource_id)
                } else {
                    Icon::None
                }
            }
            "DefaultUser" => Icon::DefaultUser,
            _ => Icon::None,
        }
    }
}
