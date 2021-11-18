use super::*;
use crate::arena::{block, BlockMut, BlockRef};

impl Renderer {
    pub fn render(
        &mut self,
        world: BlockRef<block::World>,
        camera_matrix: &CameraMatrix,
        grabbed_object_id: &ObjectId,
    ) {
        let (scene, table) = world
            .map(|world| {
                crate::debug::log_1("load world");

                let scene = world.selecting_scene().as_ref();
                let table = scene
                    .map(|scene: &block::Scene| {
                        crate::debug::log_1("load scene");
                        scene.selecting_table().as_ref()
                    })
                    .unwrap_or(BlockRef::<block::Table>::none());

                (scene, table)
            })
            .unwrap_or((
                BlockRef::<block::Scene>::none(),
                BlockRef::<block::Table>::none(),
            ));

        let mut id_table_builder = id_table::IdTableBuilder::new();

        table.map(|table| {
            self.boxblock_mesh.update_id(
                &mut id_table_builder,
                table
                    .boxblocks()
                    .iter()
                    .map(BlockMut::<block::Boxblock>::as_ref),
            );
        });

        let cs = as_f32a![self.canvas_size[0], self.canvas_size[1]];
        let vp_matrix = camera_matrix.vp_matrix(&cs);

        self.screen_frame.bind_self(&self.gl);

        self.screen_frame
            .begin_to_render_frontscreen(&self.gl, &self.tex_table);
        self.clear();

        self.screen_frame
            .begin_to_render_backscreen(&self.gl, &self.tex_table);
        self.clear();

        self.gl.blend_func_separate(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ONE,
        );

        table.map(|table: &block::Table| {
            let craftboards = table.craftboards();
            for craftboard in craftboards {
                craftboard.map(|craftboard| {
                    crate::debug::log_1("render craftboard");
                    self.craftboard_grid_mesh.render(
                        &mut self.gl,
                        &vp_matrix,
                        &camera_matrix.position(),
                        craftboard,
                    );
                });
            }
        });

        table.map(|table: &block::Table| {
            self.boxblock_mesh.render(
                &mut self.gl,
                &self.id_table,
                &vp_matrix,
                &camera_matrix.position(),
                table
                    .boxblocks()
                    .iter()
                    .map(BlockMut::<block::Boxblock>::as_ref),
                &mesh::boxblock::RenderingMode::View {
                    lighting: mesh::boxblock::LightingMode::AmbientLight {
                        direction: &[1.0, -2.0, 3.0],
                    },
                    light_color: &crate::libs::color::Pallet::gray(0),
                    light_intensity: 1.0,
                },
                &mut self.tex_table,
            );
        });

        self.render_frontscreen(&cs);

        // 当たり判定用のオフスクリーンレンダリング
        self.idmap_frame.bind_self(&self.gl);
        self.clear();

        self.gl.blend_func(
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ZERO,
        );

        table.map(|table: &block::Table| {
            self.boxblock_mesh.render(
                &mut self.gl,
                &self.id_table,
                &vp_matrix,
                &camera_matrix.position(),
                table
                    .boxblocks()
                    .iter()
                    .map(BlockMut::<block::Boxblock>::as_ref),
                &mesh::boxblock::RenderingMode::IdMap {
                    grabbed: grabbed_object_id,
                },
                &mut self.tex_table,
            );
        });

        self.view_frame.bind_self(&self.gl);
        self.clear();
        self.flip(&cs);

        self.tex_table.update(&self.gl);
    }

    fn clear(&self) {
        self.gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT
                | web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT,
        );
    }

    fn render_frontscreen(&mut self, cs: &[f32; 2]) {
        self.screen_frame
            .begin_to_render_frontscreen(&self.gl, &self.tex_table);

        self.screen_mesh.render(
            &mut self.gl,
            &self.screen_frame.backscreen_tex().1,
            &mut self.tex_table,
            &self.screen_frame.backscreen_tex().0,
            cs,
        );
    }

    fn flip(&mut self, cs: &[f32; 2]) {
        self.screen_mesh.render(
            &mut self.gl,
            &self.screen_frame.frontscreen_tex().1,
            &mut self.tex_table,
            &self.screen_frame.frontscreen_tex().0,
            cs,
        );
    }
}
