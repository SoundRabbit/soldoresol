use super::BlockId;
use crate::arena::resource::ResourceId;

pub struct Character {
    size: f32,
    tex_scale: f32,
    texture_id: Option<ResourceId>,
    name: String,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: 1.0,
            tex_scale: 1.0,
            texture_id: None,
            name: String::from(""),
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            size: this.size,
            tex_scale: this.tex_scale,
            texture_id: this.texture_id.as_ref().map(|x| ResourceId::clone(x)),
            name: this.name.clone(),
        }
    }

    pub fn size(&self) -> f32 {
        self.size
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn tex_scale(&self) -> f32 {
        self.tex_scale
    }

    pub fn set_tex_scale(&mut self, tex_scale: f32) {
        self.tex_scale = tex_scale;
    }

    pub fn tex_id(&self) -> Option<&ResourceId> {
        self.texture_id.as_ref()
    }

    pub fn set_tex_id(&mut self, tex_id: Option<ResourceId>) {
        self.texture_id = tex_id;
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
