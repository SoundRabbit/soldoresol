use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self};
use crate::libs::clone_of::CloneOf;
use crate::libs::random_id::U128Id;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod framebuffer;
mod libs;
mod mesh;

use libs::*;

use id_table::{IdTable, IdTableBuilder};
use tex_table::TexTable;
use webgl::WebGlRenderingContext;

pub use id_table::{IdColor, ObjectId, Surface};
pub use matrix::camera::CameraMatrix;

pub struct Renderer {
    canvas: Rc<web_sys::HtmlCanvasElement>,
    gl: WebGlRenderingContext,

    canvas_size: [f32; 2],
    device_pixel_ratio: f32,

    tex_table: TexTable,
    id_table: IdTable,

    view_frame: framebuffer::View,
    screen_frame: framebuffer::Screen,
    idmap_frame: framebuffer::Idmap,
    shadomap_frame: framebuffer::Shadowmap,

    screen_mesh: mesh::Screen,
    table_grid_mesh: mesh::TableGrid,
    boxblock_mesh: mesh::Boxblock,
}

impl Renderer {
    fn reset_canvas_size(canvas: &web_sys::HtmlCanvasElement, dpr: f32) -> [f32; 2] {
        let bb = canvas.get_bounding_client_rect();
        let w = bb.width() as f32 * dpr;
        let h = bb.height() as f32 * dpr;

        canvas.set_width(w as u32);
        canvas.set_height(h as u32);

        crate::debug::log_2(w, h);

        [w, h]
    }

    fn resize_renderbuffer(
        gl: &WebGlRenderingContext,
        buf: &web_sys::WebGlRenderbuffer,
        width: i32,
        height: i32,
    ) {
        gl.bind_renderbuffer(web_sys::WebGlRenderingContext::RENDERBUFFER, Some(&buf));
        gl.renderbuffer_storage(
            web_sys::WebGlRenderingContext::RENDERBUFFER,
            web_sys::WebGlRenderingContext::DEPTH_COMPONENT16,
            width,
            height,
        );
    }

