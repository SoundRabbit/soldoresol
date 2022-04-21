use crate::arena::{block, ArenaMut, BlockMut};
use kagura::prelude::*;
use nusa::prelude::*;

mod render;

pub struct Props {
    arena: ArenaMut,
    data: BlockMut<block::Property>,
}

pub enum Msg {}

pub enum On {}

pub struct BlockProp {
    arena: ArenaMut,
    data: BlockMut<block::Property>,
}

impl Component for BlockProp {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for BlockProp {}

impl Constructor for BlockProp {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            data: props.data,
        }
    }
}

impl Update for BlockProp {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.data = props.data;
        Cmd::none()
    }
}
