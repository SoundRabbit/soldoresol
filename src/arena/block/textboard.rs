#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Pack;
use crate::libs::color::Pallet;

block! {
    [pub Textboard(constructor, pack)]
    (position): [f64; 3];
    title: String = String::new();
    text: String = String::new();
    font_size: f64 = 0.5;
    size: [f64; 2] = [3.0, 4.0];
    color: Pallet = Pallet::yellow(0);
}

impl Textboard {
    pub fn color(&self) -> &Pallet {
        &self.color
    }

    pub fn set_color(&mut self, color: Pallet) {
        self.color = color;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    pub fn set_font_size(&mut self, font_size: f64) {
        self.font_size = font_size;
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }
}
