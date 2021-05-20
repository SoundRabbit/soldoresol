use super::atom::tag::{self, Tag};
use crate::arena::block::{self, BlockId};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub world_id: BlockId,
    pub removable: bool,
}

pub enum Msg {
    Sub(On),
}

pub enum On {
    Click(BlockId),
}

pub struct TagList {
    block_arena: block::ArenaRef,
    world_id: BlockId,
    removable: bool,
}

impl Constructor for TagList {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            block_arena: props.block_arena,
            world_id: props.world_id,
            removable: props.removable,
        }
    }
}

impl Component for TagList {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.world_id = props.world_id;
        self.removable = props.removable;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new(),
            Events::new(),
            self.block_arena
                .map(&self.world_id, |world: &block::world::World| {
                    world
                        .tags()
                        .map(|tag_id| {
                            Tag::empty(
                                tag::Props {
                                    block_arena: block::ArenaRef::clone(&self.block_arena),
                                    block_id: BlockId::clone(tag_id),
                                    removable: self.removable,
                                },
                                Subscription::none(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or(vec![]),
        ))
    }
}

impl Styled for TagList {
    fn style() -> Style {
        style! {}
    }
}
