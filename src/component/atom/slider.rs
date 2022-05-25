use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub range_is_editable: bool,
    pub theme: Theme,
}

#[derive(Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetValue(f64),
    InputSliderValue(f64),
}

pub enum On {
    Input(f64),
    InputRange { min: f64, max: f64 },
    InputMid(f64),
}

pub struct Slider {
    position: Position,
}

pub enum Position {
    Linear {
        min: f64,
        max: f64,
        val: f64,
        step: f64,
    },
    Inf {
        val: f64,
        mid: f64,
        step: f64,
    },
}

impl Default for Props {
    fn default() -> Self {
        Self {
            range_is_editable: true,
            theme: Theme::Dark,
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "light"),
            Self::Dark => write!(f, "dark"),
        }
    }
}

impl Component for Slider {
    type Props = Position;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Slider {}

impl Constructor for Slider {
    fn constructor(position: Position) -> Self {
        Self { position }
    }
}

impl Update for Slider {
    fn on_load(mut self: Pin<&mut Self>, position: Self::Props) -> Cmd<Self> {
        self.position = position;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetValue(input_val) => {
                let new_val = match &self.position {
                    Position::Linear { .. } => input_val,
                    Position::Inf { .. } => input_val,
                };

                Cmd::submit(On::Input(new_val))
            }
            Msg::InputSliderValue(input_val) => {
                let new_val = match &self.position {
                    Position::Linear { .. } => input_val,
                    Position::Inf { mid, step, .. } => {
                        let val = Self::inv_sigmoid(input_val, *mid);
                        let val = (*step) * (val / *step).ceil();
                        val
                    }
                };
                Cmd::submit(On::Input(new_val))
            }
        }
    }
}

impl Render<Html> for Slider {
    type Children = Props;
    fn render(&self, props: Self::Children) -> Html {
        let (min, max, val, slider_val, step, slider_step) = match &self.position {
            Position::Linear {
                min,
                max,
                val,
                step,
            } => (*min, *max, *val, *val, *step, *step),
            Position::Inf { val, mid, step } => (
                0.0,
                100.0,
                *val,
                Self::sigmoid(*val, *mid),
                *step,
                (Self::sigmoid(*val + *step, *mid) - Self::sigmoid(*val - *step, *mid)) / 2.0,
            ),
        };

        // let pos = ((slider_val - min) / (max - min)).min(1.0).max(0.0);

        Self::styled(Html::div(
            Attributes::new()
                .class("pure-form")
                .class(Self::class("base")),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new()
                        .type_("range")
                        .string("min", min.to_string())
                        .string("max", max.to_string())
                        .string("step", slider_step.to_string())
                        .value(format!("{}", slider_val))
                        .class(Self::class("slider")),
                    // .style(
                    //     "background",
                    //     format!(
                    //         "linear-gradient(to right, {} calc({}% - {}em + 0.5em), {} {}% 100%)",
                    //         color_system::blue(255, 5),
                    //         pos * 100.0,
                    //         pos,
                    //         color_system::gray(255, 3),
                    //         pos * 100.0
                    //     ),
                    // ),
                    Events::new().on_input(self, |val| {
                        val.parse()
                            .map(|val| Msg::InputSliderValue(val))
                            .unwrap_or(Msg::NoOp)
                    }),
                    vec![],
                ),
                Html::input(
                    Attributes::new()
                        .type_("number")
                        .string("step", format!("{}", step))
                        .value(format!("{}", val))
                        .class(Self::class("input"))
                        .class(Self::class("value")),
                    Events::new().on_input(self, |val| {
                        val.parse()
                            .map(|val| Msg::SetValue(val))
                            .unwrap_or(Msg::NoOp)
                    }),
                    vec![],
                ),
                match &self.position {
                    Position::Linear { min, max, .. } => {
                        self.render_range_linear(&props, *min, *max)
                    }
                    Position::Inf { mid, .. } => self.render_range_inf(&props, *mid),
                },
            ],
        ))
    }
}

impl Slider {
    fn sigmoid(val: f64, mid: f64) -> f64 {
        100.0 * 1.0 / (1.0 + f64::exp(1.0 / mid.abs().max(1.0) * (mid - val)))
    }

