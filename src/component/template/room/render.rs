use super::super::atom::{
    btn::{self, Btn},
    card::{self, Card},
    dropdown::{self, Dropdown},
    header::{self, Header},
    heading::{self, Heading},
};
use super::super::template::basic_app::{self, BasicApp};
use super::*;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Sub;

impl Render for Room {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(BasicApp::with_children(
            basic_app::Props {},
            Sub::none(),
            vec![
                Header::with_children(header::Props::new(), Sub::none(), vec![]),
                Html::div(
                    Attributes::new().class(Self::class("body")),
                    Events::new(),
                    vec![self.modeless_container.with_children(
                        tab_modeless_container::Props {},
                        Sub::none(),
                        vec![],
                    )],
                ),
            ],
        ))
    }
}

impl Styled for Room {
    fn style() -> Style {
        style! {}
    }
}
