pub mod app;
mod btn;
mod chat;
mod checkbox;
mod context_menu;
mod form;
mod handout;
mod icon;
mod measure_length;
mod measure_tool;
mod radio;

type Messenger<From: 'static, To: 'static> = Box<dyn FnOnce(From) -> To + 'static>;
type MessengerGen<From: 'static, To: 'static> = Box<dyn Fn() -> Messenger<From, To>>;
