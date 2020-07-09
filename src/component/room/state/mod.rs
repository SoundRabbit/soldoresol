pub mod chat;
pub mod contextmenu;
pub mod dicebot;
pub mod headermenu;
pub mod speech_bubble;
pub mod table;

pub use contextmenu::Contextmenu;

use crate::{
    block::{self, BlockId},
    model,
    model::modeless::{self, ModelessId},
    renderer::{Camera, Renderer},
    skyway::{Peer, Room},
    Color, Resource,
};
use kagura::prelude::*;
use std::{collections::HashSet, rc::Rc};

pub enum Modeless {
    Object {
        tabs: Vec<BlockId>,
        focused: usize,
        outlined: Option<Color>,
    },
    Chat,
}

pub enum Modal {
    Resource,
    SelectTableImage(BlockId),
    SelectCharacterImage(BlockId),
    SelectPlayerImage,
    PersonalSetting,
    TablemaskColorPicker(BlockId),
    SenderCharacterSelecter,
    TableSetting,
    ChatLog(BlockId),
    ChatTabEditor,
    DicebotSelecter,
}

pub struct State<M, S> {
    peer: Rc<Peer>,
    room: Rc<Room>,
    peers: HashSet<String>,
    personal_data: model::PersonalData,
    resource: Resource,
    block_field: block::Field,
    chat: chat::State,
    table: table::State,
    world: BlockId,
    camera: Camera,
    renderer: Option<Renderer>,
    pixel_ratio: f32,
    canvas_size: [f32; 2],
    speech_bubble: speech_bubble::State,
    contextmenu: Option<contextmenu::State>,
    headermenu: Option<headermenu::State>,
    modeless: modeless::Collection<Modeless>,
    modal: Vec<Modal>,
    dicebot: dicebot::State,
    common_database: Rc<web_sys::IdbDatabase>,
    room_database: Rc<web_sys::IdbDatabase>,
    cmd_queue: model::CmdQueue<M, S>,
}

impl Modeless {
    fn is_chat(&self) -> bool {
        match self {
            Self::Chat => true,
            _ => false,
        }
    }
}

impl<M, S> State<M, S> {
    pub fn new(
        peer: Rc<Peer>,
        room: Rc<Room>,
        common_database: Rc<web_sys::IdbDatabase>,
        room_database: Rc<web_sys::IdbDatabase>,
    ) -> Self {
        let peers = hash_set! {peer.id()};
        let mut block_field = block::Field::new();
        let chat = chat::State::new(&mut block_field);
        let texture = block_field.add(block::table::Texture::new(&[4096, 4096], [20.0, 20.0]));
        let table = block_field.add(block::Table::new(texture, [20.0, 20.0], "テーブル"));
        let world = block_field.add(block::World::new(table));

        assert!(block_field.get::<block::World>(&world).is_some());

        Self {
            peer: peer,
            room: room,
            peers: hash_set! {},
            personal_data: model::PersonalData::new(),
            resource: Resource::new(),
            block_field,
            chat,
            world,
            table: table::State::new(),
            camera: Camera::new(),
            renderer: None,
            pixel_ratio: 1.0,
            canvas_size: [0.0, 0.0],
            speech_bubble: speech_bubble::State::new(),
            contextmenu: None,
            headermenu: None,
            modeless: modeless::Collection::new(),
            modal: vec![],
            dicebot: dicebot::State::new(),
            common_database,
            room_database,
            cmd_queue: model::CmdQueue::new(),
        }
    }

    pub fn peer(&self) -> Rc<Peer> {
        Rc::clone(&self.peer)
    }

    pub fn room(&self) -> Rc<Room> {
        Rc::clone(&self.room)
    }

    pub fn peers_mut(&mut self) -> &mut HashSet<String> {
        &mut self.peers
    }

    pub fn personal_data(&self) -> &model::PersonalData {
        &self.personal_data
    }

