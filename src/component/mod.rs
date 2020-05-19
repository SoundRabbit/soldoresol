mod btn;
mod room;
// mod chat;
mod checkbox;
pub mod config_loder;
mod connection;
mod contextmenu;
// mod control;
// mod dialog;
// mod form;
// mod handout;
// mod icon;
// mod measure_length;
// mod radio;

type Messenger<From: 'static, To: 'static> = Box<dyn FnOnce(From) -> To + 'static>;
type MessengerGen<From: 'static, To: 'static> = Box<dyn Fn() -> Messenger<From, To>>;
