mod btn;
mod checkbox;
pub mod config_loder;
mod contextmenu;
mod modal;
mod modeless;
mod modeless_modal;
mod peer_connection;
mod room;
mod room_connection;

type Messenger<From: 'static, To: 'static> = Box<dyn FnOnce(From) -> To + 'static>;
type MessengerGen<From: 'static, To: 'static> = Box<dyn Fn() -> Messenger<From, To>>;
