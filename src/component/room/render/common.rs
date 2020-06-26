use super::{super::icon, Msg};
use crate::model::{Icon, Resource};
use kagura::prelude::*;

pub fn chat_icon(attrs: Attributes, icon: &Icon, alt: &str, resource: &Resource) -> Html<Msg> {
    match icon {
        Icon::None => icon::none(attrs),
        Icon::Resource(r_id) => {
            if let Some(img_url) = resource.get_as_image_url(&r_id) {
                icon::from_img(attrs, img_url.as_str())
            } else {
                icon::from_str(attrs, alt)
            }
        }
        _ => icon::from_str(attrs, &alt),
    }
}
