use crate::arena::{block, ArenaMut, BlockMut};
use kagura::prelude::*;
use nusa::prelude::*;

mod render;

pub enum Msg {}

pub enum On {}

pub struct BlockProp {}

impl Component for BlockProp {
    type Props = ();
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for BlockProp {}

impl Constructor for BlockProp {
    fn constructor(props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for BlockProp {}
