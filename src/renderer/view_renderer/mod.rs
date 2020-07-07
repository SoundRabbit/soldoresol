mod area_collection_renderer;
mod boxblock_collection_renderer;
mod character_mask_renderer;
mod character_texture_renderer;
mod measure_renderer;
mod table_grid_renderer;
mod table_texture_renderer;
mod tablemask_collection_renderer;

use super::{webgl::WebGlRenderingContext, Camera};
use crate::{
    block::{self},
    resource::ResourceId,
    Resource,
};
use area_collection_renderer::AreaCollectionRenderer;
use boxblock_collection_renderer::BoxblockCollectionRenderer;
use character_mask_renderer::CharacterMaskRenderer;
use character_texture_renderer::CharacterTextureRenderer;
use measure_renderer::MeasureRenderer;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use table_grid_renderer::TableGridRenderer;
use table_texture_renderer::TableTextureRenderer;
use tablemask_collection_renderer::TablemaskCollectionRenderer;

pub struct TextureCollection(HashMap<ResourceId, web_sys::WebGlTexture>);

impl Deref for TextureCollection {
    type Target = HashMap<ResourceId, web_sys::WebGlTexture>;
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
    area_collection_renderer: AreaCollectionRenderer,
    boxblock_collection_renderer: BoxblockCollectionRenderer,
    character_mask_renderer: CharacterMaskRenderer,
    character_texture_renderer: CharacterTextureRenderer,
    table_texture_renderer: TableTextureRenderer,
    table_grid_renderer: TableGridRenderer,
    tablemask_collection_renderer: TablemaskCollectionRenderer,
    measure_renderer: MeasureRenderer,
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
        gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        gl.cull_face(web_sys::WebGlRenderingContext::BACK);
        gl.enable(web_sys::WebGlRenderingContext::STENCIL_TEST);

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear_stencil(0);

        let area_collection_renderer = AreaCollectionRenderer::new(gl);
        let boxblock_collection_renderer = BoxblockCollectionRenderer::new(gl);
        let character_mask_renderer = CharacterMaskRenderer::new(gl);
        let character_texture_renderer = CharacterTextureRenderer::new(gl);
        let table_texture_renderer = TableTextureRenderer::new(gl);
        let table_grid_renderer = TableGridRenderer::new(gl);
        let tablemask_collection_renderer = TablemaskCollectionRenderer::new(gl);
        let measure_renderer = MeasureRenderer::new(gl);

        Self {
            area_collection_renderer,
            boxblock_collection_renderer,
            character_mask_renderer,
            character_texture_renderer,
            table_texture_renderer,
            table_grid_renderer,
            tablemask_collection_renderer,
            measure_renderer,
            img_texture_buffer: TextureCollection(HashMap::new()),
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        canvas_size: &[f32; 2],
        camera: &Camera,
        block_field: &block::Field,
        world: &block::World,
        resource: &Resource,
    ) {
        let vp_matrix = camera
            .perspective_matrix(&canvas_size)
            .dot(&camera.view_matrix());
        gl.viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT
                | web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT,
        );
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        if let Some(table) = block_field.get::<block::Table>(world.selecting_table()) {
            self.table_texture_renderer.render(
                gl,
                &vp_matrix,
                block_field,
                table,
                &mut self.img_texture_buffer,
                resource,
            );
            self.tablemask_collection_renderer.render(
                gl,
                &vp_matrix,
                block_field,
                table,
                table.tablemasks(),
            );
            self.area_collection_renderer
                .render(gl, &vp_matrix, block_field, table.areas());

            self.table_grid_renderer.render(gl, &vp_matrix, table);

            gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
            self.boxblock_collection_renderer.render(
                gl,
                &vp_matrix,
                block_field,
                table.boxblocks(),
            );
        }

        self.character_mask_renderer.render(
            gl,
            camera,
            &vp_matrix,
            block_field,
            world.characters(),
        );

        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);

        self.measure_renderer.render(gl, &vp_matrix, block_field);

        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);

        self.character_texture_renderer.render(
            gl,
            camera,
            &vp_matrix,
            block_field,
            world.characters(),
            &mut self.img_texture_buffer,
            resource,
        );
    }
}

impl TextureCollection {
    fn insert(
        &mut self,
        gl: &WebGlRenderingContext,
        texture_id: ResourceId,
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
