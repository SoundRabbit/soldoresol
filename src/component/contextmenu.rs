use kagura::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub fn div<Msg: 'static>(
    z_index: u64,
    on_close: impl FnMut() -> Msg + 'static,
    position: &[f64; 2],
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    let on_close = Rc::new(RefCell::new(Box::new(on_close)));
    Html::div(
        Attributes::new()
            .class("fullscreen")
            .style("z-index", z_index.to_string()),
        Events::new()
            .on_click({
                let on_close = Rc::clone(&on_close);
                move |e| {
                    e.stop_propagation();
                    (&mut *on_close.borrow_mut())()
                }
            })
            .on_contextmenu({
                let on_close = Rc::clone(&on_close);
                move |e| {
                    e.prevent_default();
                    (&mut *on_close.borrow_mut())()
                }
            }),
        vec![Html::div(
            attributes
                .class("pure-menu")
                .style("position", "absolute")
                .style("left", (position[0] - 5.0).to_string() + "px")
                .style("top", (position[1] - 2.5).to_string() + "px"),
            events,
            children,
        )],
    )
}
