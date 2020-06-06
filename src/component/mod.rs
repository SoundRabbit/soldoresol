mod awesome;
mod btn;
pub mod config_loder;
mod contextmenu;
mod modal;
mod modeless;
mod modeless_modal;
mod peer_connection;
mod room;
mod room_connection;

type Messenger<From, To> = Box<dyn FnOnce(From) -> To + 'static>;
type MessengerGen<From, To> = Box<dyn Fn() -> Messenger<From, To>>;
