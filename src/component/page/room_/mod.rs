use isaribi::styled::Styled;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;

mod msg;
mod on;
mod props;
mod state;
pub use msg::Msg;
pub use on::On;
pub use props::Props;
pub use state::Room;

mod style;
mod update;

impl Constructor for Room {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        Self::constructor(props, builder)
    }
}

impl Component for Room {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Msg) -> Cmd<Msg, On> {
        Self::update(self, msg)
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(self.render(children))
    }
}
