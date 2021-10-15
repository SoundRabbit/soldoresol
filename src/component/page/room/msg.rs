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
