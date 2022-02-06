uses! {}

use super::super::resource::ImageData;
use super::util::Pack;
use super::BlockRef;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use regex::Regex;

block! {
    [pub ChatPallet(constructor, pack)]
    data: String = String::from("");
    defs: Vec<(Regex, String)> = vec![];
    index: Vec<(String, Vec<String>)> = vec![];
    match_index: Regex = Regex::new(r"\A//---(.*)(\n|\z)").unwrap();
    match_def: Regex = Regex::new(r"\A//(.+)=((.*\\\n)*(.*))(\n|\z)") .unwrap();
    match_line: Regex = Regex::new(r"\A(.*)(\n|\z)").unwrap();
    match_nl: Regex = Regex::new(r"([^\\])(\\\\)*\\n").unwrap();
}

impl ChatPallet {
    pub fn set_data(&mut self, mut data: String) {
        self.data = data.clone();
        self.index.clear();

        let mut index_name = String::from("");
        let mut index_items = vec![];

        while !data.is_empty() {
            if let Some(captures) = self.match_index.captures(&data) {
                if !index_items.is_empty() || !index_name.is_empty() {
                    self.index.push((index_name, index_items));
                }

                index_name = String::from(captures.get(1).unwrap().as_str());
                index_items = vec![];

                data = self.match_index.replace(&data, "").into();
            } else if let Some(captures) = self.match_def.captures(&data) {
                let regex = String::from(r"\A") + captures.get(1).unwrap().as_str() + r"\z";
                if let Ok(regex) = Regex::new(regex.as_str()) {
                    self.defs
                        .push((regex, String::from(captures.get(2).unwrap().as_str())));
                }

                data = self.match_def.replace(&data, "").into();
            } else if let Some(captures) = self.match_line.captures(&data) {
                let item = self
                    .match_nl
                    .replace_all(captures.get(1).unwrap().as_str(), "$1\n");
                index_items.push(item.into());

                data = self.match_line.replace(&data, "").into();
            } else {
                break;
            }
        }

        self.index.push((index_name, index_items));
    }

    pub fn index(&self) -> &Vec<(String, Vec<String>)> {
        &self.index
    }

    pub fn defs(&self) -> &Vec<(Regex, String)> {
        &self.defs
    }
}

block! {
    [pub StandingTexture(constructor, pack)]
    (name) :String;
    image: Option<BlockRef<ImageData>> = None;
}

impl StandingTexture {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn image(&self) -> Option<&BlockRef<ImageData>> {
        self.image.as_ref()
    }
}

block! {
    [pub Character(constructor, pack)]
    name: String = String::from("名前未設定");
    display_name: (String, String) = (String::from("名前未設定"), String::from("新規キャラクター"));
    chat_pallet: ChatPallet = ChatPallet::new();
    position: [f64; 3] = [0.0, 0.0, 0.0];
    size: [f64; 3] = [1.0, 1.5, 1.0];
    color: Pallet = Pallet::gray(5);
    textures: SelectList<StandingTexture> =
        SelectList::new(vec![StandingTexture::new(String::from("[default]"))], 0);
}

impl Character {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn display_name(&self) -> &(String, String) {
        &self.display_name
    }

    pub fn set_display_name(&mut self, display_name: (Option<String>, Option<String>)) {
        if let Some(main) = display_name.0 {
            self.display_name.0 = main;
        }
        if let Some(sub) = display_name.1 {
            self.display_name.1 = sub;
        }
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn size(&self) -> f64 {
        (self.size[0] + self.size[2]) / 2.0
    }

    pub fn set_size(&mut self, size: f64) {
        self.size[1] *= size / self.size();
        self.size[0] = size;
        self.size[2] = size;
    }

    pub fn tex_size(&self) -> f64 {
        (self.size[1] / self.size() * 1000.0).round() / 1000.0
    }

    pub fn set_tex_size(&mut self, tex_size: f64) {
        self.size[1] = self.size() * tex_size;
    }

    pub fn color(&self) -> &crate::libs::color::Pallet {
        &self.color
    }

    pub fn set_color(&mut self, color: Pallet) {
        self.color = color;
    }
    pub fn textures(&self) -> &SelectList<StandingTexture> {
        &self.textures
    }

    pub fn selected_texture(&self) -> Option<&StandingTexture> {
        self.textures.selected()
    }

    pub fn selected_texture_idx(&self) -> usize {
        self.textures.selected_idx()
    }

    pub fn set_selected_texture_idx(&mut self, tex_idx: usize) {
        self.textures.set_selected_idx(tex_idx);
    }

    pub fn set_texture_image(&mut self, tex_idx: usize, image: Option<BlockRef<ImageData>>) {
        if let Some(texture) = self.textures.get_mut(tex_idx) {
            texture.image = image;
        }
    }

    pub fn set_texture_name(&mut self, tex_idx: usize, name: String) {
        if let Some(texture) = self.textures.get_mut(tex_idx) {
            texture.name = name;
        }
    }

    pub fn push_texture(&mut self, texture: StandingTexture) {
        self.textures.push(texture);
    }
}
