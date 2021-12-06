use super::webgl::WebGlRenderingContext;
use crate::arena::{resource, BlockRef};
use crate::libs::random_id::U128Id;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Hash, PartialEq, Eq, Clone)]
enum TextureId {
    ResourceId(U128Id),
    Custom(U128Id),
    Nameplate(String, String),
}

struct Lifespan<V> {
    value: V,
    life_expectancy: usize,
    is_used: Cell<bool>,
}

impl<V> Lifespan<V> {
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

impl<V> std::ops::Deref for Lifespan<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        self.is_used.set(true);
        &self.value
    }
}

impl<V> std::ops::DerefMut for Lifespan<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_used.set(true);
        &mut self.value
    }
}

pub struct TexTable {
    max_tex_num: i32,
    unused_tex_idx: VecDeque<i32>,
    used_tex_idx: VecDeque<(i32, TextureId)>,
    nameplate_tex_table: HashMap<(String, String), Lifespan<(Rc<web_sys::WebGlTexture>, [f64; 2])>>,
    resource_tex_table: HashMap<U128Id, Rc<web_sys::WebGlTexture>>,
    tex_idx: HashMap<TextureId, i32>,
    string_canvas: web_sys::HtmlCanvasElement,
}

impl TexTable {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let max_tex_num = (gl
            .get_parameter(web_sys::WebGlRenderingContext::MAX_TEXTURE_IMAGE_UNITS)
            .unwrap()
            .as_f64()
            .unwrap() as i32)
            .min(32);
        let mut unused_tex_idx = VecDeque::new();
        for i in 0..max_tex_num {
            unused_tex_idx.push_back(i);
        }

        let string_canvas = crate::libs::element::html_canvas_element();

