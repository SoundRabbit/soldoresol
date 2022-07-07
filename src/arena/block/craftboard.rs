use super::super::resource::ImageData;
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Cubebox;
use super::util::Pack;
use super::BlockMut;
use super::BlockRef;
use super::Terran;
use crate::libs::color::Pallet;
use std::collections::{HashMap, HashSet};

type Textures = Cubebox<Option<BlockRef<ImageData>>>;

block! {
    [pub Craftboard(constructor, pack, component)]
    (is_bind_to_grid): bool;
    (position): [f64; 3];
    (terran): BlockMut<Terran>;
    origin: BlockMut<Component> = BlockMut::<Component>::none();
    name: String = String::new();
    display_name: (String, String) = (String::from(""), String::from(""));
    size: [f64; 3] = [10.0, 10.0, 10.0];
    grid_color: Pallet = Pallet::gray(9).a(100);
    env_light_intensity: f64 = 1.0;
    is_fixed_position: bool = true;
    textures: Textures = Textures::with(|_| None);
}

impl Craftboard {
    pub fn origin(&self) -> &BlockMut<Component> {
        &self.origin
    }
    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }
    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }
    pub fn terran_mut(&mut self) -> &mut BlockMut<Terran> {
        &mut self.terran
    }
    pub fn terran(&self) -> &BlockMut<Terran> {
        &self.terran
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn set_display_name(&mut self, display_name: (Option<String>, Option<String>)) {
        if let Some(main) = display_name.0 {
            self.display_name.0 = main;
        }
        if let Some(sub) = display_name.1 {
            self.display_name.1 = sub;
        }
    }
    pub fn display_name(&self) -> &(String, String) {
        &self.display_name
    }
    pub fn set_size(&mut self, size: [f64; 3]) {
        self.size = size;
    }
    pub fn size(&self) -> &[f64; 3] {
        &self.size
    }
    pub fn set_grid_color(&mut self, grid_color: Pallet) {
        self.grid_color = grid_color;
    }
    pub fn grid_color(&self) -> &Pallet {
        &self.grid_color
    }
    pub fn env_light_intensity(&self) -> f64 {
        self.env_light_intensity
    }
    pub fn set_is_fixed_position(&mut self, is_fixed_position: bool) {
        self.is_fixed_position = is_fixed_position;
    }
    pub fn is_fixed_position(&self) -> bool {
        self.is_fixed_position
    }
    pub fn set_is_bind_to_grid(&mut self, is_bind_to_grid: bool) {
        self.is_bind_to_grid = is_bind_to_grid;
    }
    pub fn is_bind_to_grid(&self) -> bool {
        self.is_bind_to_grid
    }
    pub fn textures(&self) -> &Textures {
        &self.textures
    }
    pub fn set_textures(&mut self, textures: Textures) {
        self.textures = textures;
    }
}

impl BlockMut<Component> {
    pub fn create_clone(&self) -> Option<Craftboard> {
        self.map(|component| {
            let mut cloned = component.origin.clone();
            cloned.origin = BlockMut::clone(self);
            cloned
        })
    }
}

impl Clone for Craftboard {
    fn clone(&self) -> Self {
        Self {
            origin: BlockMut::none(),
            terran: BlockMut::clone(&self.terran),
            is_bind_to_grid: self.is_bind_to_grid,
            position: self.position.clone(),
            size: self.size.clone(),
            env_light_intensity: self.env_light_intensity.clone(),
            grid_color: self.grid_color.clone(),
            textures: self.textures.clone(),
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            is_fixed_position: self.is_fixed_position,
        }
    }
}
