use crate::model;
use kagura::prelude::*;

pub fn frame<Msg, Modeless>(
    modeless: &model::Modeless<Modeless>,
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(
        attributes
            .class("frame")
            .class("frame-modeless")
            .draggable(false)
            .style("left", modeless.position()[0].to_string() + "vw")
            .style("top", modeless.position()[1].to_string() + "vh")
            .style("width", modeless.size()[0].to_string() + "vw")
            .style("height", modeless.size()[1].to_string() + "vh"),
        events,
        children,
    )
}

pub fn header<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("frame-header"), events, children)
}

pub fn body<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("frame-body"), events, children)
}

pub fn footer<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("frame-footer"), events, children)
}

pub mod resizer {
    use kagura::prelude::*;

    fn base<Msg>(attributes: Attributes) -> Html<Msg> {
        Html::div(attributes, Events::new(), vec![])
    }

    pub fn top<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-t"))
    }

    pub fn left<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-l"))
    }

    pub fn bottom<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-b"))
    }

    pub fn right<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-r"))
    }

    pub fn top_left<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-tl"))
    }

    pub fn bottom_left<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-bl"))
    }

    pub fn bottom_right<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-br"))
    }

    pub fn top_right<Msg>(attributes: Attributes) -> Html<Msg> {
        base(attributes.class("frame-resizer frame-resizer-tr"))
    }
}
