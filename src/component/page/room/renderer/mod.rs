use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self};
use crate::libs::clone_of::CloneOf;
use crate::libs::random_id::U128Id;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod id_table;
mod matrix;
mod offscreen;
mod tex_table;
mod view;
mod webgl;

use webgl::WebGlRenderingContext;

pub use id_table::{ObjectId, Surface};
pub use matrix::camera::CameraMatrix;

pub struct Renderer {
    view_canvas: Rc<web_sys::HtmlCanvasElement>,
    view_gl: WebGlRenderingContext,
    offscreen_canvas: Rc<web_sys::HtmlCanvasElement>,
    offscreen_gl: WebGlRenderingContext,

    canvas_size: [f32; 2],
    device_pixel_ratio: f32,

    tex_table: tex_table::TexTable,
    id_table: id_table::IdTable,

    render_offscreen_character: offscreen::character::Character,
    render_offscreen_boxblock: offscreen::boxblock::Boxblock,

    render_view_tablegrid: view::tablegrid::Tablegrid,
    render_view_tabletexture: view::tabletexture::Tabletexture,
    render_view_character: view::character::Character,
    render_view_character_base: view::character_base::CharacterBase,
    render_view_boxblock: view::boxblock::Boxblock,
    render_screen: view::screen::Screen,

    depth_screen: web_sys::WebGlRenderbuffer,
    tex_screen: (web_sys::WebGlTexture, U128Id),
    shadow_map: [(web_sys::WebGlTexture, U128Id); 6],
    frame_screen: web_sys::WebGlFramebuffer,

