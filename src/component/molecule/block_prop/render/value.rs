use super::super::super::atom::slider::{self, Slider};
use super::*;
use block::property::Value;

impl BlockProp {
    pub fn render_value(&self, value: &Value) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("data")),
            Events::new(),
            vec![],
        )
    }
}

//Number
impl BlockProp {
    fn render_value_number(&self, value: f64) -> Html<Self> {
        Html::input(
            Attributes::new()
                .value(value.to_string())
                .type_("number")
                .string("step", "1"),
            Events::new(),
            vec![],
        )
    }
}

//NumberMinMax
impl BlockProp {
    fn render_value_number_min_max(&self, value: f64, min: f64, max: f64) -> Html<Self> {
        Slider::empty(
            slider::Props {
                position: slider::Position::Linear {
                    min: min,
                    max: max,
                    val: value,
                    step: 1.0,
                },
                range_is_editable: true,
                theme: slider::Theme::Light,
            },
            Sub::none(),
        )
    }
}

//NumberMid
impl BlockProp {
    fn render_value_number_mid(&self, value: f64, mid: f64) -> Html<Self> {
        Slider::empty(
            slider::Props {
                position: slider::Position::Inf {
                    mid: mid,
                    val: value,
                    step: 1.0,
                },
                range_is_editable: true,
                theme: slider::Theme::Light,
            },
            Sub::none(),
        )
    }
}

//Normal
impl BlockProp {
    fn render_value_normal(&self, value: &String) -> Html<Self> {
        Html::input(Attributes::new().value(value), Events::new(), vec![])
    }
}

//Note
