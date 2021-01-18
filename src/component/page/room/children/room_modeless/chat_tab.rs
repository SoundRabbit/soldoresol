use super::super::util::styled::{Style, Styled};
use crate::arena::block;
use async_std::sync::{Arc, Mutex};
use kagura::prelude::*;

pub struct Props {
    block_arena: block::Arena,
    tab_id: block::BlockId,
}

pub enum Msg {}

pub enum On {}

pub struct ChatTab {
    block_arena: block::Arena,
    tab_id: block::BlockId,
}

impl Constructor for ChatTab {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        Self {
            block_arena: props.block_arena,
            tab_id: props.tab_id,
        }
    }
}

impl Component for ChatTab {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.tab_id = props.tab_id;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(&self.tab_id, |tab: &block::chat::tab::Tab| {
                    Html::div(Attributes::new(), Events::new(), vec![])
                })
                .unwrap_or(Html::none()),
        )
    }
}

impl Styled for ChatTab {
    fn style() -> Style {
        style! {}
    }
}