    light_camera: [CameraMatrix; 6],
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
    ) -> (web_sys::WebGlTexture, U128Id) {
        let tex_buf = gl.create_texture().unwrap();
        let tex_id = U128Id::new();
        let (_, tex_flag) = tex_table.use_custom(&tex_id);
        gl.active_texture(tex_flag);
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
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
        Self::resize_texturebuffer(&gl, &tex_buf, &tex_id, tex_table, width, height);
        (tex_buf, tex_id)
    }

    pub fn new(view_canvas: Rc<web_sys::HtmlCanvasElement>) -> Self {
        let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio() as f32;
        let canvas_size = Self::reset_canvas_size(&view_canvas, device_pixel_ratio);

        let option: JsValue = object! {stenchil: true}.into();
        let view_gl = view_canvas
            .get_context_with_context_options("webgl", &option)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        view_gl
            .get_extension("EXT_frag_depth")
            .map_err(|err| crate::debug::log_1(&err))
            .unwrap()
            .unwrap();
        let mut view_gl = WebGlRenderingContext::new(view_gl);

        view_gl.enable(web_sys::WebGlRenderingContext::BLEND);
        view_gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        view_gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        view_gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        view_gl.cull_face(web_sys::WebGlRenderingContext::BACK);
        view_gl.enable(web_sys::WebGlRenderingContext::STENCIL_TEST);

        view_gl.clear_color(0.0, 0.0, 0.0, 0.0);
        view_gl.clear_stencil(0);

        let offscreen_canvas = Rc::new(crate::libs::element::html_canvas_element());
        offscreen_canvas.set_width(canvas_size[0] as u32);
        offscreen_canvas.set_height(canvas_size[1] as u32);
        let option: JsValue = object! {
            preserveDrawingBuffer: true,
            stencil: true
        }
        .into();
        let offscreen_gl = offscreen_canvas
            .get_context_with_context_options("webgl", &option)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        let mut offscreen_gl = WebGlRenderingContext::new(offscreen_gl);

        offscreen_gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        offscreen_gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        offscreen_gl.enable(web_sys::WebGlRenderingContext::BLEND);
        offscreen_gl.blend_func(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        offscreen_gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        offscreen_gl.cull_face(web_sys::WebGlRenderingContext::BACK);
        offscreen_gl.use_program(webgl::ProgramType::OffscreenProgram);

        offscreen_gl.clear_color(0.0, 0.0, 0.0, 0.0);
        offscreen_gl.clear_stencil(0);

        let mut tex_table = tex_table::TexTable::new(&view_gl);
        let id_table = id_table::IdTable::new();

        let render_view_tablegrid = view::tablegrid::Tablegrid::new(&view_gl);
        let render_view_tabletexture =
            view::tabletexture::Tabletexture::new(&view_gl, &mut tex_table);
        let render_view_character = view::character::Character::new(&view_gl);
        let render_view_character_base = view::character_base::CharacterBase::new(&view_gl);
        let render_view_boxblock = view::boxblock::Boxblock::new(&view_gl);
        let render_screen = view::screen::Screen::new(&view_gl);

        let render_offscreen_character = offscreen::character::Character::new(&offscreen_gl);
        let render_offscreen_boxblock = offscreen::boxblock::Boxblock::new(&offscreen_gl);

        let depth_screen = view_gl.create_renderbuffer().unwrap();
        Self::resize_renderbuffer(
            &view_gl,
            &depth_screen,
            canvas_size[0] as i32,
            canvas_size[1] as i32,
        );

        let tex_screen = Self::create_screen_texture(
            &view_gl,
            &mut tex_table,
            canvas_size[0] as i32,
            canvas_size[1] as i32,
        );

        let frame_screen = view_gl.create_framebuffer().unwrap();
        view_gl.bind_framebuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            Some(&frame_screen),
        );
        view_gl.framebuffer_renderbuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::DEPTH_ATTACHMENT,
            web_sys::WebGlRenderingContext::RENDERBUFFER,
            Some(&depth_screen),
        );
        view_gl.framebuffer_texture_2d(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&tex_screen.0),
            0,
        );

        let shadow_map = [
            Self::create_screen_texture(&view_gl, &mut tex_table, 256, 256),
            Self::create_screen_texture(&view_gl, &mut tex_table, 256, 256),
            Self::create_screen_texture(&view_gl, &mut tex_table, 256, 256),
            Self::create_screen_texture(&view_gl, &mut tex_table, 256, 256),
            Self::create_screen_texture(&view_gl, &mut tex_table, 256, 256),
            Self::create_screen_texture(&view_gl, &mut tex_table, 256, 256),
        ];

        let mut light_camera = [
            CameraMatrix::new(),
            CameraMatrix::new(),
            CameraMatrix::new(),
            CameraMatrix::new(),
            CameraMatrix::new(),
            CameraMatrix::new(),
        ];

        Self {
            view_canvas,
            view_gl,
            offscreen_canvas,
            offscreen_gl,
            canvas_size,
            device_pixel_ratio,
            tex_table,
            id_table,
            render_offscreen_character,
            render_offscreen_boxblock,
            render_view_tablegrid,
            render_view_tabletexture,
            render_view_character,
            render_view_character_base,
            render_view_boxblock,
            render_screen,
            depth_screen,
            tex_screen,
            shadow_map,
            frame_screen,
            light_camera,
        }
    }

    pub fn reset_size(&mut self) {
        let canvas_size = Self::reset_canvas_size(&self.view_canvas, self.device_pixel_ratio);

        self.offscreen_canvas.set_width(canvas_size[0] as u32);
        self.offscreen_canvas.set_height(canvas_size[1] as u32);

        self.view_gl
            .viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        self.offscreen_gl
            .viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);

        Self::resize_renderbuffer(
            &self.view_gl,
            &self.depth_screen,
            canvas_size[0] as i32,
            canvas_size[1] as i32,
        );
        Self::resize_texturebuffer(
            &self.view_gl,
            &self.tex_screen.0,
            &self.tex_screen.1,
            &mut self.tex_table,
            canvas_size[0] as i32,
            canvas_size[1] as i32,
        );

        self.canvas_size = canvas_size;
    }

    pub fn get_object_id(&self, x: f32, y: f32) -> ObjectId {
        let gl = &self.offscreen_gl;
        let x = x * self.device_pixel_ratio;
        let y = self.canvas_size[1] - y * self.device_pixel_ratio;
        let mut table_id = [0, 0, 0, 0];
        let res = gl.read_pixels_with_opt_u8_array(
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
                .get(&table_id)
                .map(|x| ObjectId::clone_of(x))
                .unwrap_or(ObjectId::None)
        } else {
            ObjectId::None
        }
    }

    pub fn get_focused_position(
        &self,
        block_arena: &block::Arena,
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
            let vp_matrix = camera_matrix
                .perspective_matrix(&self.canvas_size)
                .dot(&camera_matrix.view_matrix(true));

            self.view_gl.bind_framebuffer(
                web_sys::WebGlRenderingContext::FRAMEBUFFER,
                Some(&self.frame_screen),
            );
            self.view_gl.clear(
                web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT,
            );
            self.view_gl.blend_func_separate(
                web_sys::WebGlRenderingContext::SRC_ALPHA,
                web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
                web_sys::WebGlRenderingContext::ONE,
                web_sys::WebGlRenderingContext::ONE,
            );

            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                self.render_view_tabletexture.render(
                    &mut self.view_gl,
                    &mut self.tex_table,
                    &vp_matrix,
                    block_arena,
                    local_block_arena,
                    resource_arena,
                    table,
                );
            });

            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                self.render_view_tablegrid
                    .render(&mut self.view_gl, &vp_matrix, table)
            });

            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                self.render_view_boxblock.render(
                    &mut self.view_gl,
                    &vp_matrix,
                    block_arena,
                    table.boxblocks().map(|x| BlockId::clone(x)),
                )
            });

            self.render_view_character_base.render(
                &mut self.view_gl,
                &vp_matrix,
                block_arena,
                world.characters().map(|x| BlockId::clone(x)),
            );

            self.render_view_character.render(
                &mut self.view_gl,
                &mut self.tex_table,
                camera_matrix,
                &vp_matrix,
                block_arena,
                resource_arena,
                world.characters().map(|x| BlockId::clone(x)),
            );

            self.view_gl
                .bind_framebuffer(web_sys::WebGlRenderingContext::FRAMEBUFFER, None);

            self.view_gl.clear(
                web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT,
            );

            self.view_gl.blend_func_separate(
                web_sys::WebGlRenderingContext::SRC_ALPHA,
                web_sys::WebGlRenderingContext::DST_ALPHA,
                web_sys::WebGlRenderingContext::ONE,
                web_sys::WebGlRenderingContext::ONE,
            );

            self.render_screen.render(
                &mut self.view_gl,
                &self.tex_screen.1,
                &mut self.tex_table,
                &self.tex_screen.0,
                &self.canvas_size,
            );

            self.offscreen_gl.clear(
                web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT,
            );
            self.id_table.clear();

            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                self.render_offscreen_boxblock.render(
                    &mut self.offscreen_gl,
                    &mut self.id_table,
                    &vp_matrix,
                    block_arena,
                    table.boxblocks().map(|x| BlockId::clone(x)),
                    grabbed_object_id,
                );
            });

            self.render_offscreen_character.render(
                &mut self.offscreen_gl,
                &mut self.id_table,
                camera_matrix,
                &vp_matrix,
                block_arena,
                resource_arena,
                world.characters().map(|x| BlockId::clone(x)),
                grabbed_object_id,
            );
        });
    }
}
