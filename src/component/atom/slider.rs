use crate::libs::color::color_system;
use component::{Cmd, Sub};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub position: Position,
    pub range_is_editable: bool,
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

pub struct Slider {}

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
            position: Position::Linear {
                min: 0.0,
                max: 100.0,
                val: 50.0,
                step: 1.0,
            },
            range_is_editable: true,
        }
    }
}

impl Component for Slider {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Slider {
    fn constructor(_: &Props) -> Self {
        Self {}
    }
}

impl Update for Slider {
    fn update(&mut self, props: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::Sub(sub),
            Msg::SetValue(input_val) => {
                let new_val = match &props.position {
                    Position::Linear { .. } => input_val,
                    Position::Inf { .. } => input_val,
                };

                Cmd::Sub(On::Input(new_val))
            }
            Msg::InputSliderValue(input_val) => {
                let new_val = match &props.position {
                    Position::Linear { .. } => input_val,
                    Position::Inf { mid, step, .. } => {
                        let val = -(*mid) * (1.0 - input_val.min(1.0 - 1e-15).max(0.0)).log2();
                        let val = (*step) * (val / *step).ceil();
                        val
                    }
                };
                Cmd::Sub(On::Input(new_val))
            }
        }
    }
}

impl Render for Slider {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        let (min, max, val, slider_val, step, slider_step) = match &props.position {
            Position::Linear {
                min,
                max,
                val,
                step,
            } => (*min, *max, *val, *val, *step, *step),
            Position::Inf { val, mid, step } => {
                let val = val.max(0.0);
                let prev = (val - step).max(0.0);
                let next = (val + step).max(0.0);
                (
                    0.0_f64,
                    1.0_f64,
                    val,
                    1.0_f64 - (0.5_f64).powf(val / mid),
                    *step,
                    ((0.5_f64).powf(prev / mid) - (0.5_f64).powf(next / mid)) / 2.0,
                )
            }
        };

        let pos = ((slider_val - min) / (max - min)).min(1.0).max(0.0);

        Self::styled(Html::div(
            Attributes::new().class("pure-form").class(Self::class("base")),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new()
                        .type_("range")
                        .string("min", min.to_string())
                        .string("max", max.to_string())
                        .string("step", slider_step.to_string())
                        .value(format!("{}", slider_val))
                        .class(Self::class("slider"))
                        .style(
                            "background",
                            format!(
                                "linear-gradient(to right, {} calc({}% - {}em + 0.5em), {} {}% 100%)",
                                color_system::blue(255, 5),
                                pos * 100.0,
                                pos,
                                color_system::gray(255, 3),
                                pos * 100.0
                            ),
                        ),
                    Events::new()
                        .on_input(|val| {
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
                    Events::new()
                        .on_input(|val| {
                                val.parse()
                                .map(|val| Msg::SetValue(val))
                                .unwrap_or(Msg::NoOp)
                        }),
                    vec![],
                ),
                match &props.position {
                    Position::Linear {min,max, ..} => self.render_range_linear(props,*min, *max, ),
                    Position::Inf {mid, ..} => self.render_range_inf(props,*mid)
                }
            ],
        ))
    }
}

impl Slider {
    fn render_range_linear(&self, props: &Props, min: f64, max: f64) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("range"))
                .class(Self::class("range--linear")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new()
                        .class(Self::class("allow"))
                        .class(Self::class("allow--left")),
                    Events::new(),
                    vec![Html::input(
                        {
                            let attrs = Attributes::new()
                                .type_("number")
                                .value(format!("{}", min))
                                .class(Self::class("input"));
                            if props.range_is_editable {
                                attrs
                            } else {
                                attrs.flag("readonly")
                            }
                        },
                        Events::new().on_input(move |min| {
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
                        .class(Self::class("allow--right")),
                    Events::new(),
                    vec![Html::input(
                        {
                            let attrs = Attributes::new()
                                .type_("number")
                                .value(format!("{}", max))
                                .class(Self::class("input"));
                            if props.range_is_editable {
                                attrs
                            } else {
                                attrs.flag("readonly")
                            }
                        },
                        Events::new().on_input(move |max| {
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
    fn render_range_inf(&self, props: &Props, mid: f64) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("range"))
                .class(Self::class("range--inf")),
            Events::new(),
            vec![Html::div(
                Attributes::new()
                    .class(Self::class("allow"))
                    .class(Self::class("allow--center")),
                Events::new(),
                vec![Html::input(
                    {
                        let attrs = Attributes::new()
                            .type_("number")
                            .value(format!("{}", mid))
                            .class(Self::class("input"));
                        if props.range_is_editable {
                            attrs
                        } else {
                            attrs.flag("readonly")
                        }
                    },
                    Events::new().on_input(move |mid| {
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
            }

            ".input" {
                "width": "5em";
                "outline": "none";
            }

            ".slider" {
                "width": "100%";
                "height": "1em";
                "-webkit-appearance": "none";
                "appearance": "none";
                "border-radius": "0.5em";
                "outline": "none";
                "font-size": "1.3em";
            }

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
