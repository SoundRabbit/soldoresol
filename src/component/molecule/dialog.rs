use super::atom::{btn::Btn, fa};
use super::constant;
use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Dialog {}

pub enum Button {
    Ok(Events),
    Yes(Events),
    No(Events),
    Cancel(Events),
}

impl Component for Dialog {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Dialog {}

impl Constructor for Dialog {
    fn constructor(_props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for Dialog {}

impl Render<Html> for Dialog {
    type Children = (String, String, Vec<Button>);
    fn render(&self, (title, message, buttons): Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("background")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("header")),
                        Events::new(),
                        vec![Html::text(title)],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("body")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Self::class("message")),
                                Events::new(),
                                vec![Html::text(message)],
                            ),
                            Html::div(
                                Attributes::new().class(Self::class("buttons")),
                                Events::new(),
                                buttons
                                    .into_iter()
                                    .map(|button| match button {
                                        Button::Ok(events) => Btn::primary(
                                            Attributes::new(),
                                            events,
                                            vec![Html::text("OK")],
                                        ),
                                        Button::Yes(events) => Btn::primary(
                                            Attributes::new(),
                                            events,
                                            vec![Html::text("はい")],
                                        ),
                                        Button::No(events) => Btn::secondary(
                                            Attributes::new(),
                                            events,
                                            vec![Html::text("いいえ")],
                                        ),
                                        Button::Cancel(events) => Btn::secondary(
                                            Attributes::new(),
                                            events,
                                            vec![Html::text("キャンセル")],
                                        ),
                                    })
                                    .collect(),
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("footer")),
                        Events::new(),
                        vec![],
                    ),
                ],
            )],
        ))
    }
}

impl Styled for Dialog {
    fn style() -> Style {
        style! {
            ".background" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": constant::z_index::MODAL.to_string();
                "background-color": color_system::gray(13, 9).to_string();
                "display": "grid";
                "align-items": "center";
                "justify-items": "center";
            }
            ".base" {
                "max-width": "max-content";
                "min-height": "15%";
                "display": "grid";
                "grid-template-rows": "max-content 1fr max-content";
                "border-radius": "2px";
                "overflow": "hidden";
            }
            ".header" {
                "color": color_system::gray(100, 0).to_string();
                "background-color": color_system::gray(100, 8).to_string();
                "padding" : ".5em 1em";
            }
            ".body" {
                "background-color": color_system::gray(100, 0).to_string();
                "overflow": "hidden";
                "padding" : ".5em 1em";
                "display": "grid";
                "grid-template-rows": "1fr max-content";
                "row-gap": "1em";
            }
            ".message" {
                "white-space": "pre-wrap";
            }
            ".buttons" {
                "display": "grid";
                "grid-auto-columns": "max-content";
                "justify-content": "center";
                "grid-auto-flow": "column";
                "column-gap": ".5em";
            }
            ".footer" {
                "color": color_system::gray(100, 0).to_string();
                "background-color": color_system::gray(100, 8).to_string();
                "padding" : ".5em 1em";
            }
            @media "(orientation: portrait), (max-width: 60rem)" {
                ".base" {
                    "width": "95%";
                }
            }

            @media "(max-height: 60rem)" {
                ".base" {
                    "height": "80%";
                }
            }
        }
    }
}
