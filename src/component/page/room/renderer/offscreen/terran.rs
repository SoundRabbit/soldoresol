use super::id_table::{IdTable, ObjectId, Surface};
use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;
use std::collections::HashMap;

pub struct Terran {
    vertexes_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    colors_buffer: WebGlF32Vbo,
    colors: Vec<usize>,
    normals: HashMap<usize, (i32, usize)>,
    vertex_num: i32,
    last_terran_id: BlockId,
    terran_update_time: f64,
}

impl Terran {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexes_buffer = gl.create_vbo_with_f32array(&[]);
        let index_buffer = gl.create_ibo_with_i16array(&[]);
        let colors_buffer = gl.create_vbo_with_f32array(&[]);

        Self {
            vertexes_buffer,
            index_buffer,
            colors_buffer,
            colors: vec![],
            normals: HashMap::new(),
            vertex_num: 0,
            last_terran_id: BlockId::none(),
            terran_update_time: 0.0,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        id_table: &mut IdTable,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        table: &block::table::Table,
        terran_id: &BlockId,
    ) {
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::OffscreenProgram);

        if block_arena.timestamp_of(terran_id).unwrap_or(0.0) > self.terran_update_time
            || *terran_id != self.last_terran_id
        {
            block_arena.map(terran_id, |terran: &block::terran::Terran| {
                self.update_buffer(gl, terran);
            });
        }

        self.set_color_buffer(gl, id_table, terran_id, table.terran_height());

