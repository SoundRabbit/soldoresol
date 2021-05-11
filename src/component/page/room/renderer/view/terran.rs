use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::tex_table::TexTable;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use crate::libs::color::Pallet;
use crate::libs::random_id::U128Id;
use ndarray::{arr1, Array2};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

pub struct Terran {
    vertexes_buffer: WebGlF32Vbo,
    normals_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    colors_buffer: WebGlF32Vbo,
    vertex_num: i32,
    last_terran_id: BlockId,
    terran_update_time: f64,
}

impl Terran {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexes_buffer = gl.create_vbo_with_f32array(&[]);
        let normals_buffer = gl.create_vbo_with_f32array(&[]);
        let index_buffer = gl.create_ibo_with_i16array(&[]);
        let colors_buffer = gl.create_vbo_with_f32array(&[]);

        Self {
            vertexes_buffer,
            index_buffer,
            normals_buffer,
            colors_buffer,
            vertex_num: 0,
            last_terran_id: BlockId::none(),
            terran_update_time: 0.0,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        table: &block::table::Table,
        light: &[f32; 3],
        light_color: &crate::libs::color::Pallet,
        light_intensity: f32,
        mut tex_table: Option<&mut TexTable>,
        shadowmap: Option<&[(web_sys::WebGlTexture, U128Id); 6]>,
        light_vps: Option<&[Array2<f32>; 6]>,
        light_attenation: Option<f32>,
    ) {
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::TerranProgram);

        let terran_id = table.terran_id();
        if block_arena.timestamp_of(terran_id).unwrap_or(0.0) > self.terran_update_time
            || *terran_id != self.last_terran_id
        {
            block_arena.map(terran_id, |terran: &block::terran::Terran| {
                self.update_buffer(gl, terran);
            });
        }

        gl.set_attr_vertex(&self.vertexes_buffer, 3, 0);
        gl.set_attr_normal(&self.normals_buffer, 3, 0);
        gl.set_attr_color(&self.colors_buffer, 3, 0);

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        if let (Some(light_vps), Some(light_attenation)) = (light_vps, light_attenation) {
            gl.set_unif_light_vp_px(light_vps[0].clone().reversed_axes());
            gl.set_unif_light_vp_py(light_vps[1].clone().reversed_axes());
            gl.set_unif_light_vp_pz(light_vps[2].clone().reversed_axes());
            gl.set_unif_light_vp_nx(light_vps[3].clone().reversed_axes());
            gl.set_unif_light_vp_ny(light_vps[4].clone().reversed_axes());
            gl.set_unif_light_vp_nz(light_vps[5].clone().reversed_axes());
            gl.set_unif_is_shadowmap(1);
            gl.set_unif_attenation(light_attenation);
            gl.set_unif_shade_intensity(0.5);
        } else {
            gl.set_unif_shade_intensity(0.5);
            gl.set_unif_attenation(1.0);
            gl.set_unif_is_shadowmap(0);
        }

        gl.set_unif_light(light);
        gl.set_unif_light_color(&light_color.to_color().to_f32array());
        gl.set_unif_light_intensity(light_intensity);

