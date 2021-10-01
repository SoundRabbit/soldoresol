use super::id_table::{IdColor, IdTable, ObjectId, Surface};
use super::matrix::model::ModelMatrix;
use super::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Plane {
    surface: block::terran::Surface,
    pos: i32,
}

pub struct Terran {
    vertex_buffer: WebGlF32Vbo,
    v_color_buffer: WebGlF32Vbo,
    id_color_buffer: WebGlF32Vbo,
    normal_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
    colors: Vec<usize>,
    color_table: HashMap<Plane, IdColor>,
    vertex_num: i32,
    last_terran_id: BlockId,
    terran_update_time: f64,
}

impl Terran {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_buffer = gl.create_vbo_with_f32array(&[]);
        let v_color_buffer = gl.create_vbo_with_f32array(&[]);
        let id_color_buffer = gl.create_vbo_with_f32array(&[]);
        let normal_buffer = gl.create_vbo_with_f32array(&[]);
        let texture_coord_buffer = gl.create_vbo_with_f32array(&[]);
        let index_buffer = gl.create_ibo_with_i16array(&[]);

        Self {
            vertex_buffer,
            v_color_buffer,
            id_color_buffer,
            normal_buffer,
            texture_coord_buffer,
            index_buffer,
            colors: vec![],
            color_table: HashMap::new(),
            vertex_num: 0,
            last_terran_id: BlockId::none(),
            terran_update_time: 0.0,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        id_table: &mut IdTable,
        id_value: &mut HashMap<BlockId, u32>,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        table: &block::table::Table,
        terran_id: &BlockId,
    ) {
        if block_arena.timestamp_of(terran_id).unwrap_or(0.0) > self.terran_update_time
            || *terran_id != self.last_terran_id
        {
            block_arena.map(terran_id, |terran: &block::terran::Terran| {
                self.update_buffer(gl, terran);
            });
        }

        gl.use_program(ProgramType::UnshapedProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);

        gl.set_a_vertex(&self.vertex_buffer, 3, 0);
        gl.set_a_texture_coord(&self.texture_coord_buffer, 2, 0);
        gl.set_a_id_color(&self.id_color_buffer, 4, 0);
        gl.set_a_v_color(&self.v_color_buffer, 4, 0);
        gl.set_a_normal(&self.normal_buffer, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        gl.set_u_shape(program::SHAPE_3D_BOX);
        gl.set_u_bg_color_1(program::COLOR_NONE);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_id(program::ID_V_WRITE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_light(program::LIGHT_NONE);

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
        gl.set_u_translate(mvp_matrix.reversed_axes());
        let id_offset = self.set_id_table(id_table, terran_id, table.terran_height());
        gl.set_u_id_value(id_offset as i32);

        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            self.vertex_num,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn set_id_table(&mut self, id_table: &mut IdTable, terran_id: &BlockId, height: f32) -> u32 {
        let id_color_offset = id_table.len() as u32;

        for (plane, id_color) in &self.color_table {
            let id_color = id_color.clone_with_offset(id_color_offset);
            let pn = plane.pos as f32;
            let surface = match plane.surface {
                block::terran::Surface::PX => Surface {
                    r: [pn, 0.0, 0.0],
                    s: [0.0, 1.0, 0.0],
                    t: [0.0, 0.0, 1.0],
                },
                block::terran::Surface::PY => Surface {
                    r: [0.0, pn, 0.0],
                    s: [0.0, 0.0, 1.0],
                    t: [1.0, 0.0, 0.0],
                },
                block::terran::Surface::PZ => Surface {
                    r: [0.0, 0.0, pn * height],
                    s: [1.0, 0.0, 0.0],
                    t: [0.0, 1.0, 0.0],
                },
                block::terran::Surface::NX => Surface {
                    r: [pn, 0.0, 0.0],
                    s: [0.0, 0.0, 1.0],
                    t: [0.0, 1.0, 0.0],
                },
                block::terran::Surface::NY => Surface {
                    r: [0.0, pn, 0.0],
                    s: [1.0, 0.0, 0.0],
                    t: [0.0, 0.0, 1.0],
                },
                block::terran::Surface::NZ => Surface {
                    r: [0.0, 0.0, pn * height],
                    s: [0.0, 1.0, 0.0],
                    t: [1.0, 0.0, 0.0],
                },
            };

            let object_id = ObjectId::Terran(BlockId::clone(terran_id), surface);
            id_table.insert(id_color, object_id);
        }

        id_color_offset
    }

    fn update_buffer(&mut self, gl: &WebGlRenderingContext, terran: &block::terran::Terran) {
        let mut vertex = vec![];
        let mut index = vec![];
        let mut normal = vec![];
        let mut v_color = vec![];
        let mut id_color = vec![];
        let mut texture_coord = vec![];
        let mut color_table: HashMap<Plane, IdColor> = HashMap::new();
        let mut vertex_table: HashMap<([i32; 3], IdColor), i16> = HashMap::new();

        for (p, _) in terran.table().iter() {
            for s in block::terran::Surface::iter() {
                if !terran.is_covered(p, &s) {
                    let o = match s {
                        block::terran::Surface::PX => [[1, 1, 1], [1, 0, 1], [1, 1, 0], [1, 0, 0]],
                        block::terran::Surface::PY => [[1, 1, 1], [1, 1, 0], [0, 1, 1], [0, 1, 0]],
                        block::terran::Surface::PZ => [[1, 1, 1], [0, 1, 1], [1, 0, 1], [0, 0, 1]],
                        block::terran::Surface::NX => [[0, 1, 1], [0, 1, 0], [0, 0, 1], [0, 0, 0]],
                        block::terran::Surface::NY => [[1, 0, 1], [0, 0, 1], [1, 0, 0], [0, 0, 0]],
                        block::terran::Surface::NZ => [[1, 1, 0], [1, 0, 0], [0, 1, 0], [0, 0, 0]],
                    };
                    let pp = [p[0] + o[0][0], p[1] + o[0][1], p[2] + o[0][2]];
                    let np = [p[0] + o[1][0], p[1] + o[1][1], p[2] + o[1][2]];
                    let pn = [p[0] + o[2][0], p[1] + o[2][1], p[2] + o[2][2]];
                    let nn = [p[0] + o[3][0], p[1] + o[3][1], p[2] + o[3][2]];

                    Self::push_surface(
                        &mut index,
                        &mut vertex,
                        &mut normal,
                        &mut v_color,
                        &mut id_color,
                        &mut texture_coord,
                        &mut color_table,
                        &mut vertex_table,
                        [pp, np, pn, nn],
                        s,
                    );
                }
            }
        }

        self.color_table = color_table;
        self.vertex_buffer = gl.create_vbo_with_f32array(&vertex);
        self.id_color_buffer = gl.create_vbo_with_f32array(&id_color);
        self.normal_buffer = gl.create_vbo_with_f32array(&normal);
        self.texture_coord_buffer = gl.create_vbo_with_f32array(&texture_coord);
        self.index_buffer = gl.create_ibo_with_i16array(&index);
        self.vertex_num = vertex_table.len() as i32;
    }

    fn push_surface(
        index: &mut Vec<i16>,
        vertex: &mut Vec<f32>,
        normal: &mut Vec<f32>,
        v_color: &mut Vec<f32>,
        id_color: &mut Vec<f32>,
        texture_coord: &mut Vec<f32>,
        id_color_table: &mut HashMap<Plane, IdColor>,
        idx_table: &mut HashMap<([i32; 3], IdColor), i16>,
        pos: [[i32; 3]; 4],
        surface: block::terran::Surface,
    ) {
        let idx = [0, 0, 0, 0];
        for i in 0..4 {
            idx[i] = Self::push_vertex(
                vertex,
                normal,
                v_color,
                id_color,
                texture_coord,
                id_color_table,
                idx_table,
                pos[i],
                surface,
            );
        }

        let [pp, np, pn, nn] = idx;

        index.push(pp);
        index.push(np);
        index.push(pn);
        index.push(nn);
        index.push(pn);
        index.push(np);
    }

    fn push_vertex(
        vertex: &mut Vec<f32>,
        normal: &mut Vec<f32>,
        v_color: &mut Vec<f32>,
        id_color: &mut Vec<f32>,
        texture_coord: &mut Vec<f32>,
        id_color_table: &mut HashMap<Plane, IdColor>,
        index_table: &mut HashMap<([i32; 3], IdColor), i16>,
        pos: [i32; 3],
        surface: block::terran::Surface,
    ) -> i16 {
        let plane_pos = match surface {
            block::terran::Surface::PX | block::terran::Surface::NX => pos[0],
            block::terran::Surface::PY | block::terran::Surface::NY => pos[1],
            block::terran::Surface::PZ | block::terran::Surface::NZ => pos[2],
        };

        let plane = Plane {
            surface: surface,
            pos: plane_pos,
        };

        let color = if let Some(color) = id_color_table.get(&plane) {
            *color
        } else {
            let color = IdColor::from(id_color_table.len() as u32);
            id_color_table.insert(plane, color);
            color
        };

        let vertex_key = (pos.clone(), color);
        if let Some(idx) = index_table.get(&vertex_key) {
            *idx
        } else {
            vertex.push(pos[0] as f32);
            vertex.push(pos[1] as f32);
            vertex.push(pos[2] as f32);

            let color = color.to_f32array();
            id_color.push(color[0]);
            id_color.push(color[1]);
            id_color.push(color[2]);
            id_color.push(color[3]);

            v_color.push(0.0);
            v_color.push(0.0);
            v_color.push(0.0);
            v_color.push(0.0);

            let n = match surface {
                block::terran::Surface::PX => [1.0, 0.0, 0.0],
                block::terran::Surface::PY => [0.0, 1.0, 0.0],
                block::terran::Surface::PZ => [0.0, 0.0, 1.0],
                block::terran::Surface::NX => [-1.0, 0.0, 0.0],
                block::terran::Surface::NY => [0.0, -1.0, 0.0],
                block::terran::Surface::NZ => [0.0, 0.0, -1.0],
            };

            normal.push(n[0]);
            normal.push(n[1]);
            normal.push(n[2]);

            texture_coord.push(0.0);
            texture_coord.push(0.0);

            let idx = index_table.len() as i16;
            index_table.insert(vertex_key, idx);
            idx
        }
    }
}
