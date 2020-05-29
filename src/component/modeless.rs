use kagura::prelude::*;

pub fn container<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("grid grid-table"), events, children)
}

pub fn frame<Msg>(
    loc_a: &[i32; 2],
    loc_b: &[i32; 2],
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    let clm = format!(
        "grid-cs-{} grid-ce-{}",
        loc_a[0].min(loc_b[0]).max(1),
        loc_a[0].max(loc_b[0]).min(25)
    );
    let row = format!(
        "grid-rs-{} grid-re-{}",
        loc_a[1].min(loc_b[1]).max(1),
        loc_a[1].max(loc_b[1]).min(15)
    );

    Html::div(
        attributes
            .class(format!("frame {} {}", clm, row))
            .draggable(false),
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

    fn base<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
        Html::div(attributes, events, vec![])
    }

    pub fn top<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-t"),
            events,
        )
    }

    pub fn left<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-l"),
            events,
        )
    }

    pub fn bottom<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-b"),
            events,
        )
    }

    pub fn right<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-r"),
            events,
        )
    }

    pub fn top_left<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-tl"),
            events,
        )
    }

    pub fn bottom_left<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-bl"),
            events,
        )
    }

    pub fn bottom_right<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-br"),
            events,
        )
    }

    pub fn top_right<Msg>(events: Events<Msg>) -> Html<Msg> {
        base(
            Attributes::new().class("frame-resizer frame-resizer-tr"),
            events,
        )
    }
}
