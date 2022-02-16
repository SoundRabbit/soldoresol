#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::resource::ImageData;
use super::chat_message::Message;
use super::util::Pack;
use super::Property;
use super::{BlockMut, BlockRef};
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SECTION: Regex = Regex::new(r"\A//---\s*(.*)(\n|\z)").unwrap();
    static ref SUB_SECTION: Regex = Regex::new(r"\A//----+\s*(.*)(\n|\z)").unwrap();
    static ref DEFINITION: Regex = Regex::new(r"\A//(.+)=((.*\\\n)*(.*))(\n|\z)").unwrap();
    static ref LINE: Regex = Regex::new(r"\A(.*)(\n|\z)").unwrap();
    static ref NL: Regex = Regex::new(r"([^\\])(\\\\)*\\n").unwrap();
}

block! {
    [pub ChatPalletSubSection(constructor, pack)]
    (name): String;
    children: Vec<String> = vec![];
}

impl ChatPalletSubSection {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn children(&self) -> &Vec<String> {
        &self.children
    }
}

block! {
    [pub ChatPalletSection(constructor, pack)]
    (name): String;
    children: Vec<String> = vec![];
    sub_sections: Vec<ChatPalletSubSection> = vec![];
}

impl ChatPalletSection {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn children(&self) -> &Vec<String> {
        &self.children
    }

    pub fn sub_sections(&self) -> &Vec<ChatPalletSubSection> {
        &self.sub_sections
    }
}

block! {
    [pub ChatPallet(constructor, pack)]
    data: String = String::from("");
    defs: Vec<(Regex, String)> = vec![];
    children: Vec<String> = vec![];
    sub_sections: Vec<ChatPalletSubSection> = vec![];
    sections: Vec<ChatPalletSection> = vec![];
}

impl ChatPallet {
    pub fn set_data(&mut self, mut data: String) {
        self.data = data.clone();
        self.children.clear();
        self.sub_sections.clear();
        self.sections.clear();

        while !data.is_empty() {
            let tail_idx = self.sections.len() - 1;
            data = if let Some(captures) = SUB_SECTION.captures(&data) {
                let name = String::from(captures.get(1).unwrap().as_str());
                let sub_section = ChatPalletSubSection::new(name);

                if let Some(section) = self.sections.get_mut(tail_idx) {
                    section.sub_sections.push(sub_section);
                } else {
                    self.sub_sections.push(sub_section);
                }

                SUB_SECTION.replace(&data, "").to_string()
            } else if let Some(captures) = SECTION.captures(&data) {
                let name = String::from(captures.get(1).unwrap().as_str());
                let section = ChatPalletSection::new(name);

                self.sections.push(section);

                SECTION.replace(&data, "").to_string()
            } else if let Some(captures) = DEFINITION.captures(&data) {
                let regex = String::from(r"\A") + captures.get(1).unwrap().as_str() + r"\z";

                if let Ok(regex) = Regex::new(regex.as_str()) {
                    self.defs
                        .push((regex, String::from(captures.get(2).unwrap().as_str())));
                }

                DEFINITION.replace(&data, "").to_string()
            } else if let Some(captures) = LINE.captures(&data) {
                let item = NL.replace_all(captures.get(1).unwrap().as_str(), "$1\n");

                let sub_tail_idx = self.sub_sections.len() - 1;
                if let Some(section) = self.sections.get_mut(tail_idx) {
                    let sub_tail_idx = section.sub_sections.len() - 1;
                    if let Some(sub_section) = section.sub_sections.get_mut(sub_tail_idx) {
                        sub_section.children.push(item.into());
                    } else {
                        section.children.push(item.into());
                    }
                } else if let Some(sub_section) = self.sub_sections.get_mut(sub_tail_idx) {
                    sub_section.children.push(item.into());
                } else {
                    self.children.push(item.into());
                }

                LINE.replace(&data, "").to_string()
            } else {
                break;
            };
        }
    }

    pub fn children(&self) -> &Vec<String> {
        &self.children
    }

    pub fn sub_sections(&self) -> &Vec<ChatPalletSubSection> {
        &self.sub_sections
    }

    pub fn sections(&self) -> &Vec<ChatPalletSection> {
        &self.sections
    }

    pub fn defs(&self) -> &Vec<(Regex, String)> {
        &self.defs
    }

    pub fn data(&self) -> &String {
        &self.data
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
    [pub Description(constructor, pack)]
    raw: String = String::new();
    data: Message = Message::new("");
}

impl Description {
    pub fn raw(&self) -> &String {
        &self.raw
    }

    pub fn set_raw(&mut self, description: String) {
        self.data = Message::new(&description);
        self.raw = description;
    }

    pub fn data(&self) -> &Message {
        &self.data
    }
}

block! {
    [pub Character(constructor, pack)]
    (is_bind_to_grid): bool;
    name: String = String::from("名前未設定");
    display_name: (String, String) = (String::from("名前未設定"), String::from("新規キャラクター"));
    chatpallet: ChatPallet = {
        let mut chatpallet = ChatPallet::new();
        chatpallet.set_data(include_str!("./default_chatpallet.txt").to_string());
        chatpallet
    };
    position: [f64; 3] = [0.0, 0.0, 0.0];
    size: [f64; 3] = [1.0, 1.5, 1.0];
    color: Pallet = Pallet::gray(5);
    textures: SelectList<StandingTexture> =
        SelectList::new(vec![StandingTexture::new(String::from("[default]"))], 0);
    description: Description = Description::new();
    is_fixed_position: bool = false;
    properties: Vec<BlockMut<Property>> = vec![];
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

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description.set_raw(description);
    }

    pub fn chatpallet(&self) -> &ChatPallet {
        &self.chatpallet
    }

    pub fn set_chatpallet(&mut self, chatpallet: String) {
        self.chatpallet.set_data(chatpallet);
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

    pub fn properties(&self) -> &Vec<BlockMut<Property>> {
        &self.properties
    }

    pub fn push_property(&mut self, property: BlockMut<Property>) {
        self.properties.push(property);
    }
}

impl BlockMut<Character> {
    pub fn chat_ref<'a>(&'a self) -> impl FnMut(&String) -> Message + 'a {
        |ref_name: &String| {
            self.map(|this| {
                for (pat, text) in this.chatpallet().defs() {
                    if pat.is_match(ref_name) {
                        let message = Message::new(pat.replace(ref_name, text).as_ref());
                        return message;
                    }
                }
                Message::from(vec![])
            })
            .unwrap_or_else(|| Message::from(vec![]))
        }
    }
}
