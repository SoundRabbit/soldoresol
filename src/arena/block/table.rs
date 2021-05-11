use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::color::Pallet;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Terran {
    terran: HashMap<[i32; 3], Pallet>,
    min: [i32; 3],
    max: [i32; 3],
}

pub struct Table {
    name: Rc<String>,
    size: [f32; 2],
    is_bind_to_grid: bool,
    is_showing_grid: bool,
    drawing_texture_id: BlockId,
    drawed_texture_id: BlockId,
    background_texture_id: Option<ResourceId>,
    background_color: Pallet,
    grid_color: Pallet,
    boxblocks: Vec<BlockId>,
    pointlights: Vec<BlockId>,
    env_light_intensity: f32,
    terran: Terran,
}

impl Terran {
    pub fn new() -> Self {
        Self {
            terran: HashMap::new(),
            min: [0, 0, 0],
            max: [0, 0, 0],
        }
    }

    pub fn insert(&mut self, p: [i32; 3], c: Pallet) {
        if self.terran.is_empty() {
            self.min = p.clone();
            self.max = p.clone();
        } else {
            self.min[0] = self.min[0].min(p[0]);
            self.min[1] = self.min[1].min(p[1]);
            self.min[2] = self.min[2].min(p[2]);
            self.max[0] = self.max[0].max(p[0]);
            self.max[1] = self.max[1].max(p[1]);
            self.max[2] = self.max[2].max(p[2]);
        }

        self.terran.insert(p, c);
    }

    pub fn get(&self, p: &[i32; 3]) -> Option<&Pallet> {
        self.terran.get(p)
    }

    pub fn min(&self) -> &[i32; 3] {
        &self.min
    }

    pub fn max(&self) -> &[i32; 3] {
        &self.max
    }
}

impl Table {
    pub fn new(
        drawing_texture_id: BlockId,
        drawed_texture_id: BlockId,
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
            background_texture_id: None,
            background_color: Pallet::gray(0).a(0),
            grid_color: Pallet::gray(9).a(100),
            boxblocks: vec![],
            pointlights: vec![],
            env_light_intensity: 1.0,
            terran: Terran::new(),
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
            background_texture_id: this
                .background_texture_id
                .as_ref()
                .map(|r_id| ResourceId::clone(r_id)),
            background_color: this.background_color,
            grid_color: this.grid_color,
            boxblocks: this.boxblocks.iter().map(BlockId::clone).collect(),
            pointlights: this.pointlights.iter().map(BlockId::clone).collect(),
            env_light_intensity: this.env_light_intensity,
            terran: this.terran.clone(),
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

    pub fn terran(&self) -> &Terran {
        &self.terran
    }

    pub fn terran_mut(&mut self) -> &mut Terran {
        &mut self.terran
    }

    pub async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    pub async fn unpack(_val: JsValue) -> Self {
        unimplemented!();
    }
}
