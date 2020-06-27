pub mod chat;
pub mod contextmenu;
pub mod dice_bot;
pub mod speech_bubble;
pub mod table;

pub use contextmenu::Contextmenu;

use crate::{
    block::{self, BlockId},
    model,
    model::modeless::{self, ModelessId},
    renderer::{Camera, Renderer},
    skyway::{Peer, Room},
    Resource,
};
use kagura::prelude::*;
use std::{collections::HashSet, rc::Rc};

pub enum Modeless {
    Object { tabs: Vec<BlockId>, focused: usize },
    Chat,
}

pub enum Modal {
    Resource,
    SelectTableImage,
    SelectCharacterImage,
    SelectPlayerImage,
    PersonalSetting,
    TablemaskColorPicker,
    SenderCharacterSelecter,
    TableSetting,
    ChatLog,
    ChatTabEditor,
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
    contextmenu: Option<contextmenu::State>,
    speech_bubble: speech_bubble::State,
    modeless: modeless::Collection<Modeless>,
    modal: Vec<Modal>,
    dice_bot: dice_bot::State,
    common_database: Rc<web_sys::IdbDatabase>,
    room_database: Rc<web_sys::IdbDatabase>,
    cmd_queue: model::CmdQueue<M, S>,
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
            contextmenu: None,
            speech_bubble: speech_bubble::State::new(),
            modeless: modeless::Collection::new(),
            modal: vec![],
            dice_bot: dice_bot::State::new(),
            common_database,
            room_database,
            cmd_queue: model::CmdQueue::new(),
        }
    }

    pub fn room(&self) -> Rc<Room> {
        Rc::clone(&self.room)
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

    pub fn chat_mut(&mut self) -> &mut chat::State {
        &mut self.chat
    }

    pub fn world(&self) -> &BlockId {
        &self.world
    }

    pub fn selecting_table(&self) -> Option<&BlockId> {
        self.block_field
            .get::<block::World>(&self.world)
            .map(|world| world.selecting_table())
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

    pub fn renderer_mut(&mut self) -> Option<&mut Renderer> {
        self.renderer.as_mut()
    }

    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = Some(renderer);
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

    pub fn open_modeless(&mut self, modeless: Modeless) {
        self.modeless.open(model::Modeless::new(modeless));
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

    pub fn open_modal(&mut self, modal: Modal) {
        self.modal.push(modal);
    }

    pub fn close_modal(&mut self) {
        self.modal.pop();
    }

    pub fn dequeue(&mut self) -> Cmd<M, S> {
        self.cmd_queue.dequeue()
    }

    pub fn enqueue(&mut self, cmd: Cmd<M, S>) {
        self.cmd_queue.enqueue(cmd);
    }
}