        gl.set_attr_vertex(&self.vertexes_buffer, 3, 0);

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        let offset = [
            -table.size()[0].floor() % 2.0 * 0.5,
            -table.size()[1].floor() % 2.0 * 0.5,
            0.0,
        ];
        let model_matrix: Array2<f32> = ModelMatrix::new()
            .with_movement(&offset)
            .with_scale(&[1.0, 1.0, table.terran_height()])
            .into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);
        gl.set_unif_translate(mvp_matrix.reversed_axes());
        gl.set_unif_flag_round(0);
        gl.set_unif_bg_color(&[0.0, 0.0, 0.0, 0.0]);

        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            self.vertex_num,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn set_color_buffer(
        &mut self,
        gl: &WebGlRenderingContext,
        id_table: &mut IdTable,
        terran_id: &BlockId,
        height: f32,
    ) {
        let mut colors_buffer = vec![];
        let color_offset = id_table.len();

        for color_idx in &self.colors {
            let id_color = (*color_idx + color_offset) as u32 | 0xFF000000;
            let color = crate::libs::color::Color::from(id_color).to_f32array();
            colors_buffer.push(color[0]);
            colors_buffer.push(color[1]);
            colors_buffer.push(color[2]);
            colors_buffer.push(color[3]);

            if id_table.get(&id_color).is_none() {
                if let Some((pn, n_idx)) = self.normals.get(color_idx) {
                    let pn = *pn as f32;
                    let surface = match (*n_idx) % 6 {
                        0 => Surface {
                            r: [pn, 0.0, 0.0],
                            s: [0.0, 1.0, 0.0],
                            t: [0.0, 0.0, 1.0],
                        },
                        1 => Surface {
                            r: [0.0, pn, 0.0],
                            s: [0.0, 0.0, 1.0],
                            t: [1.0, 0.0, 0.0],
                        },
                        2 => Surface {
                            r: [0.0, 0.0, pn * height],
                            s: [1.0, 0.0, 0.0],
                            t: [0.0, 1.0, 0.0],
                        },
                        3 => Surface {
                            r: [pn, 0.0, 0.0],
                            s: [0.0, 0.0, 1.0],
                            t: [0.0, 1.0, 0.0],
                        },
                        4 => Surface {
                            r: [0.0, pn, 0.0],
                            s: [1.0, 0.0, 0.0],
                            t: [0.0, 0.0, 1.0],
                        },
                        5 => Surface {
                            r: [0.0, 0.0, pn * height],
                            s: [0.0, 1.0, 0.0],
                            t: [1.0, 0.0, 0.0],
                        },
                        _ => unreachable!(),
                    };

                    let object_id = ObjectId::Terran(BlockId::clone(terran_id), surface);

                    id_table.insert(id_color, object_id);
                }
            }
        }

        self.colors_buffer = gl.create_vbo_with_f32array(&colors_buffer);
        gl.set_attr_color(&self.colors_buffer, 4, 0);
    }

    fn update_buffer(&mut self, gl: &WebGlRenderingContext, terran: &block::terran::Terran) {
        let mut vertexes = vec![];
        let mut indexes = vec![];
        let mut colors = vec![];
        let mut colors_table: HashMap<(i32, usize), usize> = HashMap::new();
        let mut vertexes_table: HashMap<([i32; 3], usize), i16> = HashMap::new();

        for (p, _) in terran.table().iter() {
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
                        &mut colors,
                        &mut colors_table,
                        &mut vertexes_table,
                        [p[0] + o[i][0][0], p[1] + o[i][0][1], p[2] + o[i][0][2]],
                        [p[0] + o[i][1][0], p[1] + o[i][1][1], p[2] + o[i][1][2]],
                        [p[0] + o[i][2][0], p[1] + o[i][2][1], p[2] + o[i][2][2]],
                        [p[0] + o[i][3][0], p[1] + o[i][3][1], p[2] + o[i][3][2]],
                        i,
                    );
                }
            }
        }

        self.normals = colors_table.into_iter().map(|(a, b)| (b, a)).collect();
        self.colors = colors;
        self.vertexes_buffer = gl.create_vbo_with_f32array(&vertexes);
        self.index_buffer = gl.create_ibo_with_i16array(&indexes);
        self.vertex_num = indexes.len() as i32;
    }

    fn push_surface(
        indexes: &mut Vec<i16>,
        vertexes: &mut Vec<f32>,
        colors: &mut Vec<usize>,
        colors_table: &mut HashMap<(i32, usize), usize>,
        vertexes_table: &mut HashMap<([i32; 3], usize), i16>,
        pp: [i32; 3],
        np: [i32; 3],
        pn: [i32; 3],
        nn: [i32; 3],
        n_idx: usize,
    ) {
        let pp_idx = Self::push_vertex(vertexes, colors, colors_table, vertexes_table, pp, n_idx);
        let np_idx = Self::push_vertex(vertexes, colors, colors_table, vertexes_table, np, n_idx);
        let pn_idx = Self::push_vertex(vertexes, colors, colors_table, vertexes_table, pn, n_idx);
        let nn_idx = Self::push_vertex(vertexes, colors, colors_table, vertexes_table, nn, n_idx);

        indexes.push(pp_idx);
        indexes.push(np_idx);
        indexes.push(pn_idx);
        indexes.push(nn_idx);
        indexes.push(pn_idx);
        indexes.push(np_idx);
    }

    fn push_vertex(
        vertexes: &mut Vec<f32>,
        colors: &mut Vec<usize>,
        colors_table: &mut HashMap<(i32, usize), usize>,
        vertexes_table: &mut HashMap<([i32; 3], usize), i16>,
        pos: [i32; 3],
        n_idx: usize,
    ) -> i16 {
        let pn = match n_idx % 6 {
            0 => pos[0],
            1 => pos[1],
            2 => pos[2],
            3 => pos[0],
            4 => pos[1],
            5 => pos[2],
            _ => unreachable!(),
        };
        let color_key = (pn, n_idx);

        let color = if let Some(color) = colors_table.get(&color_key) {
            *color
        } else {
            let color = colors_table.len();
            colors_table.insert(color_key, color);
            color
        };

        let vertex_key = (pos, color);
        if let Some(idx) = vertexes_table.get(&vertex_key) {
            *idx
        } else {
            vertexes.push(pos[0] as f32);
            vertexes.push(pos[1] as f32);
            vertexes.push(pos[2] as f32);
            colors.push(color);
            let idx = vertexes_table.len() as i16;
            vertexes_table.insert(vertex_key, idx);
            idx
        }
    }
}
