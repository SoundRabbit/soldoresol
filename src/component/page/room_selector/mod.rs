use super::template::{editor, Editor};
use super::template::{loader, Loader};
use super::util::Prop;
use kagura::prelude::*;

mod task;

pub struct Props {
    pub common_database: Prop<web_sys::IdbDatabase>,
}

pub enum Msg {
    SetRooms(Vec<(String, js_sys::Date)>),
}

pub enum On {}

pub struct RoomSelector {
    rooms: Option<Vec<(String, js_sys::Date)>>,
}

impl Constructor for RoomSelector {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        builder.set_cmd(Cmd::task({
            let common_database = props.common_database.clone();
            move |resolve| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(rooms) = task::get_room_index(&common_database).await {
                        crate::debug::log_1("success to load index of rooms");
                        resolve(Msg::SetRooms(rooms));
                    } else {
                        crate::debug::log_1("faild to load index of rooms");
                    }
                });
            }
        }));

        Self { rooms: None }
    }
}

impl Component for RoomSelector {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, _: Vec<Html>) -> Html {
        match &self.rooms {
            None => Loader::empty(loader::Props {}, Subscription::none()),
            Some(rooms) => Loader::empty(loader::Props {}, Subscription::none()),
        }
    }
}
