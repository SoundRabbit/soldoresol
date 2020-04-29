use super::Color;
use super::ColorSystem;
use super::TexstureLayer;
use super::TexstureLayerCollection;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const STANDING_PICTURE_LAYER: usize = 0;
const FRAME_LAYER: usize = 1;

pub struct Character {
    size: [f64; 2],
    position: [f64; 3],
    layers: TexstureLayerCollection,
}

impl Character {
    pub fn new() -> Self {
        let mut layers = TexstureLayerCollection::new(&[256, 256], &[256, 256], 2);
        layers.set_background_color(ColorSystem::gray_200(255));
        Self {
            size: [1.0, 3.0],
            position: [0.0, 0.0, 0.0],
            layers,
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn texture_element(&mut self) -> &web_sys::HtmlCanvasElement {
        self.layers.to_element()
    }

    pub fn set_is_focused(&self, is_focused: bool) {
        let canvas = self.layers.element(FRAME_LAYER);
        let context = self.layers.context(FRAME_LAYER);
        context.set_line_width(30.0);
        context.set_stroke_style(&ColorSystem::gray_900(255).to_jsvalue());
        context.begin_path();
        if is_focused {
            context.stroke_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        } else {
            context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        }
        context.fill();
        context.stroke();
    }
}
