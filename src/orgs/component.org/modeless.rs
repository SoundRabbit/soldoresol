use crate::model;
use kagura::prelude::*;

pub fn frame<Modeless>(
    modeless: &model::Modeless<Modeless>,
    attributes: Attributes,
    events: Events,
    children: Vec<Html>,
) -> Html {
    Html::div(
        attributes
            .class("frame")
            .class("frame-modeless")
            .draggable(false)
            .style("left", modeless.position()[0].to_string() + "%")
            .style("top", modeless.position()[1].to_string() + "%")
            .style("width", modeless.size()[0].to_string() + "%")
            .style("height", modeless.size()[1].to_string() + "%"),
        events,
        children,
    )
}

pub fn header(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(attributes.class("frame-header"), events, children)
}

pub fn body(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(attributes.class("frame-body"), events, children)
}

pub fn footer(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(attributes.class("frame-footer"), events, children)
}

pub mod resizer {
    use kagura::prelude::*;

    fn base(attributes: Attributes) -> Html {
        Html::div(attributes, Events::new(), vec![])
    }

    pub fn top(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-t"))
    }

    pub fn left(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-l"))
    }

    pub fn bottom(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-b"))
    }

    pub fn right(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-r"))
    }

    pub fn top_left(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-tl"))
    }

    pub fn bottom_left(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-bl"))
    }

    pub fn bottom_right(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-br"))
    }

    pub fn top_right(attributes: Attributes) -> Html {
        base(attributes.class("frame-resizer frame-resizer-tr"))
    }
}
