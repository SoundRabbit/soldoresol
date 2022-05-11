use super::atom::fa;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {
    NoOp,
}

pub enum On {}

pub struct Sortable<Id: 'static> {
    __phantom_id: std::marker::PhantomData<Id>,
}

impl<Id: 'static> Component for Sortable<Id> {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl<Id: 'static> HtmlComponent for Sortable<Id> {}

impl<Id: 'static> Constructor for Sortable<Id> {
    fn constructor(props: Self::Props) -> Self {
        Self {
            __phantom_id: std::marker::PhantomData,
        }
    }
}

impl<Id: 'static> Update for Sortable<Id> {}

impl<Id: 'static> Render<Html> for Sortable<Id> {
    type Children = (Attributes, Events, Vec<(Id, Attributes, Events, Vec<Html>)>);
    fn render(&self, (attrs, events, children): Self::Children) -> Html {
        Self::styled(Html::div(
            attrs.class(Self::class("base")),
            events,
            children
                .into_iter()
                .map(|child| self.render_child(child))
                .collect(),
        ))
    }
}

impl<Id: 'static> Sortable<Id> {
    fn render_child(
        &self,
        (id, attrs, events, children): (Id, Attributes, Events, Vec<Html>),
    ) -> Html {
        Html::div(
            attrs.class(Self::class("item")).draggable("true"),
            events.on_dragstart(self, |e| {
                e.stop_propagation();
                Msg::NoOp
            }),
            children,
        )
    }
}

impl<Id: 'static> Styled for Sortable<Id> {
    fn style() -> Style {
        style! {
            ".item" {
                "display": "flex";
            }

            ".dragger" {
                "width": "max-content";
            }

            ".child" {
                "flex-grow": "1";
            }
        }
    }
}
