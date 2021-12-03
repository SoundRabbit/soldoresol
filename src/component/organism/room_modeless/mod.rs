use super::organism::room_modeless_boxblock::{self, RoomModelessBoxblock};
use super::organism::room_modeless_chat::{self, RoomModelessChat};
use crate::arena::{block, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
pub struct Content {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub client_id: Rc<String>,
    pub data: ContentData,
}

#[derive(Clone)]
pub enum ContentData {
    ChatChannel(BlockMut<block::ChatChannel>),
    Boxblock(BlockMut<block::Boxblock>),
}

pub enum Msg {
    Sub(On),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModeless {}

ElementId! {
    input_channel_name,
    input_boxblock_name
}

impl Component for RoomModeless {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModeless {
    fn constructor(_: &Content) -> Self {
        Self {}
    }
}

impl Update for RoomModeless {
    fn update(&mut self, _content: &Content, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }
}

impl Render for RoomModeless {
    fn render(&self, content: &Content, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::fragment(vec![match &content.data {
            ContentData::ChatChannel(chat_channel) => RoomModelessChat::empty(
                room_modeless_chat::Props {
                    arena: ArenaMut::clone(&content.arena),
                    data: BlockMut::clone(&chat_channel),
                    client_id: Rc::clone(&content.client_id),
                },
                Sub::map(|sub| match sub {
                    room_modeless_chat::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
            ContentData::Boxblock(boxblock) => RoomModelessBoxblock::empty(
                room_modeless_boxblock::Props {
                    arena: ArenaMut::clone(&content.arena),
                    world: BlockMut::clone(&content.world),
                    data: BlockMut::clone(&boxblock),
                },
                Sub::map(|sub| match sub {
                    room_modeless_boxblock::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
        }]))
    }
}

impl Styled for RoomModeless {
    fn style() -> Style {
        style! {
            ".banner" {
                "grid-column": "1 / -1";
            }

            ".common-label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            ".common-base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "row-gap": ".65rem";
                "padding-top": ".65rem";
                "padding-bottom": ".65rem";
                "height": "100%";
            }

            ".common-header" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-auto-rows": "max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
            }
        }
    }
}

pub struct TabName {}

impl Component for TabName {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for TabName {
    fn constructor(_: &Content) -> Self {
        Self {}
    }
}

impl Update for TabName {}

impl Render for TabName {
    fn render(&self, content: &Content, _children: Vec<Html<Self>>) -> Html<Self> {
        use super::atom::fa;
        match &content.data {
            ContentData::ChatChannel(chat_channel) => chat_channel
                .map(|cc| Html::text(String::from("#") + cc.name()))
                .unwrap_or(Html::none()),
            ContentData::Boxblock(boxblock) => boxblock
                .map(|bb| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![fa::i("fa-cube"), Html::text(" "), Html::text(bb.name())],
                    )
                })
                .unwrap_or(Html::none()),
        }
    }
}
