use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

type EventListeners = Vec<Box<dyn FnOnce(Rc<web_sys::Event>)>>;
type EventListenersTable = RefCell<HashMap<String, (EventListeners, EventListeners)>>;
thread_local!(static EVENT_LISTENERS: EventListenersTable = RefCell::new(HashMap::new()));

pub fn add_event_listener(
    event_type: impl Into<String>,
    use_capture: bool,
    listener: impl FnOnce(Rc<web_sys::Event>) + 'static,
) {
    let event_type = event_type.into();
    EVENT_LISTENERS.with(|event_listners| {
        if !event_listners.borrow().contains_key(&event_type) {
            let a = Closure::wrap(Box::new({
                let event_type = event_type.clone();
                move |e| {
                    let e = Rc::new(e);
                    EVENT_LISTENERS
                        .with(|event_listners| {
                            if let Some(listeners) =
                                event_listners.borrow_mut().get_mut(&event_type)
                            {
                                if use_capture {
                                    listeners.1.drain(..).collect()
                                } else {
                                    listeners.0.drain(..).collect()
                                }
                            } else {
                                vec![]
                            }
                        })
                        .into_iter()
                        .fold((), |_, listener| listener(Rc::clone(&e)));
                }
            }) as Box<dyn FnMut(web_sys::Event)>);
            let _ = web_sys::window()
                .unwrap()
                .add_event_listener_with_callback_and_bool(
                    &event_type,
                    a.as_ref().unchecked_ref(),
                    use_capture,
                );
            a.forget();

            event_listners
                .borrow_mut()
                .insert(event_type.clone(), (vec![], vec![]));
        }

        if let Some(listeners) = event_listners.borrow_mut().get_mut(&event_type) {
            if use_capture {
                listeners.1.push(Box::new(listener));
            } else {
                listeners.0.push(Box::new(listener));
            }
        }
    });
}
