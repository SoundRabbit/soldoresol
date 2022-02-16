use crate::arena::{block, ArenaMut, BlockMut};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

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
    type Sub = On;
}

impl Constructor for BlockProp {
    fn constructor(props: &Props) -> Self {
        Self {
            arena: ArenaMut::clone(&props.arena),
            data: BlockMut::clone(&props.data),
        }
    }
}

impl Update for BlockProp {}