    pub fn personal_data_mut(&mut self) -> &mut model::PersonalData {
        &mut self.personal_data
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    pub fn resource_mut(&mut self) -> &mut Resource {
        &mut self.resource
    }

    pub fn block_field(&self) -> &block::Field {
        &self.block_field
    }

    pub fn block_field_mut(&mut self) -> &mut block::Field {
        &mut self.block_field
    }

    pub fn chat(&self) -> &chat::State {
        &self.chat
    }

    pub fn chat_mut(&mut self) -> &mut chat::State {
        &mut self.chat
    }

    pub fn chat_block(&self) -> Option<&block::Chat> {
        self.block_field.get::<block::Chat>(self.chat.block_id())
    }

    pub fn update_chat_block(&mut self, timestamp: Option<f64>, f: impl FnOnce(&mut block::Chat)) {
        self.block_field.update(self.chat.block_id(), timestamp, f);
    }

    pub fn selecting_chat_tab_id(&self) -> Option<&BlockId> {
        self.chat_block().and_then(|chat| {
            let idx = self.chat.selecting_tab_idx();
            if idx < chat.len() {
                chat.get(idx)
            } else {
                chat.get(chat.len() - 1)
            }
        })
    }

    pub fn selecting_chat_tab_block(&self) -> Option<&block::chat::Tab> {
        self.selecting_chat_tab_id()
            .and_then(|tab_id| self.block_field.get::<block::chat::Tab>(tab_id))
    }

    pub fn update_selecting_chat_tab_block(
        &mut self,
        timestamp: Option<f64>,
        f: impl FnOnce(&mut block::chat::Tab),
    ) {
        if let Some(block_id) = self.selecting_chat_tab_id() {
            self.block_field.update(&block_id.clone(), timestamp, f);
        }
    }

    pub fn world(&self) -> &BlockId {
        &self.world
    }

    pub fn selecting_table(&self) -> Option<&BlockId> {
        self.block_field
            .get::<block::World>(&self.world)
            .map(|world| world.selecting_table())
    }

    pub fn update_world(
        &mut self,
        timestamp: Option<f64>,
        f: impl FnOnce(&mut block::World) + 'static,
    ) {
        self.block_field.update(&self.world, timestamp, f);
    }

    pub fn table(&self) -> &table::State {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut table::State {
        &mut self.table
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn renderer(&self) -> Option<&Renderer> {
        self.renderer.as_ref()
    }

    pub fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            renderer.render(
                &self.block_field,
                &self.world,
                &self.camera,
                &self.resource,
                &self.canvas_size,
                &self.table.floating_object(),
            );
        }
    }

    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = Some(renderer);
    }

    pub fn table_position(
        &self,
        vertex: &[f32; 3],
        movement: &[f32; 3],
        is_billboard: bool,
    ) -> [f32; 3] {
        Renderer::table_position(
            vertex,
            movement,
            &self.camera,
            &self.canvas_size,
            is_billboard,
        )
    }

    pub fn pixel_ratio(&self) -> f32 {
        self.pixel_ratio
    }

    pub fn canvas_size(&self) -> &[f32; 2] {
        &self.canvas_size
    }

    pub fn set_canvas_size(&mut self, canvas_size: [f32; 2]) {
        self.canvas_size = canvas_size;
    }

    pub fn speech_bubble(&self) -> &speech_bubble::State {
        &self.speech_bubble
    }

    pub fn contextmenu(&self) -> Option<&contextmenu::State> {
        self.contextmenu.as_ref()
    }

    pub fn open_contextmenu(
        &mut self,
        grobal_position: [f64; 2],
        canvas_position: [f64; 2],
        payload: Contextmenu,
    ) {
        self.contextmenu = Some(contextmenu::State::new(
            grobal_position,
            canvas_position,
            payload,
        ));
    }

    pub fn close_contextmenu(&mut self) {
        self.contextmenu = None;
    }

    pub fn headermenu(&self) -> Option<&headermenu::State> {
        self.headermenu.as_ref()
    }

    pub fn set_headermenu(&mut self, headermenu: Option<headermenu::State>) {
        self.headermenu = headermenu;
    }

    pub fn modeless(&self) -> &modeless::Collection<Modeless> {
        &self.modeless
    }

    pub fn open_modeless(&mut self, modeless: Modeless) {
        if modeless.is_chat() {
            let chat_modeless_id = self.modeless.iter().find_map(|(id, m)| {
                m.as_ref()
                    .and_then(|m| if m.is_chat() { Some(id) } else { None })
            });
            if let Some(chat_modeless_id) = chat_modeless_id {
                self.modeless.focus(chat_modeless_id);
            } else {
                let mut modeless = model::Modeless::new(modeless);
                modeless.set_position_r(0.0, 50.0);
                modeless.set_size_r(40.0, 50.0);
                self.modeless.open(modeless);
            }
        } else {
            let modeless = model::Modeless::new(modeless);
            self.modeless.open(modeless);
        };
    }

