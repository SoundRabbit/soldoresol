use super::page::{initializer, Initializer};
use crate::Config;
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {
    SetConfig(Config),
}

pub enum Sub {}

pub struct App {
    config: Option<Config>,
}

impl Constructor for App {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self { config: None }
    }
}

impl Component for App {
    type Props = Props;
    type Msg = Msg;
    type Sub = Sub;

    fn init(&mut self, _: Props, _: &mut ComponentBuilder<Msg, Sub>) {}

    fn update(&mut self, msg: Msg) -> Cmd<Msg, Sub> {
        match msg {
            Msg::SetConfig(config) => {
                self.config = Some(config);
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Initializer::view(initializer::Props {}, SubMap::empty(), vec![])
    }
}

impl App {}