    fn resize_texturebuffer(
        gl: &WebGlRenderingContext,
        buf: &web_sys::WebGlTexture,
        tex_id: &U128Id,
        tex_table: &mut tex_table::TexTable,
        width: i32,
        height: i32,
    ) {
        let (_, tex_flag) = tex_table.use_custom(tex_id);
        gl.active_texture(tex_flag);
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&buf));
        let _ = gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                width,
                height,
                0,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                None,
            );
    }

    fn create_screen_texture(
        gl: &WebGlRenderingContext,
        tex_table: &mut tex_table::TexTable,
        width: i32,
        height: i32,
        filter: Option<u32>,
    ) -> (web_sys::WebGlTexture, U128Id) {
        let tex_buf = gl.create_texture().unwrap();
        let tex_id = U128Id::new();
        let (_, tex_flag) = tex_table.use_custom(&tex_id);
        let filter = filter.unwrap_or(web_sys::WebGlRenderingContext::LINEAR);
        gl.active_texture(tex_flag);
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            filter as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
            filter as i32,
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
        Self::resize_texturebuffer(&gl, &tex_buf, &tex_id, tex_table, width, height);
        (tex_buf, tex_id)
    }

    pub fn new(canvas: Rc<web_sys::HtmlCanvasElement>) -> Self {
        let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio() as f32;
        let canvas_size = Self::reset_canvas_size(&canvas, device_pixel_ratio);

        let option: JsValue = object! {
            preserveDrawingBuffer: true,
            stenchil: true
        }
        .into();
        let gl = canvas
            .get_context_with_context_options("webgl", &option)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        gl.get_extension("EXT_frag_depth")
            .map_err(|err| crate::debug::log_1(&err))
            .unwrap()
            .unwrap();
        let mut gl = WebGlRenderingContext::new(gl);

        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        gl.cull_face(web_sys::WebGlRenderingContext::BACK);
        gl.enable(web_sys::WebGlRenderingContext::STENCIL_TEST);

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear_stencil(0);
        gl.stencil_func(web_sys::WebGlRenderingContext::ALWAYS, 0, 0);

        let mut tex_table = tex_table::TexTable::new(&gl);
        let id_table = id_table::IdTable::from(IdTableBuilder::new());

        let sw = canvas_size[0] as i32;
        let sh = canvas_size[1] as i32;

        let view_frame = framebuffer::View::new();
        let screen_frame = framebuffer::Screen::new(&gl, sw, sh, &mut tex_table);
        let idmap_frame = framebuffer::Idmap::new(&gl, sw, sh, &mut tex_table);
        let shadomap_frame = framebuffer::Shadowmap::new(&gl, &mut tex_table);

        let screen_mesh = mesh::Screen::new(&gl);
        let table_grid_mesh = mesh::TableGrid::new(&gl);
        let boxblock_mesh = mesh::Boxblock::new(&gl);

        Self {
            canvas,
            gl,

            canvas_size,
            device_pixel_ratio,

            tex_table,
            id_table,

            view_frame,
            screen_frame,
            idmap_frame,
            shadomap_frame,

            screen_mesh,
            table_grid_mesh,
            boxblock_mesh,
        }
    }

    pub fn reset_size(&mut self) {
        let canvas_size = Self::reset_canvas_size(&self.canvas, self.device_pixel_ratio);
        let sw = canvas_size[0] as i32;
        let sh = canvas_size[1] as i32;

        self.gl.viewport(0, 0, sw, sh);
        self.screen_frame
            .reset_size(&self.gl, sw, sh, &mut self.tex_table);
        self.idmap_frame
            .reset_size(&self.gl, sw, sh, &mut self.tex_table);
        self.canvas_size = canvas_size;
    }

    pub fn get_object_id(&self, x: f32, y: f32) -> ObjectId {
        self.idmap_frame.bind_self(&self.gl);
        let x = x * self.device_pixel_ratio;
        let y = self.canvas_size[1] - y * self.device_pixel_ratio;
        let mut table_id = [0, 0, 0, 0];
        let res = self.gl.read_pixels_with_opt_u8_array(
            x as i32,
            y as i32,
            1,
            1,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            Some(&mut table_id),
        );

        if res.is_ok() {
            let table_id = u32::from_be_bytes([table_id[3], table_id[0], table_id[1], table_id[2]]);
            self.id_table
                .object_id(&IdColor::from(table_id))
                .map(|x| ObjectId::clone_of(x))
                .unwrap_or(ObjectId::None)
        } else {
            ObjectId::None
        }
    }

    pub fn get_focused_position(
        &self,
        _block_arena: &block::Arena,
        camera: &CameraMatrix,
        x: f32,
        y: f32,
    ) -> ([f32; 3], [f32; 3]) {
        let focused = self.get_object_id(x, y);

        let x = x * self.device_pixel_ratio;
        let y = y * self.device_pixel_ratio;

        match focused {
            ObjectId::Character(_, srfs) => (
                camera.collision_point(&self.canvas_size, &[x, y], &srfs.r, &srfs.s, &srfs.t),
                srfs.n(),
            ),
            ObjectId::Boxblock(_, srfs) => (
                camera.collision_point(&self.canvas_size, &[x, y], &srfs.r, &srfs.s, &srfs.t),
                srfs.n(),
            ),
            ObjectId::Pointlight(_, srfs) => (
                camera.collision_point(&self.canvas_size, &[x, y], &srfs.r, &srfs.s, &srfs.t),
                srfs.n(),
            ),
            ObjectId::Terran(_, srfs) => (
                camera.collision_point(&self.canvas_size, &[x, y], &srfs.r, &srfs.s, &srfs.t),
                srfs.n(),
            ),
            _ => {
                let p = camera.collision_point_on_xy_plane(&self.canvas_size, &[x, y]);
                ([p[0], p[1], p[2]], [0.0, 0.0, 1.0])
            }
        }
    }

    pub fn render(
        &mut self,
        block_arena: &block::Arena,
        local_block_arena: &block::Arena,
        resource_arena: &resource::Arena,
        world_id: &BlockId,
        camera_matrix: &CameraMatrix,
        grabbed_object_id: &ObjectId,
    ) {
        block_arena.map(world_id, |world: &block::world::World| {
            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                let mut id_table_builder = id_table::IdTableBuilder::new();
                self.boxblock_mesh
                    .update_id(&mut id_table_builder, block_arena, table.boxblocks());

                self.id_table = IdTable::from(id_table_builder);

                let vp_matrix = camera_matrix.vp_matrix(&self.canvas_size);

                self.view_frame.bind_self(&self.gl);
                self.clear();
                // self.screen_frame
                //     .begin_to_render_frontscreen(&self.gl, &self.tex_table);
                // self.clear();

                // self.screen_frame
                //     .begin_to_render_backscreen(&self.gl, &self.tex_table);
                // self.clear();

                self.gl.blend_func_separate(
                    web_sys::WebGlRenderingContext::SRC_ALPHA,
                    web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
                    web_sys::WebGlRenderingContext::ONE,
                    web_sys::WebGlRenderingContext::ONE,
                );

                self.table_grid_mesh.render(
                    &mut self.gl,
                    &vp_matrix,
                    &camera_matrix.position(),
                    table,
                );

                self.boxblock_mesh.render(
                    &mut self.gl,
                    &self.id_table,
                    &vp_matrix,
                    &camera_matrix.position(),
                    &block_arena,
                    table.boxblocks(),
                    &mesh::boxblock::RenderingMode::View {
                        lighting: mesh::boxblock::LightingMode::AmbientLight {
                            direction: &[1.0, 1.0, 1.0],
                        },
                        light_color: &crate::libs::color::Pallet::gray(5),
                        light_intensity: 1.0,
                    },
                    &mut self.tex_table,
                );

                // self.render_frontscreen(false);

                // 当たり判定用のオフスクリーンレンダリング
                // self.idmap_frame.bind_self(&self.gl);
                // self.clear();

                // self.gl.blend_func(
                //     web_sys::WebGlRenderingContext::ONE,
                //     web_sys::WebGlRenderingContext::ZERO,
                // );

                // self.view_frame.bind_self(&self.gl);
                // self.clear();
                // self.flip();

                self.gl.finish();
            });
        });
    }

    fn clear(&self) {
        self.gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT
                | web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT,
        );
    }

    fn begin_to_render_backscreen(&self) {
        self.gl.framebuffer_texture_2d(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.screen_frame.backscreen_tex().0),
            0,
        );
    }

    fn render_frontscreen(&mut self, add_blend: bool) {
        self.screen_frame
            .begin_to_render_frontscreen(&self.gl, &self.tex_table);

        self.screen_mesh.render(
            &mut self.gl,
            &self.screen_frame.backscreen_tex().1,
            &mut self.tex_table,
            &self.screen_frame.backscreen_tex().0,
            &self.canvas_size,
        );
    }

    fn flip(&mut self) {
        self.screen_mesh.render(
            &mut self.gl,
            &self.screen_frame.frontscreen_tex().1,
            &mut self.tex_table,
            &self.screen_frame.frontscreen_tex().0,
            &self.canvas_size,
        );
    }
}
