use kagura::prelude::*;

pub fn container<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("grid grid-table"), events, children)
}

pub fn frame<Msg>(
    loc: &[u32; 2],
    size: &[u32; 2],
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    let clm = if loc[0] == 0 {
        format!("grid-cc-2x{}", size[0] / 2)
    } else {
        format!("grid-cs-{} grid-w-{}", loc[0], size[0])
    };
    let rw = if loc[1] == 0 {
        format!("grid-rc-2x{}", size[1] / 2)
    } else {
        format!("grid-rs-{} grid-h-{}", loc[1], size[1])
    };

    Html::div(
        attributes.class(format!("frame {} {}", clm, rw)),
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
