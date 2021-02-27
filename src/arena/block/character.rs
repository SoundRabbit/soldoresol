use crate::arena::resource::ResourceId;
use crate::libs::clone_of::CloneOf;
use crate::libs::select_list::SelectList;

pub struct CharacterTexture {
    name: String,
    texture_id: ResourceId,
    tex_scale: f32,
}

impl Clone for CharacterTexture {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            texture_id: ResourceId::clone(&self.texture_id),
            tex_scale: self.tex_scale,
        }
    }
}

pub struct Character {
    size: f32,
    default_tex_scale: f32,
    default_texture_id: Option<ResourceId>,
    name: String,
    position: [f32; 3],
    textures: SelectList<CharacterTexture>,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: 1.0,
            default_tex_scale: 1.0,
            default_texture_id: None,
            name: String::from(""),
            position: [0.0, 0.0, 0.0],
            textures: SelectList::new(vec![], 0),
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            size: this.size,
            default_tex_scale: this.default_tex_scale,
            default_texture_id: this
                .default_texture_id
                .as_ref()
                .map(|x| ResourceId::clone(x)),
            name: this.name.clone(),
            position: this.position.clone(),
            textures: SelectList::clone_of(&this.textures),
        }
    }

    pub fn size(&self) -> f32 {
        self.size
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn current_tex_scale(&self) -> f32 {
        if let Some(tex) = self.textures.selected() {
            tex.tex_scale
        } else {
            self.default_tex_scale
        }
    }

    pub fn set_tex_scale(&mut self, tex_idx: usize, tex_scale: f32) {
        if tex_idx == 0 {
            self.default_tex_scale = tex_scale;
        } else {
            if let Some(tex) = self.textures.get_mut(tex_idx - 1) {
                tex.tex_scale = tex_scale;
            }
        }
    }

    pub fn current_tex_id(&self) -> Option<&ResourceId> {
        if let Some(tex) = self.textures.selected() {
            Some(&tex.texture_id)
        } else {
            self.default_texture_id.as_ref()
        }
    }

    pub fn set_tex_id(&mut self, tex_idx: usize, tex_id: Option<ResourceId>) {
        if tex_idx == 0 {
            self.default_texture_id = tex_id;
        } else {
            if let (Some(tex), Some(tex_id)) = (self.textures.get_mut(tex_idx - 1), tex_id) {
                tex.texture_id = tex_id;
            }
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn tex_names(&self) -> Vec<&str> {
        let mut tex_names = vec!["[default]"];

        for tex in self.textures.iter() {
            tex_names.push(&tex.name);
        }

        tex_names
    }

    pub fn current_tex_name(&self) -> &str {
        if let Some(tex) = self.textures.selected() {
            &tex.name
        } else {
            "[default]"
        }
    }
}
