use super::atom::heading::{self, Heading};
use super::atom::slider::{self, Slider};
use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use crate::libs::color::Color;
use kagura::prelude::*;

pub struct Props {
    pub alpha: u8,
    pub default_selected: Pallet,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Pallet {
    Gray(usize),
    Red(usize),
    Orange(usize),
    Yellow(usize),
    Green(usize),
    Blue(usize),
    Purple(usize),
    Pink(usize),
}

pub enum Msg {
    SetAlpha(u8),
    SetColor(Pallet),
}

pub enum On {
    SetColor(Pallet, u8),
}

pub struct ColorPallet {
    alpha: u8,
    selected: Pallet,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            alpha: 100,
            default_selected: Pallet::Gray(9),
        }
    }
}

impl Pallet {
    fn to_color(&self) -> Color {
        self.color(100)
    }

    pub fn color(&self, alpha: u8) -> Color {
        match self {
            Pallet::Gray(idx) => color_system::gray(alpha, *idx),
            Pallet::Red(idx) => color_system::red(alpha, *idx),
            Pallet::Orange(idx) => color_system::orange(alpha, *idx),
            Pallet::Yellow(idx) => color_system::yellow(alpha, *idx),
            Pallet::Green(idx) => color_system::green(alpha, *idx),
            Pallet::Blue(idx) => color_system::blue(alpha, *idx),
            Pallet::Purple(idx) => color_system::purple(alpha, *idx),
            Pallet::Pink(idx) => color_system::pink(alpha, *idx),
        }
    }
}

impl Constructor for ColorPallet {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            alpha: props.alpha,
            selected: props.default_selected,
        }
    }
}

impl Component for ColorPallet {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        // self.alpha = props.alpha;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetAlpha(alpha) => {
                self.alpha = alpha;
                Cmd::sub(On::SetColor(self.selected.clone(), self.alpha))
            }
            Msg::SetColor(pallet) => {
                self.selected = pallet;
                Cmd::sub(On::SetColor(self.selected.clone(), self.alpha))
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Heading::with_child(
                    heading::Props { level: 5 },
                    Subscription::none(),
                    Html::text("透明度"),
                ),
                Slider::empty(
                    slider::Props {
                        position: slider::Position::Linear {
                            min: 0.0,
                            max: 100.0,
                            val: self.alpha as f64,
                            step: 1.0,
                        },
                        range_is_editable: false,
                    },
                    Subscription::new(|sub| match sub {
                        slider::On::Input(alpha) => Msg::SetAlpha(alpha.round() as u8),
                    }),
                ),
                Html::div(
                    Attributes::new().class(Self::class("table")),
                    Events::new(),
                    vec![
                        self.render_column(Pallet::Gray),
                        self.render_column(Pallet::Red),
                        self.render_column(Pallet::Orange),
                        self.render_column(Pallet::Yellow),
                        self.render_column(Pallet::Green),
                        self.render_column(Pallet::Blue),
                        self.render_column(Pallet::Purple),
                        self.render_column(Pallet::Pink),
                    ],
                ),
            ],
        ))
    }
}

impl ColorPallet {
    fn render_column(&self, pallet_gen: fn(usize) -> Pallet) -> Html {
        let mut cells = vec![];

        for idx in 0..10 {
            cells.push(self.render_cell(pallet_gen(idx), idx >= 6));
        }

        Html::fragment(cells)
    }

    fn render_cell(&self, pallet: Pallet, is_dark: bool) -> Html {
        let color = pallet.to_color();

        let attrs = Attributes::new()
            .class(Self::class("cell"))
            .style("background-color", color.to_string());

        let attrs = if self.selected == pallet {
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
            "base" {
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": "0.35em";
            }

            "table" {
                "display": "grid";
                "grid-template-rows": "repeat(10, max-content)";
                "grid-auto-columns": "1fr";
                "grid-auto-flow": "column";
            }

            "cell" {
                "min-width": "2rem";
                "max-width": "100%";
                "height": "2rem";
            }

            "cell:hover" {
                "cursor": "pointer";
            }

            "cell--selected-dark" {
                "box-shadow": format!("0 0 0.1em 0.1em {} inset", color_system::gray(100, 9));
            }

            "cell--selected-light" {
                "box-shadow": format!("0 0 0.1em 0.1em {} inset", color_system::gray(100, 0));
            }
        }
    }
}
