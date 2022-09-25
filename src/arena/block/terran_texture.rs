#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::resource::{BlockTexture, ImageData, LoadFrom};
use super::util::{Pack, PackDepth};
use super::BlockRef;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub const CELL_WIDTH: u32 = 256;
pub const CELL_HEIGHT: u32 = 256;
pub const CELL_MARGIN: u32 = 16;
pub const TEX_WIDTH: u32 = 4096;
pub const TEX_HEIGHT: u32 = 4096;
pub const COL_NUM: u32 = TEX_WIDTH / CELL_WIDTH;
pub const ROW_NUM: u32 = TEX_HEIGHT / CELL_HEIGHT;
pub const TEX_NUM: usize = (COL_NUM * ROW_NUM) as usize;

block! {
    [pub TerranTexture()]
    (canvas): Rc<web_sys::HtmlCanvasElement>;
    (textures): [BlockRef<BlockTexture>; TEX_NUM];
}

impl TerranTexture {
    pub fn new() -> Self {
        let canvas = crate::libs::element::html_canvas_element();

        let canvas = Rc::new(canvas);
        canvas.set_width(TEX_WIDTH);
        canvas.set_height(TEX_HEIGHT);

        Self {
            canvas,
            textures: array_macro::array![_ => BlockRef::none(); TEX_NUM],
        }
    }

    pub fn data(&self) -> &web_sys::HtmlCanvasElement {
        &self.canvas
    }

    pub fn set_textures<'a>(
        &mut self,
        textures: impl Iterator<Item = (u32, BlockRef<BlockTexture>)>,
    ) {
        let context = self.context();
        for (tex_idx, tex) in textures {
            if tex_idx < TEX_NUM as u32 {
                let [col, row] = Self::texture_position(tex_idx);
                let x = (col * CELL_WIDTH) as f64;
                let y = (row * CELL_HEIGHT) as f64;
                let w = CELL_WIDTH as f64;
                let h = CELL_HEIGHT as f64;
                context.clear_rect(x, y, w, h);
                tex.map(|tex| {
                    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(
                        tex.data().element(),
                        x,
                        y,
                        w - CELL_MARGIN as f64,
                        h - CELL_MARGIN as f64,
                    );
                });
                self.textures[tex_idx as usize] = tex;
            }
        }
    }

    pub fn set_texture(&mut self, tex_idx: u32, texture: BlockRef<BlockTexture>) {
        self.set_textures(vec![(tex_idx, texture)].into_iter());
    }

    pub fn textures(&self) -> &[BlockRef<BlockTexture>; TEX_NUM] {
        &self.textures
    }

    pub fn uv(tex_idx: u32, uv: &[f64; 2]) -> [f64; 2] {
        let [c, r] = Self::texture_position(tex_idx);
        let w = CELL_WIDTH as f64 / TEX_WIDTH as f64;
        let h = CELL_HEIGHT as f64 / TEX_HEIGHT as f64;
        let w_m = CELL_MARGIN as f64 / TEX_WIDTH as f64;
        let h_m = CELL_MARGIN as f64 / TEX_HEIGHT as f64;
        let left = (c * CELL_WIDTH) as f64 / TEX_WIDTH as f64;
        let top = 1.0 - ((r * CELL_HEIGHT) as f64 / TEX_HEIGHT as f64 + h);

        [uv[0] * (w - w_m) + left, uv[1] * (h - h_m) + top + h_m]
    }

    pub fn uv_f32(tex_idx: u32, uv: &[f32; 2]) -> [f32; 2] {
        let res = Self::uv(tex_idx, &[uv[0] as f64, uv[1] as f64]);
        [res[0] as f32, res[1] as f32]
    }

    fn context(&self) -> web_sys::CanvasRenderingContext2d {
        self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }

    fn texture_position(idx: u32) -> [u32; 2] {
        let col = idx % COL_NUM;
        let row = idx / COL_NUM;
        [col, row]
    }
}

#[async_trait(?Send)]
impl Pack for TerranTexture {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let textures = array![];
        for texture in &self.textures {
            let texture = texture
                .pack(match pack_depth {
                    PackDepth::Recursive => PackDepth::Recursive,
                    _ => PackDepth::FirstBlock,
                })
                .await;
            textures.push(&texture);
        }

        (object! {
            "textures": textures
        })
        .into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = unwrap!(data.dyn_ref::<crate::libs::js_object::Object>(); None);
        let packed_textures = unwrap!(data.get("textures"); None);
        let packed_textures = js_sys::Array::from(&packed_textures).to_vec();
        let mut textures = vec![];

        for packed_texture in packed_textures {
            if let Some(texture) =
                BlockRef::<BlockTexture>::unpack(&packed_texture, ArenaMut::clone(&arena)).await
            {
                texture
                    .map(|_| {
                        crate::debug::log_1("push texture");
                    })
                    .unwrap_or_else(|| {
                        crate::debug::log_1("push none");
                    });
                textures.push(*texture);
            } else {
                textures.push(BlockRef::none());
            }
        }

        let mut this = Self::new();
        this.set_textures(
            textures
                .into_iter()
                .enumerate()
                .map(|(idx, tex)| (idx as u32, tex)),
        );

        Some(Box::new(this))
    }
}
