use super::atom::common::Common;
use super::atom::heading::{self, Heading};
use super::atom::slider::{self, Slider};
use crate::libs::color::{pallet, Pallet};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

struct Children {
    pub title: Option<String>,
    pub theme: slider::Theme,
}

pub type Props = Children;

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

impl Default for Children {
    fn default() -> Self {
        Self {
            title: None,
            theme: slider::Theme::Dark,
        }
    }
}

impl Component for ColorPallet {
    type Props = Pallet;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ColorPallet {}

impl Constructor for ColorPallet {
    fn constructor(default_selected: Pallet) -> Self {
        Self {
            selected: default_selected.clone(),
            default_selected: default_selected.clone(),
        }
    }
}

impl Update for ColorPallet {
    fn on_load(&mut self, default_selected: Pallet) -> Cmd<Self> {
        if self.default_selected != default_selected {
            self.default_selected = default_selected.clone();
            self.selected = default_selected.clone();
        }
        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetColor(pallet) => {
                self.selected = pallet;
                Cmd::submit(On::SelectColor(self.selected.clone()))
            }
        }
    }
}

impl Render<Html> for ColorPallet {
    type Children = Children;
    fn render(&self, props: Self::Children) -> Html {
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
                Slider::new(
                    self,
                    None,
                    slider::Position::Linear {
                        min: 0.0,
                        max: 100.0,
                        val: self.selected.alpha as f64,
                        step: 1.0,
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
                    slider::Props {
                        range_is_editable: false,
                        theme: props.theme,
                    },
                ),
            ],
        ))
    }
}

impl ColorPallet {
    pub fn render_color_base(theme: &slider::Theme, pallet: &Pallet) -> Html {
        Html::div(
            Attributes::new()
                .class(Common::bg_transparent())
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
            Events::new().on_click(self, move |_| Msg::SetColor(pallet)),
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
