mod character_collection_renderer;
mod table_grid_renderer;
mod table_texture_renderer;
mod tablemask_collection_renderer;

use super::webgl::WebGlRenderingContext;
use crate::model::{Camera, Resource, World};
use character_collection_renderer::CharacterCollectionRenderer;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use table_grid_renderer::TableGridRenderer;
use table_texture_renderer::TableTextureRenderer;
use tablemask_collection_renderer::TablemaskCollectionRenderer;

pub struct TextureCollection(HashMap<u128, web_sys::WebGlTexture>);

impl Deref for TextureCollection {
    type Target = HashMap<u128, web_sys::WebGlTexture>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct ViewRenderer {
    character_collection_renderer: CharacterCollectionRenderer,
    table_texture_renderer: TableTextureRenderer,
    table_grid_renderer: TableGridRenderer,
    tablemask_collection_renderer: TablemaskCollectionRenderer,
    img_texture_buffer: TextureCollection,
}

impl ViewRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func_separate(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ONE,
        );
        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);

        let character_collection_renderer = CharacterCollectionRenderer::new(gl);
        let table_texture_renderer = TableTextureRenderer::new(gl);
        let table_grid_renderer = TableGridRenderer::new(gl);
        let tablemask_collection_renderer = TablemaskCollectionRenderer::new(gl);

        Self {
            character_collection_renderer,
            table_texture_renderer,
            table_grid_renderer,
            tablemask_collection_renderer,
            img_texture_buffer: TextureCollection(HashMap::new()),
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        canvas_size: &[f64; 2],
        camera: &Camera,
        world: &mut World,
        resource: &Resource,
    ) {
        let vp_matrix = camera
            .view_matrix()
            .dot(&camera.perspective_matrix(&canvas_size));
        gl.viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        if let Some(table) = world.selecting_table_mut() {
            self.table_texture_renderer.render(
                gl,
                camera,
                &vp_matrix,
                table,
                &mut self.img_texture_buffer,
                resource,
            );
        }
        self.tablemask_collection_renderer
            .render(gl, camera, &vp_matrix, world.tablemasks());
        if let Some(table) = world.selecting_table_mut() {
            self.table_grid_renderer
                .render(gl, camera, &vp_matrix, table);
            table.rendered();
        }
        self.character_collection_renderer.render(
            gl,
            camera,
            &vp_matrix,
            world.characters_mut(),
            &mut self.img_texture_buffer,
            resource,
        );
    }
}

impl TextureCollection {
    fn insert(
        &mut self,
        gl: &WebGlRenderingContext,
        texture_id: u128,
        texture_data: &web_sys::HtmlImageElement,
    ) {
        let texture_buffer = gl.create_texture().unwrap();
        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&texture_buffer),
        );
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
        gl.tex_image_2d_with_u32_and_u32_and_image(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            0,
            web_sys::WebGlRenderingContext::RGBA as i32,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            texture_data,
        )
        .unwrap();
        self.0.insert(texture_id, texture_buffer);
    }
}
