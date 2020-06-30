mod awesome;
mod btn;
mod color_picker;
pub mod config_loder;
mod contextmenu;
mod icon;
mod modal;
mod modeless;
mod peer_connection;
mod room;
mod room_connection;

type Messenger<From, To> = Box<dyn FnOnce(From) -> To + 'static>;
type MessengerGen<From, To> = Box<dyn Fn() -> Messenger<From, To>>;
