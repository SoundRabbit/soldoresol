mod key_state;
mod mouse_state;
mod renderer;
mod table_tool;
use key_state::KeyState;
use mouse_state::MouseState;
use renderer::Renderer;
use table_tool::TableTool;

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

pub struct Room {
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
