use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::color::Pallet;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Table {
    name: Rc<String>,
    size: [f32; 2],
    is_bind_to_grid: bool,
    is_showing_grid: bool,
    drawing_texture_id: BlockId,
    drawed_texture_id: BlockId,
    drawing_terran_id: BlockId,
    drawed_terran_id: BlockId,
    terran_height: f32,
    background_texture_id: Option<ResourceId>,
    background_color: Pallet,
    grid_color: Pallet,
    boxblocks: Vec<BlockId>,
    pointlights: Vec<BlockId>,
    env_light_intensity: f32,
}

impl Table {
    pub fn new(
        drawing_texture_id: BlockId,
        drawed_texture_id: BlockId,
        drawing_terran_id: BlockId,
        drawed_terran_id: BlockId,
        size: [f32; 2],
        name: impl Into<String>,
    ) -> Self {
        Self {
            name: Rc::new(name.into()),
            size,
            is_bind_to_grid: true,
            is_showing_grid: true,
            drawing_texture_id,
            drawed_texture_id,
            drawing_terran_id,
            drawed_terran_id,
            terran_height: 1.0,
            background_texture_id: None,
            background_color: Pallet::gray(0).a(0),
            grid_color: Pallet::gray(9).a(100),
            boxblocks: vec![],
            pointlights: vec![],
            env_light_intensity: 1.0,
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            name: Rc::clone(&this.name),
            size: this.size.clone(),
            is_bind_to_grid: this.is_bind_to_grid,
            is_showing_grid: this.is_showing_grid,
            drawing_texture_id: BlockId::clone(&this.drawing_texture_id),
            drawed_texture_id: BlockId::clone(&this.drawed_texture_id),
            drawing_terran_id: BlockId::clone(&this.drawing_terran_id),
            drawed_terran_id: BlockId::clone(&this.drawed_terran_id),
            terran_height: this.terran_height,
            background_texture_id: this
                .background_texture_id
                .as_ref()
                .map(|r_id| ResourceId::clone(r_id)),
            background_color: this.background_color,
            grid_color: this.grid_color,
            boxblocks: this.boxblocks.iter().map(BlockId::clone).collect(),
            pointlights: this.pointlights.iter().map(BlockId::clone).collect(),
            env_light_intensity: this.env_light_intensity,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Rc::new(name);
    }

    pub fn size(&self) -> &[f32; 2] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 2]) {
        self.size = size;
    }

    pub fn is_bind_to_grid(&self) -> bool {
        self.is_bind_to_grid
    }

    pub fn set_is_bind_to_grid(&mut self, is_bind_to_grid: bool) {
        self.is_bind_to_grid = is_bind_to_grid;
    }

    pub fn is_showing_grid(&self) -> bool {
        self.is_showing_grid
    }

    pub fn set_is_showing_grid(&mut self, is_showing_grid: bool) {
        self.is_showing_grid = is_showing_grid;
    }

    pub fn drawing_texture_id(&self) -> &BlockId {
        &self.drawing_texture_id
    }

    pub fn drawed_texture_id(&self) -> &BlockId {
        &self.drawed_texture_id
    }

    pub fn drawing_terran_id(&self) -> &BlockId {
        &self.drawing_terran_id
    }

    pub fn drawed_terran_id(&self) -> &BlockId {
        &self.drawed_terran_id
    }

    pub fn background_texture_id(&self) -> Option<&ResourceId> {
        self.background_texture_id.as_ref()
    }

    pub fn set_background_texture_id(&mut self, background_texture_id: Option<ResourceId>) {
        self.background_texture_id = background_texture_id
    }

    pub fn background_color(&self) -> &Pallet {
        &self.background_color
    }

    pub fn set_background_color(&mut self, color: Pallet) {
        self.background_color = color;
    }

    pub fn grid_color(&self) -> &Pallet {
        &self.grid_color
    }

    pub fn set_grid_color(&mut self, color: Pallet) {
        self.grid_color = color;
    }

    pub fn boxblocks(&self) -> impl Iterator<Item = &BlockId> {
        self.boxblocks.iter()
    }

    pub fn add_boxblock(&mut self, boxblock_id: BlockId) {
        self.boxblocks.push(boxblock_id);
    }

    pub fn pointlights(&self) -> impl Iterator<Item = &BlockId> {
        self.pointlights.iter()
    }

    pub fn add_pointlight(&mut self, pointlight_id: BlockId) {
        self.pointlights.push(pointlight_id);
    }

    pub fn env_light_intensity(&self) -> f32 {
        self.env_light_intensity
    }

    pub fn set_env_light_intensity(&mut self, env_light_intensity: f32) {
        self.env_light_intensity = env_light_intensity;
    }

    pub fn terran_height(&self) -> f32 {
        self.terran_height
    }

    pub fn set_terran_height(&mut self, terran_height: f32) {
        self.terran_height = terran_height;
    }

    pub async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    pub async fn unpack(_val: JsValue) -> Self {
        unimplemented!();
    }
}
