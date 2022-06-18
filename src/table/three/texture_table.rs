use crate::arena::{resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct TextureTable {
    data: HashMap<U128Id, Texture>,
    text: HashMap<(String, String), Fate<Rc<TextTexture>>>,
    canvas: web_sys::HtmlCanvasElement,
}

pub enum Texture {
    Image(Rc<three::Texture>),
    Block(Rc<three::Texture>),
}

pub struct TextTexture {
    pub data: three::Texture,
    pub size: [f64; 2],
}

struct Fate<V> {
    value: V,
    life_expectancy: usize,
    is_used: Cell<bool>,
}

impl TextureTable {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            text: HashMap::new(),
            canvas: crate::libs::element::html_canvas_element(),
        }
    }

    pub fn update(&mut self) {
        let mut deleted = vec![];
        for (key_text, tex) in &mut self.text {
            if tex.aging() {
                deleted.push(key_text.clone());
            }
        }

        for key_text in &deleted {
            if let Some(tex) = self.text.remove(key_text) {
                tex.data.dispose();
            }
        }
    }

    pub fn load_image(
        &mut self,
        image: BlockRef<resource::ImageData>,
    ) -> Option<Rc<three::Texture>> {
        let image_id = image.id();
        if let Some(Texture::Image(texture)) = self.data.get(&image_id) {
            return Some(Rc::clone(texture));
        }

        let texture = image.map(|image| {
            let texture = Rc::new(three::Texture::new_with_image(image.element()));
            texture.set_needs_update(true);
            texture
        });
        texture.map(|texture| {
            self.data
                .insert(image_id, Texture::Image(Rc::clone(&texture)));
            texture
        })
    }

    pub fn load_block(
        &mut self,
        block_texture: BlockRef<resource::BlockTexture>,
    ) -> Option<Rc<three::Texture>> {
        let texture_id = block_texture.id();
        if let Some(Texture::Image(texture)) = self.data.get(&texture_id) {
            return Some(Rc::clone(texture));
        }

        let texture = block_texture.map(|block_texture| {
            let texture = Rc::new(three::Texture::new_with_image(
                block_texture.data().element(),
            ));
            texture.set_wrap_s(three::REPEAT_WRAPPING);
            texture.set_needs_update(true);
            texture
        });
        texture.map(|texture| {
            self.data
                .insert(texture_id, Texture::Block(Rc::clone(&texture)));
            texture
        })
    }

    pub fn load_text(&mut self, text: &(String, String)) -> Rc<TextTexture> {
        if let Some(texture) = self.text.get(text) {
            return Rc::clone(texture);
        }

        let text_1 = text.1 != "";
        let canvas = &self.canvas;
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let font_size = (64.0, 32.0);
        let text_size = if text_1 {
            (
                Self::calc_text_size(&ctx, font_size.0, &text.0),
                Self::calc_text_size(&ctx, font_size.1, &text.1),
            )
        } else {
            (Self::calc_text_size(&ctx, font_size.0, &text.0), [0.0, 0.0])
        };
        let canvas_size = [
            f64::max(text_size.0[0], text_size.1[0]),
            text_size.0[1] + text_size.1[1],
        ];

        canvas.set_width(canvas_size[0] as u32);
        canvas.set_height(canvas_size[1] as u32);
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        ctx.set_fill_style(&JsValue::from("#000000"));
        ctx.fill_rect(0.0, 0.0, canvas_size[0], canvas_size[1]);
        ctx.set_text_baseline("top");
        ctx.set_fill_style(&JsValue::from("#FFFFFF"));

        ctx.set_font(&format!("{}px sans-serif", font_size.0));
        let _ = ctx.fill_text(&text.0, 0.0, text_size.1[1]);

        if text_1 {
            ctx.set_font(&format!("{}px sans-serif", font_size.1));
            let _ = ctx.fill_text(&text.1, 0.0, 0.0);
        }

        let texture = three::Texture::new_with_canvas(&canvas);
        texture.set_needs_update(true);

        let texture = TextTexture {
            data: texture,
            size: [canvas_size[0] / font_size.0, canvas_size[1] / font_size.0],
        };
        let texture = Rc::new(texture);
        self.text
            .insert(text.clone(), Fate::new(Rc::clone(&texture)));

        texture
    }

    fn calc_text_size(
        ctx: &web_sys::CanvasRenderingContext2d,
        font_size: f64,
        text: &String,
    ) -> [f64; 2] {
        ctx.set_font(&format!("{}px sans-serif", font_size));
        let lines = text.lines().count();
        let metrix = ctx.measure_text(&text).unwrap();
        let width = metrix.width();
        let height = font_size * lines as f64;

        [width, height]
    }
}

impl<V> Fate<V> {
    pub fn new(value: V) -> Self {
        Self {
            value,
            life_expectancy: 0,
            is_used: Cell::new(true),
        }
    }

    pub fn aging(&mut self) -> bool {
        if self.is_used.get() {
            self.life_expectancy += 1;
            self.is_used.set(false);
            false
        } else if self.life_expectancy == 0 {
            true
        } else {
            self.life_expectancy -= 1;
            false
        }
    }
}

impl<V> std::ops::Deref for Fate<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        self.is_used.set(true);
        &self.value
    }
}

impl<V> std::ops::DerefMut for Fate<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_used.set(true);
        &mut self.value
    }
}
