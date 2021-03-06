use super::{
    super::util::cmds::Cmds,
    super::util::State,
    children::room_modeless,
    model::controller_state::MouseState,
    model::table::TableTool,
    renderer::{CameraMatrix, ObjectId, Renderer},
};
use crate::arena::block::{self, BlockId};
use crate::arena::player;
use crate::arena::resource::{self, ResourceId};
use crate::libs::color::Pallet;
use crate::libs::modeless_list::ModelessList;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use crate::libs::skyway::{MeshRoom, Peer};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use std::rc::Rc;

mod constructor;
mod render;
mod update;

pub struct Props {
    pub peer: Rc<Peer>,
    pub peer_id: Rc<String>,
    pub room: Rc<MeshRoom>,
    pub room_id: Rc<String>,
    pub client_id: Rc<String>,
}

pub enum Msg {
    NoOp,
    SetCanvasElement {
        canvas: web_sys::HtmlCanvasElement,
    },
    ResetCanvasSize,
    RenderCanvas,
    UpdateMouseState {
        e: web_sys::MouseEvent,
    },
    UpdateMouseWheelState {
        e: web_sys::WheelEvent,
    },
    UpdateKeyState {
        e: web_sys::KeyboardEvent,
        is_key_down: bool,
    },
    SetTableToolIdx {
        idx: usize,
    },
    SetSelectingTableTool {
        tool: TableTool,
    },
    OpenNewModal {
        modal: Modal,
    },
    OpenNewModeless {
        content: room_modeless::Content,
    },
    OpenNewChatModeless,
    FocusModeless {
        modeless_id: U128Id,
    },
    CloseModeless {
        modeless_id: U128Id,
    },
    MinimizeModeless {
        modeless_id: U128Id,
    },
    RestoreModeless {
        modeless_id: U128Id,
    },
    SetModelessContainerElement {
        element: web_sys::Element,
    },
    MoveModelessTab {
        modeless_id: U128Id,
        modeless_tab_idx: Option<usize>,
        tab_modeless_id: U128Id,
        tab_idx: usize,
    },
    DropModelessTab {
        page_x: i32,
        page_y: i32,
        tab_modeless_id: U128Id,
        tab_idx: usize,
    },
    SelectModelessTab {
        modeless_id: U128Id,
        tab_idx: usize,
    },
    SetOverlay {
        overlay: Overlay,
    },
    SetContextmenu {
        contextmenu: Option<Contextmenu>,
    },
    LoadFile {
        files: Vec<web_sys::File>,
        overlay: Option<Overlay>,
    },
    LoadData {
        blocks: Vec<(BlockId, block::ArenaBlock)>,
        resources: Vec<resource::Data>,
    },
    LoadResourceData {
        data: Vec<resource::Data>,
    },
    LoadArenaBlocks {
        blocks: Vec<(BlockId, block::ArenaBlock)>,
    },
    CreateNewChannel {
        channel_name: String,
        channel_type: block::chat::channel::ChannelType,
    },
    UpdateTableProps {
        table_id: BlockId,
        size: Option<[f32; 2]>,
        grid_color: Option<Pallet>,
        background_color: Option<Pallet>,
        background_image: Option<Option<ResourceId>>,
        env_light_intensity: Option<f32>,
        terran_height: Option<f32>,
    },
    SetCharacterCommonProps {
        character_id: BlockId,
        name: Option<String>,
        display_name: Option<String>,
        description: Option<String>,
        name_color: Option<crate::libs::color::Pallet>,
    },
    SetCharacterTextureId {
        character_id: BlockId,
        tex_idx: usize,
        resource_id: Option<ResourceId>,
    },
    AddCharacterTexture {
        character_id: BlockId,
    },
    RemoveCharacterTexture {
        character_id: BlockId,
        tex_idx: usize,
    },
    SetCharacterTextureIdx {
        character_id: BlockId,
        tex_idx: usize,
    },
    SetCharacterTextureName {
        character_id: BlockId,
        tex_idx: usize,
        tex_name: String,
    },
    SetBoxblockCommonProps {
        boxblock_id: BlockId,
        name: Option<String>,
        display_name: Option<String>,
        color: Option<crate::libs::color::Pallet>,
        size: Option<[f32; 3]>,
    },
    SetPropertyName {
        property_id: BlockId,
        name: String,
    },
    AddPropertyChild {
        block_id: BlockId,
        name: String,
    },
    AddPropertyValue {
        property_id: BlockId,
    },
    SetPropertyValue {
        property_id: BlockId,
        idx: usize,
        value: block::property::Value,
    },
    RemovePropertyValue {
        property_id: BlockId,
        idx: usize,
    },
    SetPropertyValueMode {
        property_id: BlockId,
        value_mode: block::property::ValueMode,
    },
    RemoveProperty {
        property_id: BlockId,
        idx: usize,
    },
}

