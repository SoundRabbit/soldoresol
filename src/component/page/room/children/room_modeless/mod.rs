use super::atom::btn::{self, Btn};
use super::atom::fa;
use super::molecule::modeless::{self, Modeless};
use super::util::styled::{Style, Styled};
use super::util::Prop;
use crate::libs::random_id::U128Id;
use kagura::prelude::*;

pub enum Content {
    ChatPanel,
}

pub struct Props {
    pub content: Prop<Content>,
    pub z_index: usize,
    pub modeless_id: U128Id,
}

pub enum Msg {}

pub enum On {}

pub struct RoomModeless {
    content: Prop<Content>,
    pub z_index: usize,
    pub modeless_id: U128Id,
}

impl Constructor for RoomModeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            content: props.content,
            z_index: props.z_index,
            modeless_id: props.modeless_id,
        }
    }
}

impl Component for RoomModeless {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.content = props.content
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Modeless::with_children(
            modeless::Props {
                z_index: self.z_index,
                ..Default::default()
            },
            Subscription::none(),
            vec![],
        ))
    }
}

impl RoomModeless {
    fn render_header(&self) -> Html {
        Html::none()
    }

    fn render_content(&self) -> Html {
        Html::none()
    }
}

impl Styled for RoomModeless {
    fn style() -> Style {
        style! {}
    }
}