    pub fn open_modeless_with_position(&mut self, modeless: Modeless, pos: [f64; 2]) {
        let mut modeless = model::Modeless::new(modeless);
        modeless.set_position(pos[0], pos[1]);
        self.modeless.open(modeless);
    }

    pub fn focus_modeless(&mut self, modeless_id: ModelessId) {
        self.modeless.focus(modeless_id);
    }

    pub fn grub_modeless(
        &mut self,
        modeless_id: ModelessId,
        mouse_position: [f64; 2], // x, y
        movable: [bool; 4],       // top, left, bottom, right
    ) {
        self.modeless.get_mut(modeless_id).map(|m| {
            m.grub(mouse_position[0], mouse_position[1]);
            m.set_movable(movable[0], movable[1], movable[2], movable[3]);
        });
    }

    pub fn drag_modeless(&mut self, modeless_id: ModelessId, mouse_position: [f64; 2]) {
        self.modeless.get_mut(modeless_id).map(|m| {
            m.move_with_mouse_pos(mouse_position[0], mouse_position[1]);
        });
    }

    pub fn drop_modeless(&mut self, modeless_id: ModelessId) {
        self.modeless.get_mut(modeless_id).map(|m| {
            m.drop();
        });
    }

    pub fn close_modeless(&mut self, modeless_id: ModelessId) {
        self.modeless.close(modeless_id);
    }

    pub fn outline_modeless(&mut self, modeless_id: ModelessId, color: Option<Color>) {
        self.modeless
            .get_mut(modeless_id)
            .map(|m| match m.as_mut() {
                Modeless::Object { outlined, .. } => {
                    *outlined = color;
                }
                _ => (),
            });
    }

    pub fn set_modeless_focused_tab(&mut self, modeless_id: ModelessId, tab_idx: usize) {
        if let Some(modeless) = self.modeless.get_mut(modeless_id) {
            match modeless.as_mut() {
                Modeless::Object { focused, .. } => {
                    *focused = tab_idx;
                }
                _ => (),
            }
        }
    }

    pub fn remove_modeless_tab(
        &mut self,
        modeless_id: ModelessId,
        tab_idx: usize,
    ) -> Option<BlockId> {
        let mut remove_modeless_flag = false;

        let result = self
            .modeless
            .get_mut(modeless_id)
            .and_then(|m| match m.as_mut() {
                Modeless::Object { tabs, focused, .. } => {
                    if *focused >= tab_idx {
                        *focused = if *focused > 0 { *focused - 1 } else { 0 };
                    }
                    if let Some(block_id) = tabs.get(tab_idx) {
                        let block_id = block_id.clone();
                        tabs.remove(tab_idx);
                        remove_modeless_flag = tabs.is_empty();
                        Some(block_id)
                    } else {
                        None
                    }
                }
                _ => None,
            });

        if remove_modeless_flag {
            self.modeless.close(modeless_id);
        }

        result
    }

    pub fn add_modeless_tab(&mut self, modeless_id: ModelessId, block_id: BlockId) {
        if let Some(tabs) = self
            .modeless
            .get_mut(modeless_id)
            .and_then(|m| match m.as_mut() {
                Modeless::Object { tabs, .. } => Some(tabs),
                _ => None,
            })
        {
            tabs.push(block_id);
        }
    }

    pub fn set_modeless_parent(&mut self, element: Option<web_sys::Element>) {
        self.modeless.set_parent(element);
    }

    pub fn modal(&self) -> &Vec<Modal> {
        &self.modal
    }

    pub fn open_modal(&mut self, modal: Modal) {
        self.modal.push(modal);
    }

    pub fn close_modal(&mut self) {
        self.modal.pop();
    }

    pub fn dicebot(&self) -> &dicebot::State {
        &self.dicebot
    }

    pub fn dicebot_mut(&mut self) -> &mut dicebot::State {
        &mut self.dicebot
    }

    pub fn dequeue(&mut self) -> Cmd<M, S> {
        self.cmd_queue.dequeue()
    }

    pub fn enqueue(&mut self, cmd: Cmd<M, S>) {
        self.cmd_queue.enqueue(cmd);
    }
}
