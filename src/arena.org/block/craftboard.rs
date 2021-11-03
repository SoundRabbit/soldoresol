use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::color::Pallet;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Craftboard {
    name: Rc<String>,
    size: [f32; 2],
    position: [f32; 3],
    is_bind_to_grid: bool,
    is_showing_grid: bool,
    drawing_terran_id: BlockId,
    drawed_terran_id: BlockId,
    terran_height: f32,
    grid_color: Pallet,
    env_light_intensity: f32,
    layer_group_id: BlockId,
}

impl Craftboard {
    pub fn new(
        layer_group_id: BlockId,
        drawing_terran_id: BlockId,
        drawed_terran_id: BlockId,
        size: [f32; 2],
        position: [f32; 3],
        name: impl Into<String>,
    ) -> Self {
        Self {
            name: Rc::new(name.into()),
            size,
            position,
            is_bind_to_grid: true,
            is_showing_grid: true,
            drawing_terran_id,
            drawed_terran_id,
            terran_height: 1.0,
            grid_color: Pallet::gray(9).a(100),
            env_light_intensity: 1.0,
            layer_group_id,
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

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
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

    pub fn drawing_terran_id(&self) -> &BlockId {
        &self.drawing_terran_id
    }

    pub fn drawed_terran_id(&self) -> &BlockId {
        &self.drawed_terran_id
    }

    pub fn grid_color(&self) -> &Pallet {
        &self.grid_color
    }

    pub fn set_grid_color(&mut self, color: Pallet) {
        self.grid_color = color;
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

    pub fn layer_group_id(&self) -> &BlockId {
        &self.layer_group_id
    }

    pub async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    pub async fn unpack(_val: JsValue) -> Self {
        unimplemented!();
    }
}
