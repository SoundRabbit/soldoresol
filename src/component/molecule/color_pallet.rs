use super::atom::heading::{self, Heading};
use super::atom::slider::{self, Slider};
use crate::libs::color::{pallet, Pallet};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

pub struct Props {
    pub default_selected: Pallet,
    pub title: Option<String>,
    pub theme: slider::Theme,
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
}

impl Default for Props {
    fn default() -> Self {
        Self {
            default_selected: Pallet::gray(9).a(100),
            title: None,
            theme: slider::Theme::Dark,
        }
    }
}

impl Component for ColorPallet {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ColorPallet {
    fn constructor(props: &Props) -> Self {
        Self {
            selected: props.default_selected.clone(),
            default_selected: props.default_selected.clone(),
        }
    }
}

impl Update for ColorPallet {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        if self.default_selected != props.default_selected {
            self.default_selected = props.default_selected.clone();
            self.selected = props.default_selected.clone();
        }
        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetColor(pallet) => {
                self.selected = pallet;
                Cmd::sub(On::SelectColor(self.selected.clone()))
            }
        }
    }
}

impl Render for ColorPallet {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                props
                    .title
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
                Self::render_color_base(&props.theme, &self.selected),
                Html::div(
                    Attributes::new()
                        .class(Self::class("table"))
                        .class(Self::class(&format!("table--{}", &props.theme))),
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
                        theme: props.theme,
                    },
                    Sub::map({
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
    pub fn render_color_base<C: Component>(theme: &slider::Theme, pallet: &Pallet) -> Html<C> {
        Html::div(
            Attributes::new()
                .class(Self::class("color-base"))
                .class(Self::class(&format!("color-base--{}", theme))),
            Events::new(),
            vec![Html::div(
                Attributes::new()
                    .class(Self::class("color-sample"))
                    .style("background-color", pallet.to_string()),
                Events::new(),
                vec![],
            )],
        )
    }

    fn render_column(&self, kind: pallet::Kind) -> Html<Self> {
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

    fn render_cell(&self, pallet: Pallet, is_dark: bool) -> Html<Self> {
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
            }

            ".table" {
                "display": "grid";
                "grid-template-rows": "repeat(10, max-content)";
                "grid-auto-columns": "1fr";
                "grid-auto-flow": "column";
            }

            ".table--light" {
                "border": format!("0.1rem solid {}", Pallet::gray(9).a(100));
            }

            ".table--dark" {
                "border": format!("0.1rem solid {}", Pallet::gray(0).a(100));
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
            }

            ".color-base--light" {
                "border": format!("0.1rem solid {}", Pallet::gray(9).a(100));
            }

            ".color-base--dark" {
                "border": format!("0.1rem solid {}", Pallet::gray(0).a(100));
            }

            ".color-sample" {
                "width": "100%";
                "height": "100%";
            }
        }
    }
}
