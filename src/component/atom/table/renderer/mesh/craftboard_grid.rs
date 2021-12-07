use super::libs::matrix::model::ModelMatrix;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::{block, BlockRef};
use crate::libs::random_id::U128Id;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

struct Buffer {
    index: WebGlI16Ibo,
    index_len: i32,
    vertex: WebGlF32Vbo,
    v_color: WebGlF32Vbo,
    id: WebGlF32Vbo,
    normal: WebGlF32Vbo,
    texture_coord: WebGlF32Vbo,
}

struct Craftboard {
    buffer: Buffer,
    size: [u64; 2],
}

pub struct CraftboardGrid {
    boards: HashMap<U128Id, Craftboard>,
}

impl CraftboardGrid {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        Self {
            boards: HashMap::new(),
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        camera_position: &[f32; 3],
        craftboards: impl Iterator<Item = BlockRef<block::Craftboard>>,
    ) {
        gl.use_program(ProgramType::UnshapedProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.set_u_expand(0.0);
        gl.set_u_id(program::ID_NONE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_light(program::LIGHT_NONE);
        gl.set_u_bg_color_1(program::COLOR_SOME);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_vp_matrix(vp_matrix.clone().reversed_axes());
        gl.set_u_shape(program::SHAPE_2D_BOX);
        gl.set_u_camera_position(camera_position);
        let mut unrendered: HashSet<U128Id> =
            HashSet::from_iter(self.boards.keys().map(U128Id::clone));

        for craftboard in craftboards {
            let craftboard_id = craftboard.id();
            craftboard.map(|craftboard| {
                unrendered.remove(&craftboard_id);

                let this = if let Some(this) = self.boards.get_mut(&craftboard_id) {
                    this
                } else {
                    let craftboard_size = {
                        let sz = craftboard.size();
                        [sz[0].floor() as u64, sz[1].floor() as u64]
                    };
                    let buffer = Self::create_grid_buffers(&gl, &craftboard_size);

                    let board = Craftboard {
                        buffer,
                        size: craftboard_size,
                    };

                    self.boards.insert(craftboard_id.clone(), board);
                    self.boards.get_mut(&craftboard_id).unwrap()
                };

                Self::render_craftboard(this, gl, vp_matrix, craftboard);
            });
        }

        for craftboard_id in unrendered {
            self.boards.remove(&craftboard_id);
        }
    }

    fn render_craftboard(
        this: &mut Craftboard,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        craftboard: &block::Craftboard,
    ) {
        let craftboard_size = {
            let sz = craftboard.size();
            [sz[0].floor() as u64, sz[1].floor() as u64]
        };
        let grid_color = craftboard.grid_color().to_color().to_f32array();

        if craftboard_size[0] != this.size[0] || craftboard_size[1] != this.size[1] {
            let buffer = Self::create_grid_buffers(&gl, &craftboard_size);

            this.buffer = buffer;
            this.size = craftboard_size;
        }

        gl.line_width(2.0);
        gl.set_a_vertex(&this.buffer.vertex, 3, 0);
        gl.set_a_id(&this.buffer.id, 1, 0);
        gl.set_a_v_color(&this.buffer.v_color, 4, 0);
        gl.set_a_normal(&this.buffer.normal, 3, 0);
        gl.set_a_texture_coord(&this.buffer.texture_coord, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&this.buffer.index),
        );

        let p = craftboard.position();
        let p = [p[0] as f32, p[1] as f32, p[2] as f32];
        let model_matrix: Array2<f32> = ModelMatrix::new().with_movement(&p).into();
        let inv_model_matrix: Array2<f32> = ModelMatrix::new()
            .with_movement(&[-p[0], -p[1], -p[2]])
            .into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);
        gl.set_u_translate(mvp_matrix.reversed_axes());

        gl.set_u_inv_model_matrix(inv_model_matrix.reversed_axes());
        gl.set_u_model_matrix(model_matrix.reversed_axes());
        gl.set_u_bg_color_1_value(&grid_color);

        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::LINES,
            this.buffer.index_len,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn create_grid_buffers(gl: &WebGlRenderingContext, table_size: &[u64; 2]) -> Buffer {
        let width = table_size[0];
        let height = table_size[1];

        let x_offset = width as f32 / 2.0;
        let y_offset = height as f32 / 2.0;

        let mut grid_vertex = vec![];
        let mut grid_texture_coord = vec![];
        let mut grid_idx_len = 0;

        for x in 0..(width + 1) {
            let x = x as f32 - x_offset;
            grid_vertex.append(&mut vec![x, -y_offset, 0.5 / 128.0]);
            grid_vertex.append(&mut vec![x, y_offset, 0.5 / 128.0]);
            grid_texture_coord.append(&mut vec![x, -y_offset]);
            grid_texture_coord.append(&mut vec![x, y_offset]);
            grid_idx_len += 2;
        }

        for y in 0..(height + 1) {
            let y = y as f32 - y_offset;
            grid_vertex.append(&mut vec![-x_offset, y, 0.5 / 128.0]);
            grid_vertex.append(&mut vec![x_offset, y, 0.5 / 128.0]);
            grid_texture_coord.append(&mut vec![-x_offset, y]);
            grid_texture_coord.append(&mut vec![x_offset, y]);
            grid_idx_len += 2;
        }

        let mut grid_idx = vec![];
        let mut grid_v_color = vec![];
        let mut grid_id = vec![];
        let mut grid_normal = vec![];

        for idx in 0..grid_idx_len {
            grid_idx.push(idx as i16);
            grid_v_color.append(&mut vec![0.0, 0.0, 0.0, 0.0]);
            grid_id.push(0.0);
            grid_normal.append(&mut vec![0.0, 0.0, 1.0]);
        }

        Buffer {
            index: gl.create_ibo_with_i16array(&grid_idx),
            index_len: grid_idx_len,
            vertex: gl.create_vbo_with_f32array(&grid_vertex),
            texture_coord: gl.create_vbo_with_f32array(&grid_texture_coord),
            v_color: gl.create_vbo_with_f32array(&grid_v_color),
            id: gl.create_vbo_with_f32array(&grid_id),
            normal: gl.create_vbo_with_f32array(&grid_normal),
        }
    }
}
