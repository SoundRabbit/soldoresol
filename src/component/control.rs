use kagura::prelude::*;

pub fn input<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::input(attributes.class("control-input"), events, vec![])
}

pub fn textarea<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::textarea(attributes.class("control-textarea"), events, vec![])
}