        Self {
            max_tex_num,
            unused_tex_idx,
            used_tex_idx: VecDeque::new(),
            nameplate_tex_table: HashMap::new(),
            resource_tex_table: HashMap::new(),
            tex_idx: HashMap::new(),
            string_canvas,
        }
    }

    pub fn update(&mut self, gl: &WebGlRenderingContext) {
        let mut deleted = vec![];
        for (key_text, tex) in &mut self.nameplate_tex_table {
            if tex.aging() {
                deleted.push(key_text.clone());
            }
        }

        for key_text in &deleted {
            if let Some(tex) = self.nameplate_tex_table.remove(key_text) {
                gl.delete_texture(Some(&tex.0));
            }
        }
    }

    pub fn use_blocktexture(
        &mut self,
        gl: &WebGlRenderingContext,
        texture: BlockRef<resource::BlockTexture>,
    ) -> Option<i32> {
        let resource_id = texture.id();
        texture.map(|texture| self.use_resource(gl, &resource_id, texture.data()))
    }

    pub fn use_resource(
        &mut self,
        gl: &WebGlRenderingContext,
        resource_id: &U128Id,
        data: &resource::ImageData,
    ) -> i32 {
        let tex_id = TextureId::ResourceId(U128Id::clone(resource_id));
        if let Some(tex_idx) = self.tex_idx.get(&tex_id) {
            *tex_idx
        } else if let Some(tex_buf) = self
            .resource_tex_table
            .get(&resource_id)
            .map(|tex_buf| Rc::clone(&tex_buf))
        {
            let tex_idx = self.use_idx();
            gl.active_texture(Self::tex_flag(tex_idx));
            gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
            self.tex_idx.insert(TextureId::clone(&tex_id), tex_idx);
            self.used_tex_idx.push_back((tex_idx, tex_id));
            tex_idx
        } else {
            let tex_idx = self.use_idx();
            let tex_buf = gl.create_texture().unwrap();
            gl.active_texture(Self::tex_flag(tex_idx));
            gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
            gl.pixel_storei(web_sys::WebGlRenderingContext::PACK_ALIGNMENT, 1);
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
                web_sys::WebGlRenderingContext::LINEAR as i32,
            );
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
                web_sys::WebGlRenderingContext::LINEAR as i32,
            );
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
                web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
                web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            let _ = gl.tex_image_2d_with_u32_and_u32_and_image(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                &data.element(),
            );
            self.resource_tex_table
                .insert(U128Id::clone(&resource_id), Rc::new(tex_buf));
            self.tex_idx.insert(TextureId::clone(&tex_id), tex_idx);
            self.used_tex_idx.push_back((tex_idx, tex_id));
            tex_idx
        }
    }

    pub fn use_nameplate(
        &mut self,
        gl: &WebGlRenderingContext,
        text: &(String, String),
    ) -> Option<(i32, [f64; 2])> {
        let tex_id = TextureId::Nameplate(text.0.clone(), text.1.clone());
        if let Some((tex_buf, size)) = self
            .nameplate_tex_table
            .get(text)
            .map(|tex| (Rc::clone(&tex.0), tex.1.clone()))
        {
            if let Some(tex_idx) = self.tex_idx.get(&tex_id) {
                Some((*tex_idx, size))
            } else {
                let tex_idx = self.use_idx();
                gl.active_texture(Self::tex_flag(tex_idx));
                gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
                self.tex_idx.insert(TextureId::clone(&tex_id), tex_idx);
                self.used_tex_idx.push_back((tex_idx, tex_id));
                Some((tex_idx, size))
            }
        } else {
            let canvas = &self.string_canvas;
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            let font_height = 64.0;
            ctx.set_font(&format!("{}px sans-serif", font_height));

            let metrix = ctx.measure_text(&text.0).unwrap();
            let r = font_height / 4.0;
            let mut height = font_height + 2.0 * r;
            let mut width = metrix.width() + 2.0 * r;
            let mut sub_font_height = 0.0;
            let mut sub_line_margin = 0.0;

            if !text.1.is_empty() {
                sub_font_height = font_height * 5.0 / 8.0;
                sub_line_margin = sub_font_height / 2.0;
                ctx.set_font(&format!("{}px sans-serif", sub_font_height));

                let metrix = ctx.measure_text(&text.1).unwrap();
                height += sub_font_height + sub_line_margin;
                width = width.max(metrix.width() + 2.0 * r);
            }

            let height = height;
            let width = width;
            let sub_font_height = sub_font_height;
            let sub_line_margin = sub_line_margin;

            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            ctx.clear_rect(0.0, 0.0, width, height);

            ctx.set_fill_style(&JsValue::from("#000000"));
            ctx.set_stroke_style(&JsValue::from("#FFFFFF"));
            ctx.set_line_width(r * 0.5);
            let x = 0.0;
            let y = 0.0;
            ctx.begin_path();
            ctx.move_to(x + r, y);
            let _ = ctx.arc_to(x + width, y, x + width, y + height, r);
            let _ = ctx.arc_to(x + width, y + height, x, y + height, r);
            let _ = ctx.arc_to(x, y + height, x, y, r);
            let _ = ctx.arc_to(x, y, x + width, y, r);
            ctx.close_path();
            ctx.fill();
            ctx.stroke();

            ctx.set_font(&format!("{}px sans-serif", font_height));
            ctx.set_fill_style(&JsValue::from("#FFFFFF"));
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(
                &text.0,
                r,
                sub_font_height
                    + sub_line_margin
                    + r
                    + (height - r * 2.0 - sub_font_height - sub_line_margin) / 2.0,
            );

            if !text.1.is_empty() {
                ctx.set_font(&format!("{}px sans-serif", sub_font_height));
                let _ = ctx.fill_text(&text.1, r, r + sub_font_height / 2.0);
            }

            let tex_idx = self.use_idx();
            let tex_buf = gl.create_texture().unwrap();
            gl.active_texture(Self::tex_flag(tex_idx));
            gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
            gl.pixel_storei(web_sys::WebGlRenderingContext::PACK_ALIGNMENT, 1);
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
                web_sys::WebGlRenderingContext::LINEAR as i32,
            );
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
                web_sys::WebGlRenderingContext::LINEAR as i32,
            );
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
                web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
                web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            let _ = gl.tex_image_2d_with_u32_and_u32_and_canvas(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                &self.string_canvas,
            );
            self.nameplate_tex_table.insert(
                text.clone(),
                Lifespan::new((Rc::new(tex_buf), [width, height])),
            );
            self.tex_idx.insert(TextureId::clone(&tex_id), tex_idx);
            self.used_tex_idx.push_back((tex_idx, tex_id));

            Some((tex_idx, [width, height]))
        }
    }

    pub fn use_custom(&mut self, id: &U128Id) -> (i32, u32) {
        let tex_id = TextureId::Custom(U128Id::clone(id));
        if let Some(tex_idx) = self.tex_idx.get(&tex_id) {
            (*tex_idx, Self::tex_flag(*tex_idx))
        } else {
            let tex_idx = self.use_idx();
            self.used_tex_idx
                .push_back((tex_idx, TextureId::clone(&tex_id)));
            self.tex_idx.insert(tex_id, tex_idx);
            (tex_idx, Self::tex_flag(tex_idx))
        }
    }

    pub fn try_use_custom(&self, id: &U128Id) -> Option<(i32, u32)> {
        let tex_id = TextureId::Custom(U128Id::clone(id));
        if let Some(tex_idx) = self.tex_idx.get(&tex_id) {
            Some((*tex_idx, Self::tex_flag(*tex_idx)))
        } else {
            None
        }
    }

    fn use_idx(&mut self) -> i32 {
        if let Some(tex_idx) = self.unused_tex_idx.pop_front() {
            tex_idx
        } else {
            let (tex_idx, tex_id) = self.used_tex_idx.pop_front().unwrap();
            self.tex_idx.remove(&tex_id);
            tex_idx
        }
    }

    fn tex_flag(idx: i32) -> u32 {
        match idx {
            0 => web_sys::WebGlRenderingContext::TEXTURE0,
            1 => web_sys::WebGlRenderingContext::TEXTURE1,
            2 => web_sys::WebGlRenderingContext::TEXTURE2,
            3 => web_sys::WebGlRenderingContext::TEXTURE3,
            4 => web_sys::WebGlRenderingContext::TEXTURE4,
            5 => web_sys::WebGlRenderingContext::TEXTURE5,
            6 => web_sys::WebGlRenderingContext::TEXTURE6,
            7 => web_sys::WebGlRenderingContext::TEXTURE7,
            8 => web_sys::WebGlRenderingContext::TEXTURE8,
            9 => web_sys::WebGlRenderingContext::TEXTURE9,
            10 => web_sys::WebGlRenderingContext::TEXTURE10,
            11 => web_sys::WebGlRenderingContext::TEXTURE11,
            12 => web_sys::WebGlRenderingContext::TEXTURE12,
            13 => web_sys::WebGlRenderingContext::TEXTURE13,
            14 => web_sys::WebGlRenderingContext::TEXTURE14,
            15 => web_sys::WebGlRenderingContext::TEXTURE15,
            16 => web_sys::WebGlRenderingContext::TEXTURE16,
            17 => web_sys::WebGlRenderingContext::TEXTURE17,
            18 => web_sys::WebGlRenderingContext::TEXTURE18,
            19 => web_sys::WebGlRenderingContext::TEXTURE19,
            20 => web_sys::WebGlRenderingContext::TEXTURE20,
            21 => web_sys::WebGlRenderingContext::TEXTURE21,
            22 => web_sys::WebGlRenderingContext::TEXTURE22,
            23 => web_sys::WebGlRenderingContext::TEXTURE23,
            24 => web_sys::WebGlRenderingContext::TEXTURE24,
            25 => web_sys::WebGlRenderingContext::TEXTURE25,
            26 => web_sys::WebGlRenderingContext::TEXTURE26,
            27 => web_sys::WebGlRenderingContext::TEXTURE27,
            28 => web_sys::WebGlRenderingContext::TEXTURE28,
            29 => web_sys::WebGlRenderingContext::TEXTURE29,
            30 => web_sys::WebGlRenderingContext::TEXTURE30,
            31 => web_sys::WebGlRenderingContext::TEXTURE31,
            _ => unreachable!(),
        }
    }
}
