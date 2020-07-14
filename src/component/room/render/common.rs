use super::super::super::icon;
use super::Msg;
use crate::{block::chat::item::Icon, resource::Data, Resource};
use kagura::prelude::*;

pub fn chat_icon(attrs: Attributes, icon: &Icon, alt: &str, resource: &Resource) -> Html {
    match icon {
        Icon::None => icon::none(attrs),
        Icon::Resource(r_id) => {
            if let Some(Data::Image { url: img_url, .. }) = resource.get(&r_id) {
                icon::from_img(attrs, img_url.as_str())
            } else {
                icon::from_str(attrs, alt)
            }
        }
        _ => icon::from_str(attrs, &alt),
    }
}
