use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, resource::ResourceId, Color, JsObject, Promise};
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

mod texture;

pub use texture::Texture;

#[derive(Clone)]
pub struct Horizon {
    radius: f32,
    color: Color,
}

#[derive(Clone)]
pub struct Table {
    name: String,
    size: [f32; 2],
    is_bind_to_grid: bool,
    is_showing_grid: bool,
    drawing_texture_id: BlockId,
    image_texture_id: Option<ResourceId>,
    horizon: Option<Horizon>,
    tablemasks: Vec<BlockId>,
    areas: Vec<BlockId>,
    boxblocks: Vec<BlockId>,
}

impl Horizon {
    fn to_jsvalue(&self) -> JsValue {
        let data = array![self.radius, self.color.to_u32()];
        data.into()
    }

    fn from_jsvalue(data: &JsValue) -> Option<Self> {
        let data = js_sys::Array::from(&data);
        let radius = data.get(0).as_f64().map(|x| x as f32);
        let color = data.get(1).as_f64().map(|x| Color::from(x as u32));
        if let (Some(radius), Some(color)) = (radius, color) {
            Some(Self { radius, color })
        } else {
            None
        }
    }
}

impl Table {
    pub fn new(drawing_texture_id: BlockId, size: [f32; 2], name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size,
            is_bind_to_grid: true,
            is_showing_grid: true,
            drawing_texture_id,
            image_texture_id: None,
            horizon: None,
            tablemasks: vec![],
            areas: vec![],
            boxblocks: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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

    pub fn image_texture_id(&self) -> Option<&ResourceId> {
        self.image_texture_id.as_ref()
    }

    pub fn set_image_texture_id(&mut self, image_texture_id: Option<ResourceId>) {
        self.image_texture_id = image_texture_id
    }

    pub fn tablemasks(&self) -> impl Iterator<Item = &BlockId> {
        self.tablemasks.iter()
    }

    pub fn add_tablemask(&mut self, tablemask: BlockId) {
        self.tablemasks.push(tablemask);
    }

    pub fn remove_tablemask(&mut self, tablemask: &BlockId) {
        if let Some(idx) = self.tablemasks.iter().position(|x| x == tablemask) {
            self.tablemasks.remove(idx);
        }
    }

    pub fn areas(&self) -> impl Iterator<Item = &BlockId> {
        self.areas.iter()
    }

    pub fn add_area(&mut self, area: BlockId) {
        self.areas.push(area);
    }

    pub fn remove_area(&mut self, area: &BlockId) {
        if let Some(idx) = self.areas.iter().position(|x| x == area) {
            self.areas.remove(idx);
        }
    }

    pub fn boxblocks(&self) -> impl Iterator<Item = &BlockId> {
        self.boxblocks.iter()
    }

    pub fn add_boxblock(&mut self, boxblock: BlockId) {
        self.boxblocks.push(boxblock);
    }

    pub fn remove_boxblock(&mut self, boxblock: &BlockId) {
        if let Some(idx) = self.boxblocks.iter().position(|x| x == boxblock) {
            self.boxblocks.remove(idx);
        }
    }
}

impl Block for Table {
    fn pack(&self) -> Promise<JsValue> {
        let tablemasks = array![];
        for id in &self.tablemasks {
            tablemasks.push(&id.to_jsvalue());
        }

        let areas = array![];
        for id in &self.areas {
            areas.push(&id.to_jsvalue());
        }

        let boxblocks = array![];
        for id in &self.boxblocks {
            boxblocks.push(&id.to_jsvalue());
        }

        let data = object! {
            name: &self.name,
            size: array![self.size[0], self.size[1]],
            is_bind_to_grid: self.is_bind_to_grid,
            is_showing_grid: self.is_showing_grid,
            drawing_texture_id: self.drawing_texture_id.to_jsvalue(),
            image_texture_id: self.image_texture_id.as_ref().map(|id| id.to_jsvalue()),
            horizon: self.horizon.as_ref().map(|x| x.to_jsvalue()),
            tablemasks: tablemasks,
            areas: areas,
            boxblocks: boxblocks
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let name = val.get("name").and_then(|name| name.as_string());
            let size = val.get("size").map(|p| {
                let p: js_sys::Object = p.into();
                js_sys::Array::from(p.as_ref())
            });
            let is_bind_to_grid = val.get("is_bind_to_grid").and_then(|i| i.as_bool());
            let is_showing_grid = val.get("is_showing_grid").and_then(|i| i.as_bool());
            let drawing_texture_id = val
                .get("drawing_texture_id")
                .and_then(|id| U128Id::from_jsvalue(&id))
                .map(|id| field.block_id(id));
            let image_texture_id = Some(
                val.get("image_texture_id")
                    .and_then(|id| U128Id::from_jsvalue(&id)),
            );
            let horizon = val.get("horizon").and_then(|x| Horizon::from_jsvalue(&x));
            let tablemasks = val.get("tablemasks").map(|p| {
                let p: js_sys::Object = p.into();
                js_sys::Array::from(p.as_ref())
            });
            let areas = val.get("areas").map(|p| {
                let p: js_sys::Object = p.into();
                js_sys::Array::from(p.as_ref())
            });
            let boxblocks = val.get("boxblocks").map(|p| {
                let p: js_sys::Object = p.into();
                js_sys::Array::from(p.as_ref())
            });
            if let (
                Some(name),
                Some(size),
                Some(is_bind_to_grid),
                Some(is_showing_grid),
                Some(drawing_texture_id),
                Some(image_texture_id),
                Some(raw_tablemasks),
                Some(raw_areas),
                Some(raw_boxblocks),
            ) = (
                name,
                size,
                is_bind_to_grid,
                is_showing_grid,
                drawing_texture_id,
                image_texture_id,
                tablemasks,
                areas,
                boxblocks,
            ) {
                let size = if let (Some(x), Some(y)) = (
                    size.get(0).as_f64().map(|x| x as f32),
                    size.get(1).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y])
                } else {
                    None
                };

                let mut tablemasks = vec![];
                for id in raw_tablemasks.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        tablemasks.push(id);
                    }
                }

                let mut areas = vec![];
                for id in raw_areas.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        areas.push(id);
                    }
                }

                let mut boxblocks = vec![];
                for id in raw_boxblocks.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        boxblocks.push(id);
                    }
                }

                if let Some(size) = size {
                    Some(Box::new(Self {
                        name,
                        size,
                        is_bind_to_grid,
                        is_showing_grid,
                        drawing_texture_id,
                        image_texture_id,
                        horizon,
                        tablemasks,
                        areas,
                        boxblocks,
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        Promise::new(move |resolve| resolve(self_))
    }

    fn dependents(&self, field: &Field) -> HashSet<BlockId> {
        let mut deps = set! {};

        for block_id in &self.tablemasks {
            if let Some(block) = field.get::<super::Table>(block_id) {
                let block_deps = block.dependents(field);
                for block_dep in block_deps {
                    deps.insert(block_dep);
                }
                deps.insert(block_id.clone());
            }
        }

        for block_id in &self.areas {
            if let Some(block) = field.get::<super::Character>(block_id) {
                let block_deps = block.dependents(field);
                for block_dep in block_deps {
                    deps.insert(block_dep);
                }
                deps.insert(block_id.clone());
            }
        }

        for block_id in &self.boxblocks {
            if let Some(block) = field.get::<super::Memo>(block_id) {
                let block_deps = block.dependents(field);
                for block_dep in block_deps {
                    deps.insert(block_dep);
                }
                deps.insert(block_id.clone());
            }
        }

        deps
    }
}
