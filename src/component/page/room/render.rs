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
                Header::with_children(
                    header::Props::new(),
                    Sub::none(),
                    vec![self.render_header_row_0()],
                ),
                Html::div(
                    Attributes::new().class(Self::class("body")),
                    Events::new(),
                    vec![self.modeless_container.with_children(
                        tab_modeless_container::Props {},
                        Sub::map(|sub| match sub {
                            tab_modeless_container::On::Sub(..) => Msg::NoOp,
                        }),
                        vec![],
                    )],
                ),
            ],
        ))
    }
}

impl Room {
    fn render_header_row_0(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_header_row_0_left(),
                Html::div(
                    Attributes::new().class(Self::class("right")),
                    Events::new(),
                    vec![],
                ),
            ],
        )
    }

    fn render_header_row_0_left(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("view-room-id")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new().class(Self::class("label")),
                    Events::new(),
                    vec![Html::text("ルームID")],
                ),
                Html::input(Attributes::new().flag("readonly"), Events::new(), vec![]),
            ],
        )
    }
}

impl Styled for Room {
    fn style() -> Style {
        style! {
            ".header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            ".view-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "column-gap": "0.65em";
            }

            ".label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }
        }
    }
}
