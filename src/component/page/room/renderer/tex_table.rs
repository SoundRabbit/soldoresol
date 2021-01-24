use super::webgl::WebGlRenderingContext;
use crate::arena::resource::{self, ResourceId};
use crate::libs::random_id::U128Id;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Hash, PartialEq, Eq)]
enum TextureId {
    ResourceId(ResourceId),
    Custom(U128Id),
}

impl TextureId {
    fn clone(this: &Self) -> Self {
        match this {
            Self::ResourceId(id) => Self::ResourceId(ResourceId::clone(id)),
            Self::Custom(id) => Self::Custom(U128Id::clone(id)),
        }
    }
}

pub struct TexTable {
    max_tex_num: i32,
    unused_tex_idx: VecDeque<i32>,
    used_tex_idx: VecDeque<(i32, TextureId)>,
    tex_table: HashMap<ResourceId, Rc<web_sys::WebGlTexture>>,
    tex_idx: HashMap<TextureId, i32>,
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
        for i in 1..max_tex_num {
            unused_tex_idx.push_back(i);
        }

        Self {
            max_tex_num,
            unused_tex_idx,
            used_tex_idx: VecDeque::new(),
            tex_table: HashMap::new(),
            tex_idx: HashMap::new(),
        }
    }

    pub fn use_resource(
        &mut self,
        gl: WebGlRenderingContext,
        resource_arena: &resource::ArenaRef,
        resource_id: &ResourceId,
    ) -> Option<i32> {
        let tex_id = TextureId::ResourceId(ResourceId::clone(resource_id));
        if let Some(tex_idx) = self.tex_idx.get(&tex_id) {
            Some(*tex_idx)
        } else {
            if let Some(tex_buf) = self
                .tex_table
                .get(&resource_id)
                .map(|tex_buf| Rc::clone(&tex_buf))
            {
                let tex_idx = self.use_idx();
                gl.active_texture(Self::tex_flag(tex_idx));
                gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
                self.used_tex_idx.push_back((tex_idx, tex_id));
                Some(tex_idx)
            } else {
                let data = resource_arena.get_as::<resource::ImageData>(resource_id);
                if let Some(data) = data {
                    let tex_idx = self.use_idx();
                    let tex_buf = gl.create_texture().unwrap();
                    gl.active_texture(Self::tex_flag(tex_idx));
                    gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
                    gl.pixel_storei(web_sys::WebGlRenderingContext::PACK_ALIGNMENT, 1);
                    gl.tex_parameteri(
                        web_sys::WebGlRenderingContext::TEXTURE_2D,
                        web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
                        web_sys::WebGlRenderingContext::NEAREST as i32,
                    );
                    gl.tex_parameteri(
                        web_sys::WebGlRenderingContext::TEXTURE_2D,
                        web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
                        web_sys::WebGlRenderingContext::NEAREST as i32,
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
                    self.used_tex_idx.push_back((tex_idx, tex_id));
                    Some(tex_idx)
                } else {
                    return None;
                }
            }
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