    fn inv_sigmoid(val: f64, mid: f64) -> f64 {
        mid - mid * f64::ln(100.0 / val - 1.0)
    }

    fn render_range_linear(&self, props: &Props, min: f64, max: f64) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("range"))
                .class(Self::class("range--linear")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new()
                        .class(Self::class("allow"))
                        .class(Self::class("allow--left"))
                        .class(Self::class(&format!("allow--{}", props.theme))),
                    Events::new(),
                    vec![Html::input(
                        Attributes::new()
                            .type_("number")
                            .value(format!("{}", min))
                            .class(Self::class("input"))
                            .flag("readonly", !props.range_is_editable),
                        Events::new().on_input(self, move |min| {
                            if let Ok(min) = min.parse::<f64>() {
                                Msg::Sub(On::InputRange { min, max })
                            } else {
                                Msg::NoOp
                            }
                        }),
                        vec![],
                    )],
                ),
                Html::div(
                    Attributes::new()
                        .class(Self::class("allow"))
                        .class(Self::class("allow--right"))
                        .class(Self::class(&format!("allow--{}", props.theme))),
                    Events::new(),
                    vec![Html::input(
                        Attributes::new()
                            .type_("number")
                            .value(format!("{}", max))
                            .class(Self::class("input"))
                            .flag("readonly", !props.range_is_editable),
                        Events::new().on_input(self, move |max| {
                            if let Ok(max) = max.parse::<f64>() {
                                Msg::Sub(On::InputRange { min, max })
                            } else {
                                Msg::NoOp
                            }
                        }),
                        vec![],
                    )],
                ),
            ],
        )
    }
    fn render_range_inf(&self, props: &Props, mid: f64) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("range"))
                .class(Self::class("range--inf")),
            Events::new(),
            vec![Html::div(
                Attributes::new()
                    .class(Self::class("allow"))
                    .class(Self::class("allow--center"))
                    .class(Self::class(&format!("allow--{}", props.theme))),
                Events::new(),
                vec![Html::input(
                    Attributes::new()
                        .type_("number")
                        .value(format!("{}", mid))
                        .class(Self::class("input"))
                        .flag("readonly", !props.range_is_editable),
                    Events::new().on_input(self, move |mid| {
                        if let Ok(mid) = mid.parse::<f64>() {
                            Msg::Sub(On::InputMid(mid))
                        } else {
                            Msg::NoOp
                        }
                    }),
                    vec![],
                )],
            )],
        )
    }
}

impl Styled for Slider {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "grid-auto-rows": "max-content";
                "align-items": "center";
                "column-gap": "0.35em";
                "font-size": ".85em";
            }

            ".input" {
                "width": "5em";
                "outline": "none";
            }

            // ".slider" {
            //     "width": "100%";
            //     "height": "1em";
            //     "-webkit-appearance": "none";
            //     "appearance": "none";
            //     "border-radius": "0.5em";
            //     "outline": "none";
            //     "font-size": "1.3em";
            // }

            ".slider::-webkit-slider-thumb" {
                "-webkit-appearance": "none";
                "appearance": "none";
                "background-color": format!("{}", color_system::gray(100, 5));
                "width": "1em";
                "height": "1em";
                "border-radius": "50%";
            }

            ".range" {
                "display": "flex";
                "align-items": "center";
            }

            ".range--linear" {
                "justify-content": "space-between";
            }

            ".range--inf" {
                "justify-content": "center";
            }

            ".allow" {
                "position": "relative";
            }

            ".allow:before" {
                "content":"\"\"";
                "position": "absolute";
                "border": "0.5em solid transparent";
            }

            ".allow--light:before" {
                "border-bottom": format!("0.5em solid {}", color_system::gray(100, 9));
            }

            ".allow--dark:before" {
                "border-bottom": format!("0.5em solid {}", color_system::gray(100, 0));
            }

            ".allow--left:before" {
                "top": "-1em";
                "left": "0";
            }

            ".allow--center:before" {
                "top": "-1em";
                "left": "calc(50% - 0.5em)";
            }

            ".allow--right:before" {
                "top": "-1em";
                "left": "calc(100% - 1em)";
            }
        }
    }
}
