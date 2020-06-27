use kagura::prelude::*;

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
