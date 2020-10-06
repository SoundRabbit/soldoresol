use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, resource::ResourceId, Color, JsObject, Promise};
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Character {
    size: [f32; 3],
    position: [f32; 3],
    texture_id: Option<ResourceId>,
    background_color: Color,
    name: String,
    property_id: BlockId,
    owner_client_id: String,
    is_hidden: bool,
}

impl Character {
    pub fn new(
        property_id: BlockId,
        name: impl Into<String>,
        owner_client_id: impl Into<String>,
    ) -> Self {
        Self {
            size: [1.0, 1.0, 0.0],
            position: [0.0, 0.0, 0.0],
            texture_id: None,
            background_color: Color::from(0),
            name: name.into(),
            property_id: property_id,
            owner_client_id: owner_client_id.into(),
            is_hidden: false,
        }
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 3]) {
        self.size = size;
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn texture_id(&self) -> Option<&ResourceId> {
        self.texture_id.as_ref()
    }

    pub fn set_texture_id(&mut self, texture_id: Option<ResourceId>) {
        self.texture_id = texture_id;
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn property_id(&self) -> &BlockId {
        &self.property_id
    }

    pub fn set_property_id(&mut self, property_id: BlockId) {
        self.property_id = property_id;
    }

    pub fn owner_client_id(&self) -> &String {
        &self.owner_client_id
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub fn set_is_hidden(&mut self, is_hidden: bool) {
        self.is_hidden = is_hidden;
    }

    pub fn is_showable(&self, client_id: &String) -> bool {
        if self.is_hidden {
            if *client_id == self.owner_client_id.as_ref() {
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

impl Block for Character {
    fn pack(&self) -> Promise<JsValue> {
        let size = array![self.size[0], self.size[1], self.size[2]];
        let position = array![self.position[0], self.position[1], self.position[2]];
        let texture_id = self
            .texture_id
            .as_ref()
            .map(|r| r.to_jsvalue())
            .unwrap_or(JsValue::undefined());
        let name = self.name();
        let property_id = self.property_id.to_jsvalue();
        let owner_client_id = self.owner_client_id();
        let is_hidden = self.is_hidden();

        let data = object! {
            size: size,
            position: position,
            texture_id: texture_id,
            name: name,
            property_id: property_id,
            owner_client_id: owner_client_id,
            is_hidden: is_hidden
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();

        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let size = val.get("size").map(|x| js_sys::Array::from(&x));
            let position = val.get("position").map(|x| js_sys::Array::from(&x));
            let texture_id = Some(val.get("texture_id").and_then(|x| U128Id::from_jsvalue(&x)));
            let name = val.get("name").and_then(|x| x.as_string());
            let property_id = val
                .get("property_id")
                .and_then(|x| U128Id::from_jsvalue(&x))
                .map(|x| field.block_id(x));
            let owner_client_id = val.get("owner_client_id").and_then(|x| x.as_string());
            let is_hidden = val.get("is_hidden").and_then(|x| x.as_bool());
            if let (
                Some(size),
                Some(position),
                Some(texture_id),
                Some(name),
                Some(property_id),
                Some(owner_client_id),
                Some(is_hidden),
            ) = (
                size,
                position,
                texture_id,
                name,
                property_id,
                owner_client_id,
                is_hidden,
            ) {
                let size = if let (Some(x), Some(y), Some(z)) = (
                    size.get(0).as_f64().map(|x| x as f32),
                    size.get(1).as_f64().map(|x| x as f32),
                    size.get(2).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y, z])
                } else {
                    None
                };

                let position = if let (Some(x), Some(y), Some(z)) = (
                    position.get(0).as_f64().map(|x| x as f32),
                    position.get(1).as_f64().map(|x| x as f32),
                    position.get(2).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y, z])
                } else {
                    None
                };
                if let (Some(size), Some(position)) = (size, position) {
                    Some(Box::new(Self {
                        size,
                        position,
                        texture_id,
                        background_color: Color::from(0),
                        name,
                        property_id,
                        owner_client_id,
                        is_hidden,
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

        if let Some(block) = field.get::<super::Property>(&self.property_id) {
            let block_deps = block.dependents(field);

            for block_dep in block_deps {
                deps.insert(block_dep);
            }

            deps.insert(self.property_id.clone());
        }

        deps
    }
}
