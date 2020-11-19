use super::util::styled::{Style, Styled};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub position: Position,
    pub range_is_editable: bool,
}

pub enum Msg {
    NoOp,
    SetValue(f64),
    InputSliderValue(f64),
}

pub enum On {
    Input(f64),
}

pub struct Slider {
    position: Position,
    range_is_editable: bool,
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

impl Constructor for Slider {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            position: props.position,
            range_is_editable: props.range_is_editable,
        }
    }
}

impl Component for Slider {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.position = props.position;
        self.range_is_editable = props.range_is_editable;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetValue(input_val) => {
                let new_val = match &mut self.position {
                    Position::Linear { .. } => input_val,
                    Position::Inf { .. } => input_val,
                };

                Cmd::sub(On::Input(new_val))
            }
            Msg::InputSliderValue(input_val) => {
                let new_val = match &mut self.position {
                    Position::Linear { .. } => input_val,
                    Position::Inf { mid, step, .. } => {
                        let val = -(*mid) * (1.0 - input_val.min(1.0 - 1e-15).max(0.0)).log2();
                        let val = (*step) * (val / *step).ceil();
                        val
                    }
                };
                Cmd::sub(On::Input(new_val))
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        let (min, max, val, slider_val, step, slider_step) = match &self.position {
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
                                crate::color_system::blue(255, 5),
                                pos * 100.0,
                                pos,
                                crate::color_system::gray(255, 3),
                                pos * 100.0
                            ),
                        ),
                    Events::new()
                        .on_input(|val| {
                            val.parse()
                                .map(|val| Msg::InputSliderValue(val))
                                .unwrap_or(Msg::NoOp)
                        })
                        .on("wheel", move |e| {
                            let e = e.dyn_into::<web_sys::WheelEvent>().unwrap();
                            if e.delta_y() < 0.0 {
                                Msg::SetValue(val + step)
                            } else {
                                Msg::SetValue(val - step)
                            }
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
                match &self.position {
                    Position::Linear {min,max, ..} => self.render_range_linear(*min, *max),
                    Position::Inf {mid, ..} => self.render_range_inf(*mid)
                }
            ],
        ))
    }
}

impl Slider {
    fn render_range_linear(&self, min: f64, max: f64) -> Html {
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
                            if self.range_is_editable {
                                attrs
                            } else {
                                attrs.flag("readonly")
                            }
                        },
                        Events::new(),
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
                            if self.range_is_editable {
                                attrs
                            } else {
                                attrs.flag("readonly")
                            }
                        },
                        Events::new(),
                        vec![],
                    )],
                ),
            ],
        )
    }
    fn render_range_inf(&self, mid: f64) -> Html {
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
                        if self.range_is_editable {
                            attrs
                        } else {
                            attrs.flag("readonly")
                        }
                    },
                    Events::new(),
                    vec![],
                )],
            )],
        )
    }
}

impl Styled for Slider {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "grid-auto-rows": "max-content";
                "align-items": "center";
                "column-gap": "0.35em";
            }

            "input" {
                "width": "5em";
                "outline": "none";
            }

            "slider" {
                "width": "100%";
                "height": "1em";
                "-webkit-appearance": "none";
                "appearance": "none";
                "border-radius": "0.5em";
                "outline": "none";
                "font-size": "1.3em";
            }

            "slider::-webkit-slider-thumb" {
                "-webkit-appearance": "none";
                "appearance": "none";
                "background-color": format!("{}", crate::color_system::gray(100, 5));
                "width": "1em";
                "height": "1em";
                "border-radius": "50%";
            }

            "range" {
                "display": "flex";
                "align-items": "center";
            }

            "range--linear" {
                "justify-content": "space-between";
            }

            "range--inf" {
                "justify-content": "center";
            }

            "allow" {
                "position": "relative";
            }

            "allow:before" {
                "content":"\"\"";
                "position": "absolute";
                "border": "0.5em solid transparent";
                "border-bottom": format!("0.5em solid {}", crate::color_system::gray(100, 0));
            }

            "allow--left:before" {
                "top": "-1em";
                "left": "0";
            }

            "allow--center:before" {
                "top": "-1em";
                "left": "calc(50% - 0.5em)";
            }

            "allow--right:before" {
                "top": "-1em";
                "left": "calc(100% - 1em)";
            }
        }
    }
}
