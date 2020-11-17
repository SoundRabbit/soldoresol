use super::util::styled::{Style, Styled};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub position: Position,
}

pub enum Msg {
    NoOp,
    InputSliderValue(f64),
}

pub enum On {
    Input(f64),
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

impl Constructor for Slider {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            position: props.position,
        }
    }
}

impl Component for Slider {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        // self.position = props.position;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::InputSliderValue(input_val) => {
                let new_val = match &mut self.position {
                    Position::Linear { val, .. } => {
                        *val = input_val;
                        *val
                    }
                    Position::Inf { val, mid, .. } => {
                        *val = -(*mid) * (1.0 - input_val.min(1.0).max(0.0)).log2();
                        *val
                    }
                };
                Cmd::Sub(On::Input(new_val))
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        let (min, max, val, step) = match &self.position {
            Position::Linear {
                min,
                max,
                val,
                step,
            } => (*min, *max, *val, *step),
            Position::Inf { val, mid, step } => {
                let val = val.max(0.0);
                let prev = (val - step).max(0.0);
                let next = (val + step).max(0.0);
                (
                    0.0_f64,
                    1.0_f64,
                    1.0_f64 - (0.5_f64).powf(val / mid),
                    ((0.5_f64).powf(prev / mid) - (0.5_f64).powf(next / mid)) / 2.0,
                )
            }
        };

        let pos = ((val - min) / (max - min)).min(1.0).max(0.0);

        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new()
                        .type_("number")
                        .class(Self::class("value")).style("left", format!("calc({}% - {}em + 0.5em)", pos * 100.0, pos * 5.0)),
                    Events::new(),
                    vec![],
                ),
                Html::input(
                    Attributes::new()
                        .type_("range")
                        .string("min", min.to_string())
                        .string("max", max.to_string())
                        .string("step", step.to_string())
                        .value(val.to_string())
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
                                Msg::InputSliderValue(val + step)
                            } else {
                                Msg::InputSliderValue(val - step)
                            }
                        }),
                    vec![],
                ),
            ],
        ))
    }
}

impl Styled for Slider {
    fn style() -> Style {
        style! {
            "base" {
                "position": "relative";
                "padding": "2em 2em";
            }

            "slider" {
                "width": "100%";
                "height": "1em";
                "-webkit-appearance": "none";
                "appearance": "none";
                "border-radius": "0.5em";
                "outline": "none";
            }

            "slider::-webkit-slider-thumb" {
                "-webkit-appearance": "none";
                "appearance": "none";
                "background-color": format!("{}", crate::color_system::gray(255, 5));
                "width": "1em";
                "height": "1em";
                "border-radius": "50%";
            }

            "value" {
                "position": "absolute";
                "width": "4em";
                "top": "0";
                "outline": "none";
                "border": "none";
            }
        }
    }
}
