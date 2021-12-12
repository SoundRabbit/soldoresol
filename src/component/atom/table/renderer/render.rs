use super::*;
use crate::arena::{block, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;

impl Renderer {
    pub fn render(
        &mut self,
        is_debug_mode: bool,
        world: BlockRef<block::World>,
        camera_matrix: &CameraMatrix,
        grabbed_object_id: &U128Id,
    ) {
        let (scene, table) = world
            .map(|world| {
                let scene = world.selecting_scene().as_ref();
                let table = scene
                    .map(|scene: &block::Scene| scene.selecting_table().as_ref())
                    .unwrap_or(BlockRef::<block::Table>::none());

                (scene, table)
            })
            .unwrap_or((
                BlockRef::<block::Scene>::none(),
                BlockRef::<block::Table>::none(),
            ));

        let mut id_table_builder = id_table::IdTableBuilder::new();

        id_table_builder.insert(
            &crate::libs::random_id::U128Id::none(),
            IdColor::from(0),
            ObjectId::None,
        );

        table.map(|table| {
            self.craftboard_cover_mesh.update_id(
                &mut id_table_builder,
                table
                    .craftboards()
                    .iter()
                    .map(BlockMut::<block::Craftboard>::as_ref),
            );
        });

        table.map(|table| {
            self.boxblock_mesh.update_id(
                &mut id_table_builder,
                table
                    .boxblocks()
                    .iter()
                    .map(BlockMut::<block::Boxblock>::as_ref),
            );
        });

        world.map(|world| {
            self.character_base_mesh.update_id(
                &mut id_table_builder,
                world
                    .characters()
                    .iter()
                    .map(BlockMut::<block::Character>::as_ref),
            );
        });

        world.map(|world| {
            self.character_mesh.update_id(
                &mut id_table_builder,
                camera_matrix,
                world
                    .characters()
                    .iter()
                    .map(BlockMut::<block::Character>::as_ref),
            );
        });

        self.id_table = IdTable::from(id_table_builder);

        let cs = as_f32a![self.canvas_size[0], self.canvas_size[1]];
        let vp_matrix = camera_matrix.vp_matrix(&cs);
        let camera_position = camera_matrix.relative_position();

        if !is_debug_mode {
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
                self.craftboard_grid_mesh.render(
                    &mut self.gl,
                    &vp_matrix,
                    &camera_position,
                    table
                        .craftboards()
                        .iter()
                        .map(BlockMut::<block::Craftboard>::as_ref),
                );
            });

            table.map(|table: &block::Table| {
                self.boxblock_mesh.render(
                    &mut self.gl,
                    &self.id_table,
                    &vp_matrix,
                    &camera_position,
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
                    camera_matrix.is_2d_mode(),
                    &mut self.tex_table,
                );
            });

            world.map(|world| {
                self.character_base_mesh.render(
                    &mut self.gl,
                    &self.id_table,
                    &vp_matrix,
                    &camera_position,
                    world
                        .characters()
                        .iter()
                        .map(BlockMut::<block::Character>::as_ref),
                    &mesh::character_base::RenderingMode::View,
                    camera_matrix.is_2d_mode(),
                );
            });

            world.map(|world| {
                table.map(|table: &block::Table| {
                    self.nameplate_mesh.render(
                        &mut self.gl,
                        &vp_matrix,
                        &camera_position,
                        &camera_matrix,
                        table.boxblocks(),
                        world.characters(),
                        camera_matrix.is_2d_mode(),
                        &mut self.tex_table,
                    );
                });
            });

            world.map(|world| {
                self.character_mesh.render(
                    &mut self.gl,
                    &self.id_table,
                    &vp_matrix,
                    &camera_position,
                    camera_matrix,
                    world
                        .characters()
                        .iter()
                        .map(BlockMut::<block::Character>::as_ref),
                    &mesh::character::RenderingMode::View,
                    camera_matrix.is_2d_mode(),
                    &mut self.tex_table,
                );
            });

            self.render_frontscreen(&cs);
        }

        // 当たり判定用のオフスクリーンレンダリング
        self.idmap_frame.bind_self(&self.gl);
        self.idmap_frame.begin_to_render(&self.gl, &self.tex_table);
        self.clear();

        self.gl.blend_func_separate(
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ZERO,
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ZERO,
        );

        table.map(|table: &block::Table| {
            self.craftboard_cover_mesh.render(
                &mut self.gl,
                &self.id_table,
                &vp_matrix,
                &camera_position,
                table
                    .craftboards()
                    .iter()
                    .map(BlockMut::<block::Craftboard>::as_ref),
                &grabbed_object_id,
                camera_matrix.is_2d_mode(),
            );
        });

        table.map(|table: &block::Table| {
            self.boxblock_mesh.render(
                &mut self.gl,
                &self.id_table,
                &vp_matrix,
                &camera_position,
                table
                    .boxblocks()
                    .iter()
                    .map(BlockMut::<block::Boxblock>::as_ref),
                &mesh::boxblock::RenderingMode::IdMap {
                    grabbed: grabbed_object_id,
                },
                camera_matrix.is_2d_mode(),
                &mut self.tex_table,
            );
        });

        world.map(|world| {
            self.character_base_mesh.render(
                &mut self.gl,
                &self.id_table,
                &vp_matrix,
                &camera_position,
                world
                    .characters()
                    .iter()
                    .map(BlockMut::<block::Character>::as_ref),
                &mesh::character_base::RenderingMode::IdMap {
                    grabbed: grabbed_object_id,
                },
                camera_matrix.is_2d_mode(),
            );
        });

        world.map(|world| {
            self.character_mesh.render(
                &mut self.gl,
                &self.id_table,
                &vp_matrix,
                &camera_position,
                camera_matrix,
                world
                    .characters()
                    .iter()
                    .map(BlockMut::<block::Character>::as_ref),
                &mesh::character::RenderingMode::IdMap {
                    grabbed: grabbed_object_id,
                },
                camera_matrix.is_2d_mode(),
                &mut self.tex_table,
            );
        });

        self.view_frame.bind_self(&self.gl);
        self.clear();
        self.flip(&cs, is_debug_mode);

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

    fn flip(&mut self, cs: &[f32; 2], is_debug_mode: bool) {
        if !is_debug_mode {
            self.screen_mesh.render(
                &mut self.gl,
                &self.screen_frame.frontscreen_tex().1,
                &mut self.tex_table,
                &self.screen_frame.frontscreen_tex().0,
                cs,
            );
        } else {
            self.screen_mesh.render(
                &mut self.gl,
                &self.idmap_frame.screen_tex().1,
                &mut self.tex_table,
                &self.idmap_frame.screen_tex().0,
                cs,
            );
        }
    }
}
