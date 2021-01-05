use super::Msg;
use crate::Resource;
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(resource: &Resource) -> Html {
    common::select_image(resource, Msg::SetPersonalDataWithIconImageToCloseModal)
}
