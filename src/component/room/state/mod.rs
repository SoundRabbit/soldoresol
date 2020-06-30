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
    Color, Resource,
};
use kagura::prelude::*;
use std::{collections::HashSet, rc::Rc};

pub enum Modeless {
    Object {
        tabs: Vec<BlockId>,
        focused: usize,
        outlined: Option<Color>,
        cover: Option<ModelessId>,
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

    pub fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            renderer.render(
                &self.block_field,
                &self.world,
                &self.camera,
                &self.resource,
                &self.canvas_size,
            );
        }
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

    pub fn modeless(&self) -> &modeless::Collection<Modeless> {
        &self.modeless
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

    pub fn covering_modeless(&self, modeless_id: ModelessId) -> Option<ModelessId> {
        self.modeless
            .get(modeless_id)
            .and_then(|m| match m.as_ref() {
                Modeless::Object { cover, .. } => cover.clone(),
                _ => None,
            })
    }

    pub fn set_covering_modeless(&mut self, modeless_id: ModelessId, covering: Option<ModelessId>) {
        self.modeless
            .get_mut(modeless_id)
            .map(|m| match m.as_mut() {
                Modeless::Object { cover, .. } => {
                    *cover = covering;
                }
                _ => (),
            });
    }

    pub fn merge_object_modeless(&mut self, a: ModelessId, b: ModelessId) {
        let mut a = match self.modeless.get_mut(a).map(|m| m.as_mut()) {
            Some(Modeless::Object { tabs, .. }) => tabs.drain(..).collect::<Vec<_>>(),
            _ => vec![],
        };
        if let Some(Modeless::Object { tabs, .. }) = self.modeless.get_mut(b).map(|m| m.as_mut()) {
            tabs.append(&mut a);
        }
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

    pub fn modal(&self) -> &Vec<Modal> {
        &self.modal
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
