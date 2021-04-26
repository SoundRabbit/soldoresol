use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::clone_of::CloneOf;
use crate::libs::select_list::SelectList;

pub struct CharacterTexture {
    name: String,
    texture_id: Option<ResourceId>,
    height: f32,
}

impl Clone for CharacterTexture {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            texture_id: self.texture_id.as_ref().map(|x| ResourceId::clone(x)),
            height: self.height,
        }
    }
}

pub struct Character {
    size: f32,
    name: String,
    display_name: String,
    position: [f32; 3],
    textures: SelectList<CharacterTexture>,
    properties: Vec<BlockId>,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: 1.0,
            name: String::from(""),
            display_name: String::from(""),
            position: [0.0, 0.0, 0.0],
            textures: SelectList::new(
                vec![CharacterTexture {
                    name: String::from("[default]"),
                    texture_id: None,
                    height: 1.0,
                }],
                0,
            ),
            properties: vec![],
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            size: this.size,
            name: this.name.clone(),
            display_name: this.display_name.clone(),
            position: this.position.clone(),
            textures: SelectList::clone_of(&this.textures),
            properties: this.properties.iter().map(BlockId::clone).collect(),
        }
    }

    pub fn size(&self) -> f32 {
        self.size
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn current_tex_height(&self) -> f32 {
        if let Some(tex) = self.textures.selected() {
            tex.height
        } else {
            1.0
        }
    }

    pub fn set_tex_height(&mut self, tex_idx: usize, height: f32) {
        if let Some(tex) = self.textures.get_mut(tex_idx) {
            tex.height = height;
        }
    }

    pub fn current_tex_id(&self) -> Option<&ResourceId> {
        if let Some(tex) = self.textures.selected() {
            tex.texture_id.as_ref()
        } else {
            None
        }
    }

    pub fn set_tex_id(&mut self, tex_idx: usize, tex_id: Option<ResourceId>) {
        if let Some(tex) = self.textures.get_mut(tex_idx) {
            tex.texture_id = tex_id;
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
        self.textures.iter().map(|tex| tex.name.as_str()).collect()
    }

    pub fn current_tex_name(&self) -> &str {
        self.textures
            .selected()
            .map(|tex| tex.name.as_str())
            .unwrap_or("")
    }

    pub fn current_tex_idx(&self) -> usize {
        self.textures.selected_idx()
    }

    pub fn set_current_tex_idx(&mut self, idx: usize) {
        self.textures.set_selected_idx(idx);
    }

    pub fn add_tex_to_select(&mut self) {
        self.textures.push(CharacterTexture {
            name: String::from("新規立ち絵"),
            texture_id: None,
            height: self.size,
        });
        self.textures.set_selected_idx(self.textures.len() - 1);
    }

    pub fn remove_tex(&mut self, tex_idx: usize) {
        if self.textures.len() > 1 {
            self.textures.remove(tex_idx);
            if self.textures.selected_idx() >= self.textures.len() {
                self.textures.set_selected_idx(self.textures.len() - 1);
            }
        }
    }

    pub fn set_tex_name(&mut self, tex_idx: usize, tex_name: String) {
        if let Some(tex) = self.textures.get_mut(tex_idx) {
            tex.name = tex_name;
        }
    }

    pub fn properties(&self) -> impl Iterator<Item = &BlockId> {
        self.properties.iter()
    }

    pub fn add_property(&mut self, property_id: BlockId) {
        self.properties.push(property_id);
    }
}
