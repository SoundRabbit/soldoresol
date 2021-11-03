use super::block_trait::DisplayNamed;
use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;

#[derive(Clone)]
pub struct CharacterTexture {
    name: String,
    texture_id: Option<ResourceId>,
    height: f32,
}

#[derive(Clone)]
pub struct Character {
    size: f32,
    name: String,
    display_name: String,
    description: String,
    position: [f32; 3],
    textures: SelectList<CharacterTexture>,
    properties: Vec<BlockId>,
    name_color: Pallet,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: 1.0,
            name: String::from(""),
            display_name: String::from(""),
            description: String::from(""),
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
            name_color: Pallet::gray(9).a(100),
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

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    pub fn name_color(&self) -> &Pallet {
        &self.name_color
    }

    pub fn set_name_color(&mut self, color: Pallet) {
        self.name_color = color;
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
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

impl DisplayNamed for Character {
    fn display_name(&self) -> &String {
        self.display_name()
    }

    fn set_display_name(&mut self, name: String) {
        self.set_display_name(name);
    }
}

impl CharacterTexture {
    async fn pack_to_toml(&self) -> toml::Value {
        let mut packed = toml::value::Table::new();

        packed.insert(String::from("name"), toml::Value::String(self.name.clone()));

        if let Some(texture_id) = &self.texture_id {
            packed.insert(
                String::from("texture_id"),
                toml::Value::String(texture_id.to_string()),
            );
        }

        packed.insert(
            String::from("height"),
            toml::Value::Float(self.height as f64),
        );
        toml::Value::Table(packed)
    }

    async fn unpack_from_toml(packed: toml::Value) -> Self {
        let mut unpacked = Self {
            name: String::new(),
            texture_id: None,
            height: 1.0,
        };

        if let toml::Value::Table(mut packed) = packed {
            if let Some(toml::Value::String(name)) = packed.remove("name") {
                unpacked.name = name;
            }
            if let Some(toml::Value::String(texture_id)) = packed.remove("texture_id") {
                if let Some(texture_id) = ResourceId::from_str(&texture_id) {
                    unpacked.texture_id = Some(texture_id);
                }
            }
            if let Some(toml::Value::Float(height)) = packed.remove("height") {
                unpacked.height = height as f32;
            }
        }

        unpacked
    }
}

impl Character {
    pub async fn pack_to_toml(&self) -> toml::Value {
        let mut packed = toml::value::Table::new();

        packed.insert(String::from("size"), toml::Value::Float(self.size as f64));
        packed.insert(String::from("name"), toml::Value::String(self.name.clone()));
        packed.insert(
            String::from("description"),
            toml::Value::String(self.description.clone()),
        );
        packed.insert(
            String::from("display_name"),
            toml::Value::String(self.display_name.clone()),
        );

        let props = {
            let mut props = toml::value::Array::new();

            for prop_id in self.properties.iter() {
                props.push(toml::Value::String(prop_id.to_string()));
            }

            props
        };
        packed.insert(String::from("propaties"), toml::Value::Array(props));

        let textures = {
            let mut textures = toml::value::Table::new();

            textures.insert(
                String::from("_selected_idx"),
                toml::Value::Integer(self.textures.selected_idx() as i64),
            );

            let payload = {
                let mut payload = toml::value::Array::new();

                for texture in self.textures.iter() {
                    payload.push(texture.pack_to_toml().await);
                }

                payload
            };
            textures.insert(String::from("_payload"), toml::Value::Array(payload));

            textures
        };
        packed.insert(String::from("textures"), toml::Value::Table(textures));

        toml::Value::Table(packed)
    }

    pub async fn unpack_from_toml(packed: toml::Value) -> Self {
        let mut unpacked = Self::new();

        if let toml::Value::Table(mut packed) = packed {
            if let Some(toml::Value::Float(size)) = packed.remove("size") {
                unpacked.size = size as f32;
            }
            if let Some(toml::Value::String(name)) = packed.remove("name") {
                unpacked.name = name;
            }
            if let Some(toml::Value::String(description)) = packed.remove("description") {
                unpacked.description = description;
            }
            if let Some(toml::Value::String(display_name)) = packed.remove("display_name") {
                unpacked.display_name = display_name;
            }
            if let Some(toml::Value::Array(packed_props)) = packed.remove("propaties") {
                let mut props = vec![];

                for packed_prop_id in packed_props {
                    if let toml::Value::String(prop_id) = packed_prop_id {
                        if let Some(prop_id) = BlockId::from_str(&prop_id) {
                            props.push(prop_id);
                        }
                    }
                }

                unpacked.properties = props;
            }
            if let Some(toml::Value::Table(mut textures)) = packed.remove("textures") {
                let selected_idx =
                    if let Some(toml::Value::Integer(x)) = textures.remove("_selected_idx") {
                        x.max(0) as usize
                    } else {
                        0
                    };

                let payload =
                    if let Some(toml::Value::Array(textures)) = textures.remove("_payload") {
                        let mut payload = vec![];

                        for texture in textures {
                            payload.push(CharacterTexture::unpack_from_toml(texture).await);
                        }

                        payload
                    } else {
                        vec![]
                    };

                if payload.len() > 0 {
                    let selected_idx = selected_idx.min(payload.len());
                    unpacked.textures = SelectList::new(payload, selected_idx);
                }
            }
        }

        unpacked
    }
}
