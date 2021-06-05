use crate::libs::color::color_system;
use crate::libs::type_id::type_id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct TabBtn {}

impl TabBtn {
    pub fn new(
        is_draggable: bool,
        is_selected: bool,
        attrs: Attributes,
        events: Events,
        children: Vec<Html>,
    ) -> Html {
        Self::styled(Html::div(
            attrs.class(Self::class("base")).draggable(is_draggable),
            events,
            vec![Html::div(
                Attributes::new()
                    .class("pure-button")
                    .class(Self::class("btn"))
                    .string("data-tab-selected", is_selected.to_string()),
                Events::new(),
                children,
            )],
        ))
    }

    pub fn id<T>(suffix: Vec<&str>) -> String {
        let mut id = type_id::<Self>() + ";" + &type_id::<T>();

        for s in suffix {
            id += ";";
            id += s;
        }

        id
    }

    pub fn validate_id(id: &str) -> bool {
        let id = id.split(";").collect::<Vec<_>>();
        if id.len() > 1 {
            id[0] == type_id::<Self>()
        } else {
            false
        }
    }

    pub fn validate_prefix<T>(id: &str) -> bool {
        if Self::validate_id(id) {
            let id = id.split(";").collect::<Vec<_>>();
            id[1] == type_id::<T>()
        } else {
            false
        }
    }

    pub fn get_suffix(id: &str) -> Vec<&str> {
        if Self::validate_id(id) {
            let mut id = id.split(";").collect::<Vec<_>>();
            id.drain(2..).collect()
        } else {
            vec![]
        }
    }
}

impl Styled for TabBtn {
    fn style() -> Style {
        style! {
            ".base" {
                "max-width": "max-content";
                "min-width": "max-content";
                "max-height": "max-content";
                "min-height": "max-content";
            }

            ".btn" {
                "border-radius": "2px 2px 0 0";
                "color": color_system::gray(100, 0).to_string();
                "max-width": "12em";
                "overflow": "hidden";
                "text-overflow": "ellipsis";
            }

            r#".btn[data-tab-selected="true"]"# {
                "background-color": color_system::blue(100, 5).to_string();
            }

            r#".btn[data-tab-selected="false"]"# {
                "background-color": color_system::gray(100, 9).to_string();
            }
        }
    }
}