pub enum On {}

struct KeyState {
    space_key: bool,
    alt_key: bool,
    ctrl_key: bool,
    shift_key: bool,
}

impl KeyState {
    fn update(&mut self, e: web_sys::KeyboardEvent, is_key_down: bool) {
        let alt_key = e.alt_key();
        let ctrl_key = e.ctrl_key() || e.meta_key();
        let shift_key = e.shift_key();
        let space_key = if e.code() == "Space" {
            is_key_down
        } else {
            self.space_key
        };

        self.alt_key = alt_key;
        self.ctrl_key = ctrl_key;
        self.shift_key = shift_key;
        self.space_key = space_key;
    }
}

pub struct Implement {
    cmds: Cmds<Msg, On>,

    peer: Rc<Peer>,
    peer_id: Rc<String>,
    room: Rc<MeshRoom>,
    room_id: Rc<String>,
    client_id: Rc<String>,

    element_id: ElementId,

    table_tools: State<SelectList<TableTool>>,
    modeless_list: ModelessList<ModelessContent>,
    modeless_container_element: Option<State<web_sys::Element>>,

    block_arena: block::Arena,
    local_block_arena: block::Arena,
    player_arena: player::Arena,
    resource_arena: resource::Arena,

    renderer: Option<Renderer>,
    camera_matrix: CameraMatrix,

    chat_id: BlockId,
    world_id: BlockId,

    modal: Modal,
    overlay: Overlay,
    contextmenu: Option<Contextmenu>,

    mouse_state: MouseState,
    key_state: KeyState,
    canvas: Option<Rc<web_sys::HtmlCanvasElement>>,
    canvas_pos: [f32; 2],
    canvas_size: [f32; 2],
    drawing_line: Vec<[f64; 2]>,
    grabbed_object_id: ObjectId,
}

struct ModelessContent {
    content: State<SelectList<room_modeless::Content>>,
    page_x: i32,
    page_y: i32,
    minimized: bool,
}

struct ElementId {
    header_room_id: String,
}

enum Modal {
    None,
    NewChannel,
    ImportedFiles,
}

enum Overlay {
    None,
    DragFile,
}

struct Contextmenu {
    page_x: i32,
    page_y: i32,
    kind: ContextmenuKind,
}

enum ContextmenuKind {
    Character(BlockId),
    Boxblock(BlockId),
}

impl Constructor for Implement {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        Self::constructor(props, builder)
    }
}

impl Component for Implement {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Msg) -> Cmd<Msg, On> {
        self.update(msg)
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(self.render(children))
    }
}

impl Styled for Implement {
    fn style() -> Style {
        style! {
            ".overlay" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "height": "100vh";
                "width": "100vw";
                "z-index": format!("{}", super::super::constant::z_index::OVERLAY);
            }

            ".overlay-file-import" {
                "background-color": format!("{}", crate::libs::color::color_system::gray(25, 9));
            }

            ".overlay-file-import-text" {
                "position": "absolute";
                "color": format!("{}", crate::libs::color::color_system::gray(100, 0));
                "font-size": "4rem";
                "bottom": "1em";
                "right": "1em";
            }

            ".contextmenu" {
                "position": "absolute";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "rows";
                "row-gap": "0.05rem";
                "justify-items": "stretch";
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "border-radius": "2px";
                "display": "grid";
            }

            ".header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            ".header-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "column-gap": "0.65em";
            }

            ".header-controller-menu" {
                "display": "grid";
                "grid-auto-columns": "max-content";
                "grid-auto-flow": "column";
                "column-gap": "0.65em";
            }

            ".body" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
            }

            ".side-menu" {
                "z-index": "1";
                "min-height": "max-content";
                "min-width": "max-content";
            }

            ".main" {
                "position": "relative";
            }

            ".canvas" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
            }

            ".modeless-container" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": "0";
                "overflow": "hidden";
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "max-content";
                "justify-content": "start";
                "align-content": "end";
            }
        }
    }
}
