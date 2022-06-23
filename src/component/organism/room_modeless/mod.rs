use super::organism::room_modeless_boxblock::{self, RoomModelessBoxblock};
use super::organism::room_modeless_character::{self, RoomModelessCharacter};
use super::organism::room_modeless_chat::{self, RoomModelessChat};
use super::organism::room_modeless_craftboard::{self, RoomModelessCraftboard};
use super::organism::room_modeless_textboard::{self, RoomModelessTextboard};
use crate::arena::{block, ArenaMut, BlockMut};
use crate::libs::bcdice::js::GameSystemClass;
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use room_modeless_chat::ChatUser;
use std::cell::RefCell;
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
    Chat {
        user: ChatUser,
        data: BlockMut<block::Chat>,
        game_system_class: Rc<RefCell<Option<GameSystemClass>>>,
    },
    Boxblock(BlockMut<block::Boxblock>),
    Character(BlockMut<block::Character>),
    Craftboard(BlockMut<block::Craftboard>),
    Textboard(BlockMut<block::Textboard>),
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

pub struct RoomModeless {
    content: Content,
}

ElementId! {
    input_channel_name,
    input_boxblock_name
}

impl Component for RoomModeless {
    type Props = Content;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomModeless {}

impl Constructor for RoomModeless {
    fn constructor(content: Content) -> Self {
        Self { content }
    }
}

impl Update for RoomModeless {
    fn on_load(mut self: Pin<&mut Self>, content: Content) -> Cmd<Self> {
        self.content = content;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
        }
    }
}

impl Render<Html> for RoomModeless {
    type Children = ();
    fn render(&self, _: ()) -> Html {
        Self::styled(Html::fragment(vec![match &self.content.data {
            ContentData::Chat {
                user,
                data,
                game_system_class,
            } => RoomModelessChat::empty(
                self,
                None,
                room_modeless_chat::Props {
                    arena: ArenaMut::clone(&self.content.arena),
                    data: BlockMut::clone(&data),
                    user: ChatUser::clone(&user),
                    client_id: Rc::clone(&self.content.client_id),
                    game_system_class: Rc::clone(&game_system_class),
                },
                Sub::map(|sub| match sub {
                    room_modeless_chat::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
            ContentData::Boxblock(boxblock) => RoomModelessBoxblock::empty(
                self,
                None,
                room_modeless_boxblock::Props {
                    arena: ArenaMut::clone(&self.content.arena),
                    world: BlockMut::clone(&self.content.world),
                    data: BlockMut::clone(&boxblock),
                },
                Sub::map(|sub| match sub {
                    room_modeless_boxblock::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
            ContentData::Character(character) => RoomModelessCharacter::empty(
                self,
                None,
                room_modeless_character::Props {
                    arena: ArenaMut::clone(&self.content.arena),
                    world: BlockMut::clone(&self.content.world),
                    data: BlockMut::clone(&character),
                },
                Sub::map(|sub| match sub {
                    room_modeless_character::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
            ContentData::Craftboard(craftboard) => RoomModelessCraftboard::empty(
                self,
                None,
                room_modeless_craftboard::Props {
                    arena: ArenaMut::clone(&self.content.arena),
                    world: BlockMut::clone(&self.content.world),
                    data: BlockMut::clone(&craftboard),
                },
                Sub::map(|sub| match sub {
                    room_modeless_craftboard::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
            ContentData::Textboard(textboard) => RoomModelessTextboard::empty(
                self,
                None,
                room_modeless_textboard::Props {
                    arena: ArenaMut::clone(&self.content.arena),
                    world: BlockMut::clone(&self.content.world),
                    data: BlockMut::clone(&textboard),
                },
                Sub::map(|sub| match sub {
                    room_modeless_textboard::On::UpdateBlocks { insert, update } => {
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
                "overflow": "hidden";
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

pub struct TabName {
    content: Content,
}

impl Component for TabName {
    type Props = Content;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for TabName {}

impl Constructor for TabName {
    fn constructor(props: Self::Props) -> Self {
        Self { content: props }
    }
}

impl Update for TabName {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.content = props;
        Cmd::none()
    }
}

impl Render<Html> for TabName {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        use super::atom::fa;
        match &self.content.data {
            ContentData::Chat { user, .. } => match user {
                ChatUser::Player(player) => player.map(|player| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            fa::fas_i("fa-comment"),
                            Html::text(" "),
                            Html::text(player.name()),
                        ],
                    )
                }),
                ChatUser::Character(character) => character.map(|character| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            fa::fas_i("fa-comment"),
                            Html::text(" "),
                            Html::text(character.name()),
                        ],
                    )
                }),
            }
            .unwrap_or(Html::none()),
            ContentData::Boxblock(boxblock) => boxblock
                .map(|bb| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![fa::fas_i("fa-cube"), Html::text(" "), Html::text(bb.name())],
                    )
                })
                .unwrap_or(Html::none()),
            ContentData::Character(character) => character
                .map(|c| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![fa::fas_i("fa-user"), Html::text(" "), Html::text(c.name())],
                    )
                })
                .unwrap_or(Html::none()),
            ContentData::Craftboard(craftboard) => craftboard
                .map(|cb| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            fa::fas_i("fa-border-all"),
                            Html::text(" "),
                            Html::text(cb.name()),
                        ],
                    )
                })
                .unwrap_or(Html::none()),
            ContentData::Textboard(textboard) => textboard
                .map(|tb| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            fa::fas_i("fa-file-lines"),
                            Html::text(" "),
                            Html::text(tb.title()),
                        ],
                    )
                })
                .unwrap_or(Html::none()),
        }
    }
}
