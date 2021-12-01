use super::atom::table::{self, table_tool::TableTool, Table};
use super::organism::room_modeless::{self, RoomModeless};
use super::organism::tab_modeless_container::{self, TabModelessContainer};
use crate::arena::{block, resource, Arena, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use crate::libs::skyway::{MeshRoom, Peer};
use kagura::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;

mod constructor;
mod render;
mod update;

pub struct Props {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
}

pub enum Msg {
    NoOp,
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
    OpenChatModeless(Option<U128Id>),
    OpenBoxblockModeless(U128Id),
    SetOkToCatchFile(bool),
    SetSelectedTableTool(TableTool),
    SetShowingContextmenu(Option<ShowingContextmenu>),
    OnTableClicked(web_sys::MouseEvent),
    OnTableContextmenu(web_sys::MouseEvent),
    AddResourceImageData(resource::ImageData),
    SetIs2dMode(bool),
}

pub enum On {}

pub struct Room {
    arena: ArenaMut,
    local_arena: Arena,

    chat: BlockMut<block::Chat>,
    world: BlockMut<block::World>,

    table: PrepackedComponent<Table>,
    modeless_container:
        PrepackedComponent<TabModelessContainer<RoomModeless, room_modeless::TabName>>,

    table_tool: TableTool,
    ok_to_catch_file: bool,
    is_2d_mode: bool,

    showing_contextmenu: Option<ShowingContextmenu>,
}

pub struct ShowingContextmenu {
    page_x: f64,
    page_y: f64,
    data: ShowingContextmenuData,
}

enum ShowingContextmenuData {
    Boxblock(BlockMut<block::Boxblock>),
}

impl Component for Room {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}
