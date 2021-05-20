use super::atom::heading::{self, Heading};
use super::atom::slider::{self, Slider};
use crate::libs::color::{pallet, Pallet};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub default_selected: Pallet,
    pub title: Option<String>,
}

pub enum Msg {
    NoOp,
    SetColor(Pallet),
}

pub enum On {
    SelectColor(Pallet),
}

pub struct ColorPallet {
    selected: Pallet,
    default_selected: Pallet,
    title: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            default_selected: Pallet::gray(9).a(100),
            title: None,
        }
    }
}

impl Constructor for ColorPallet {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            selected: props.default_selected.clone(),
            default_selected: props.default_selected.clone(),
            title: props.title,
        }
    }
}

impl Component for ColorPallet {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        if self.default_selected != props.default_selected {
            self.default_selected = props.default_selected.clone();
            self.selected = props.default_selected.clone();
        }

        self.title = props.title;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetColor(pallet) => {
                self.selected = pallet;
                Cmd::sub(On::SelectColor(self.selected.clone()))
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                self.title
                    .as_ref()
                    .map(|title| {
                        Heading::h4(
                            heading::Variant::Dark,
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text(title)],
                        )
                    })
                    .unwrap_or(Html::none()),
                Html::div(
                    Attributes::new().class(Self::class("color-base")),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new()
                            .class(Self::class("color-sample"))
                            .style("background-color", self.selected.to_string()),
                        Events::new(),
                        vec![],
                    )],
                ),
                Html::div(
                    Attributes::new().class(Self::class("table")),
                    Events::new(),
                    vec![
                        self.render_column(pallet::Kind::Gray),
                        self.render_column(pallet::Kind::Red),
                        self.render_column(pallet::Kind::Orange),
                        self.render_column(pallet::Kind::Yellow),
                        self.render_column(pallet::Kind::Green),
                        self.render_column(pallet::Kind::Blue),
                        self.render_column(pallet::Kind::Purple),
                        self.render_column(pallet::Kind::Pink),
                    ],
                ),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            min: 0.0,
                            max: 100.0,
                            val: self.selected.alpha as f64,
                            step: 1.0,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new({
                        let selected = self.selected.clone();
                        move |sub| match sub {
                            slider::On::Input(alpha) => {
                                Msg::SetColor(selected.a(alpha.round() as u8))
                            }
                            _ => Msg::NoOp,
                        }
                    }),
                ),
            ],
        ))
    }
}

impl ColorPallet {
    fn render_column(&self, kind: pallet::Kind) -> Html {
        let mut cells = vec![];
        let mut pallet = Pallet {
            alpha: self.selected.alpha,
            idx: 0,
            kind: kind,
        };

        for idx in 0..10 {
            pallet.idx = idx;
            cells.push(self.render_cell(pallet, idx >= 6));
        }

        Html::fragment(cells)
    }

    fn render_cell(&self, pallet: Pallet, is_dark: bool) -> Html {
        let color = pallet.clone().a(100).to_color();

        let attrs = Attributes::new()
            .class(Self::class("cell"))
            .style("background-color", color.to_string());

        let attrs = if self.selected.kind == pallet.kind && self.selected.idx == pallet.idx {
            if is_dark {
                attrs.class(Self::class("cell--selected-light"))
            } else {
                attrs.class(Self::class("cell--selected-dark"))
            }
        } else {
            attrs
        };

        Html::div(
            attrs,
            Events::new().on_click(move |_| Msg::SetColor(pallet)),
            vec![],
        )
    }
}

impl Styled for ColorPallet {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": "0.35em";
                "background-color": Pallet::gray(8).a(100).to_string();
                "padding": "0.35em";
            }

            ".table" {
                "display": "grid";
                "grid-template-rows": "repeat(10, max-content)";
                "grid-auto-columns": "1fr";
                "grid-auto-flow": "column";
            }

            ".cell" {
                "min-width": "2rem";
                "max-width": "100%";
                "height": "2rem";
            }

            ".cell:hover" {
                "cursor": "pointer";
            }

            ".cell--selected-dark" {
                "box-shadow": format!("0 0 0.1em 0.1em {} inset", Pallet::gray(9).a(100));
            }

            ".cell--selected-light" {
                "box-shadow": format!("0 0 0.1em 0.1em {} inset", Pallet::gray(0).a(100));
            }

            ".color-base" {
                "width": "100%";
                "height": "2rem";
                "background-color": format!("{}", Pallet::gray(2).a(100));
                "background-image": "linear-gradient(45deg,  #fff 25%, #fff 25%, transparent 25%, transparent 75%, #fff 75%, #fff 75%),
                    linear-gradient(-135deg, #fff 25%, #fff 25%, transparent 25%, transparent 75%, #fff 75%, #fff 75%)";
                "background-size": "1rem 1rem";
                "background-position": "0 0, 0.5rem 0.5rem";
                "border": format!("0.1rem solid {}", Pallet::gray(0).a(100));
            }

            ".color-sample" {
                "width": "100%";
                "height": "100%";
            }
        }
    }
}
