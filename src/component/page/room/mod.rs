use super::organism::room_modeless;
use super::organism::room_modeless_chat::ChatUser;
use super::organism::tab_modeless_container::TabModelessList;
use crate::arena::{block, resource, user, Arena, ArenaMut, BlockMut, Untyped};
use crate::libs::bcdice::js::{DynamicLoader, GameSystemClass};
use crate::libs::random_id::U128Id;
use crate::table::{table_tool::TableTool, Table};
use kagura::prelude::*;
use nusa::prelude::*;
use nusa::v_node::v_element::VEvent;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

mod constructor;
mod render;
mod update;

pub struct Props {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
    pub bcdice_loader: Rc<DynamicLoader>,
}

pub enum Msg {
    NoOp,
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
    OpenChatModeless(ChatUser),
    OpenBoxblockModeless(U128Id),
    OpenCharacterModeless(U128Id),
    OpenCraftboardModeless(U128Id),
    OpenTextboardModeless(U128Id),
    SetOkToCatchFile(bool),
    SetSelectedTableTool(TableTool),
    SetShowingContextmenu(Option<ShowingContextmenu>),
    SetShowingModal(ShowingModal),
    CloseModalChatUser(Vec<BlockMut<block::Character>>),
    OnTableWheel(VEvent<web_sys::WheelEvent>),
    OnTableClick(VEvent<web_sys::MouseEvent>),
    OnTableMousedown(VEvent<web_sys::MouseEvent>),
    OnTableMouseup(VEvent<web_sys::MouseEvent>),
    OnTableMousemove(VEvent<web_sys::MouseEvent>),
    OnTableContextmenu(VEvent<web_sys::MouseEvent>),
    AddResourceImageData(resource::ImageData),
    SetIs2dMode(bool, bool),
    SetBlockIsFixedPosition(BlockMut<Untyped>, bool),
    SetBlockIsBindToGrid(BlockMut<Untyped>, bool),
    SetGameSystemClass(GameSystemClass),
    RemoveCharacter(U128Id),
    RemoveBoxblock(U128Id),
    RemoveCraftboard(U128Id),
    RemoveTextboard(U128Id),
    CreateComponent(BlockMut<Untyped>),
}

pub enum On {}

pub struct Room {
    arena: ArenaMut,
    local_arena: Arena,
    client_id: Rc<String>,

    bcdice_loader: Rc<DynamicLoader>,
    game_system_class: Rc<RefCell<Option<GameSystemClass>>>,

    chat: BlockMut<block::Chat>,
    world: BlockMut<block::World>,
    me: BlockMut<user::Player>,

    table: Rc<RefCell<Table>>,
    modeless_container:
        Rc<RefCell<TabModelessList<room_modeless::RoomModeless, room_modeless::TabName>>>,

    table_tool: TableTool,
    ok_to_catch_file: bool,
    is_2d_mode: bool,
    is_debug_mode: bool,
    chat_users: Vec<ChatUser>,

    showing_contextmenu: Option<ShowingContextmenu>,
    showing_modal: ShowingModal,
}

pub struct ShowingContextmenu {
    page_x: f64,
    page_y: f64,
    data: ShowingContextmenuData,
}

enum ShowingContextmenuData {
    Boxblock(BlockMut<block::Boxblock>),
    Character(BlockMut<block::Character>),
    Craftboard(BlockMut<block::Craftboard>),
    Textboard(BlockMut<block::Textboard>),
}

pub enum ShowingModal {
    None,
    ChatUser,
    Dicebot,
    Resource,
}

impl Component for Room {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Room {}

fn open_modeless(
    client_id: &Rc<String>,
    arena: &ArenaMut,
    world: &BlockMut<block::World>,
    modeless_container: &Rc<
        RefCell<TabModelessList<room_modeless::RoomModeless, room_modeless::TabName>>,
    >,
    content: room_modeless::ContentData,
) {
    modeless_container
        .borrow_mut()
        .open_modeless(vec![room_modeless::Content {
            arena: ArenaMut::clone(arena),
            world: BlockMut::clone(world),
            client_id: Rc::clone(&client_id),
            data: content,
        }]);
}