        if let (Some(tex_table), Some(shadowmap)) = (tex_table.as_mut(), shadowmap.as_ref()) {
            for i in 0..6 {
                let (tex_idx, tex_flag) = tex_table.use_custom(&shadowmap[i].1);
                gl.active_texture(tex_flag);
                gl.bind_texture(
                    web_sys::WebGlRenderingContext::TEXTURE_2D,
                    Some(&shadowmap[i].0),
                );
                match i {
                    0 => {
                        gl.set_unif_shadowmap_px(tex_idx);
                    }
                    1 => {
                        gl.set_unif_shadowmap_py(tex_idx);
                    }
                    2 => {
                        gl.set_unif_shadowmap_pz(tex_idx);
                    }
                    3 => {
                        gl.set_unif_shadowmap_nx(tex_idx);
                    }
                    4 => {
                        gl.set_unif_shadowmap_ny(tex_idx);
                    }
                    5 => {
                        gl.set_unif_shadowmap_nz(tex_idx);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        gl.set_unif_vp(vp_matrix.clone().reversed_axes());

        let model_matrix: Array2<f32> = ModelMatrix::new().with_movement(&[-0.5, -0.5, 0.0]).into();
        let inv_model_matrix: Array2<f32> = ModelMatrix::new().into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);

        gl.set_unif_model(model_matrix.reversed_axes());
        gl.set_unif_inv_model(inv_model_matrix.reversed_axes());
        gl.set_unif_translate(mvp_matrix.reversed_axes());

        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            self.vertex_num,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn update_buffer(&mut self, gl: &mut WebGlRenderingContext, terran: &block::terran::Terran) {
        let mut vertexes = vec![];
        let mut normals = vec![];
        let mut indexes = vec![];
        let mut colors = vec![];
        let mut vertexes_table: HashMap<([i32; 3], usize, Pallet), i16> = HashMap::new();

        for (p, terran_block) in terran.table().iter() {
            let o = [
                [[1, 1, 1], [1, 0, 1], [1, 1, 0], [1, 0, 0]],
                [[1, 1, 1], [1, 1, 0], [0, 1, 1], [0, 1, 0]],
                [[1, 1, 1], [0, 1, 1], [1, 0, 1], [0, 0, 1]],
                [[0, 1, 1], [0, 1, 0], [0, 0, 1], [0, 0, 0]],
                [[1, 0, 1], [0, 0, 1], [1, 0, 0], [0, 0, 0]],
                [[1, 1, 0], [1, 0, 0], [0, 1, 0], [0, 0, 0]],
            ];
            for i in 0..6 {
                if !terran.is_covered(p, i) {
                    Self::push_surface(
                        &mut indexes,
                        &mut vertexes,
                        &mut normals,
                        &mut colors,
                        &mut vertexes_table,
                        [p[0] + o[i][0][0], p[1] + o[i][0][1], p[2] + o[i][0][2]],
                        [p[0] + o[i][1][0], p[1] + o[i][1][1], p[2] + o[i][1][2]],
                        [p[0] + o[i][2][0], p[1] + o[i][2][1], p[2] + o[i][2][2]],
                        [p[0] + o[i][3][0], p[1] + o[i][3][1], p[2] + o[i][3][2]],
                        i,
                        terran_block.color(),
                    );
                }
            }
        }

        self.vertexes_buffer = gl.create_vbo_with_f32array(&vertexes);
        self.normals_buffer = gl.create_vbo_with_f32array(&normals);
        self.colors_buffer = gl.create_vbo_with_f32array(&colors);
        self.index_buffer = gl.create_ibo_with_i16array(&indexes);
        self.vertex_num = indexes.len() as i32;
    }

    fn push_surface(
        indexes: &mut Vec<i16>,
        vertexes: &mut Vec<f32>,
        normals: &mut Vec<f32>,
        colors: &mut Vec<f32>,
        vertexes_table: &mut HashMap<([i32; 3], usize, Pallet), i16>,
        pp: [i32; 3],
        np: [i32; 3],
        pn: [i32; 3],
        nn: [i32; 3],
        n_idx: usize,
        color: &Pallet,
    ) {
        let pp_idx = Self::push_vertex(vertexes, normals, colors, vertexes_table, pp, n_idx, color);
        let np_idx = Self::push_vertex(vertexes, normals, colors, vertexes_table, np, n_idx, color);
        let pn_idx = Self::push_vertex(vertexes, normals, colors, vertexes_table, pn, n_idx, color);
        let nn_idx = Self::push_vertex(vertexes, normals, colors, vertexes_table, nn, n_idx, color);

        indexes.push(pp_idx);
        indexes.push(np_idx);
        indexes.push(pn_idx);
        indexes.push(nn_idx);
        indexes.push(pn_idx);
        indexes.push(np_idx);
    }

    fn push_vertex(
        vertexes: &mut Vec<f32>,
        normals: &mut Vec<f32>,
        colors: &mut Vec<f32>,
        vertexes_table: &mut HashMap<([i32; 3], usize, Pallet), i16>,
        pos: [i32; 3],
        n_idx: usize,
        color: &Pallet,
    ) -> i16 {
        let key = (pos, n_idx, color.clone());
        if let Some(idx) = vertexes_table.get(&key) {
            *idx
        } else {
            let n = Self::n(n_idx);
            vertexes.push(pos[0] as f32);
            vertexes.push(pos[1] as f32);
            vertexes.push(pos[2] as f32);
            normals.push(n[0]);
            normals.push(n[1]);
            normals.push(n[2]);

            let color = color.to_color().to_f32array();
            colors.push(color[0]);
            colors.push(color[1]);
            colors.push(color[2]);

            let idx = vertexes_table.len() as i16;
            vertexes_table.insert(key, idx);
            idx
        }
    }

    fn n(n_idx: usize) -> [f32; 3] {
        match n_idx % 6 {
            0 => [1.0, 0.0, 0.0],  // PX
            1 => [0.0, 1.0, 0.0],  // PY
            2 => [0.0, 0.0, 1.0],  // PZ
            3 => [-1.0, 0.0, 0.0], // NX
            4 => [0.0, -1.0, 0.0], // NY
            5 => [0.0, 0.0, -1.0], // NZ
            _ => unreachable!(),
        }
    }
}
