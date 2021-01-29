use super::{block, BlockId, Implement, ShapeTool, TableTool};

impl Implement {
    pub fn update_mouse(&mut self) -> bool {
        let mouse_point = &self.mouse_state.now_point;
        let client_x = mouse_point[0] - self.canvas_pos[0];
        let client_y = mouse_point[1] - self.canvas_pos[1];
        let last_point = &self.mouse_state.last_point;
        let last_client_x = last_point[0] - self.canvas_pos[0];
        let last_client_y = last_point[1] - self.canvas_pos[1];
        let changing_point = &self.mouse_state.changing_point;
        let changing_client_x = changing_point[0] - self.canvas_pos[0];
        let changing_client_y = changing_point[1] - self.canvas_pos[1];
        let last_changing_point = &self.mouse_state.last_changing_point;
        let last_changing_client_x = last_changing_point[0] - self.canvas_pos[0];
        let last_changing_client_y = last_changing_point[1] - self.canvas_pos[1];

        if self.mouse_state.is_dragging && (self.key_state.space_key || self.key_state.alt_key) {
            if self.key_state.space_key {
                let mov_x = (client_x - last_client_x) as f32;
                let mov_y = (client_y - last_client_y) as f32;
                let intensity = 0.05;
                let mov = self.camera_matrix.movement();
                let mov = [
                    mov[0] - mov_x * intensity,
                    mov[1] + mov_y * intensity,
                    mov[2],
                ];
                self.camera_matrix.set_movement(mov);
            }
            if self.key_state.alt_key {
                let mov_x = (client_x - last_client_x) as f32;
                let mov_y = (client_y - last_client_y) as f32;
                let intensity = 0.005;
                let rot_x = self.camera_matrix.x_axis_rotation();
                let rot_z = self.camera_matrix.z_axis_rotation();

                self.camera_matrix
                    .set_x_axis_rotation(rot_x - mov_y * intensity);
                self.camera_matrix
                    .set_z_axis_rotation(rot_z - mov_x * intensity);
            }
        } else {
            match self.table_tools.selected() {
                Some(TableTool::Pen(_)) => {
                    self.update_tabletool_pen(client_x, client_y, last_client_x, last_client_y)
                }
                Some(TableTool::Shape(shape_tool)) => match shape_tool.selected() {
                    Some(ShapeTool::Line(_)) => self.update_tabletool_shape_line(
                        client_x,
                        client_y,
                        changing_client_x,
                        changing_client_y,
                        last_changing_client_x,
                        last_changing_client_y,
                    ),
                    Some(ShapeTool::Rect(_)) => self.update_tabletool_shape_rect(
                        client_x,
                        client_y,
                        changing_client_x,
                        changing_client_y,
                        last_changing_client_x,
                        last_changing_client_y,
                    ),
                    Some(ShapeTool::Ellipse(_)) => self.update_tabletool_shape_ellipse(
                        client_x,
                        client_y,
                        changing_client_x,
                        changing_client_y,
                        last_changing_client_x,
                        last_changing_client_y,
                    ),
                    _ => {}
                },
                _ => {}
            }
        }

        true
    }

    fn selecting_table_id(&self) -> Option<BlockId> {
        self.block_arena
            .map(&self.world_id, |world: &block::world::World| {
                BlockId::clone(&world.selecting_table())
            })
    }

    fn drawing_texture_id(&self) -> Option<BlockId> {
        self.selecting_table_id().and_then(|b_id| {
            self.block_arena.map(&b_id, |table: &block::table::Table| {
                BlockId::clone(&table.drawing_texture_id())
            })
        })
    }

    fn drawed_texture_id(&self) -> Option<BlockId> {
        self.selecting_table_id().and_then(|b_id| {
            self.block_arena.map(&b_id, |table: &block::table::Table| {
                BlockId::clone(&table.drawed_texture_id())
            })
        })
    }

    fn update_tabletool_pen(
        &mut self,
        client_x: f32,
        client_y: f32,
        last_client_x: f32,
        last_client_y: f32,
    ) {
        let pen = match self.table_tools.selected() {
            Some(TableTool::Pen(x)) => x,
            _ => {
                return;
            }
        };

        if self.mouse_state.is_dragging {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[last_client_x, last_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);
                let p = self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();

                        context.begin_path();
                        context.set_stroke_style(&pen.pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(pen.line_width);
                        context.move_to(a[0], a[1]);
                        context.line_to(b[0], b[1]);
                        context.stroke();

                        (a, b)
                    },
                );

                if let Some((a, b)) = p {
                    if self.mouse_state.is_changed_dragging_state {
                        self.drawing_line = vec![a, b];
                    } else {
                        self.drawing_line.push(a);
                    }
                }
            }
        } else if self.mouse_state.is_changed_dragging_state && self.drawing_line.len() >= 2 {
            let mut points = self
                .drawing_line
                .drain(..)
                .collect::<std::collections::VecDeque<_>>();

            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let context = texture.context();

                        let a = points.pop_front().unwrap();

                        context.begin_path();
                        context.set_stroke_style(&pen.pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(pen.line_width);
                        context.set_line_join("round");
                        context.move_to(a[0], a[1]);

                        for b in points {
                            context.line_to(b[0], b[1]);
                        }

                        context.stroke();
                    },
                );
            }
        }
    }

    fn update_tabletool_shape_line(
        &mut self,
        client_x: f32,
        client_y: f32,
        changing_client_x: f32,
        changing_client_y: f32,
        last_changing_client_x: f32,
        last_changing_client_y: f32,
    ) {
        let line = match self.table_tools.selected() {
            Some(TableTool::Shape(shape_tool)) => match shape_tool.selected() {
                Some(ShapeTool::Line(x)) => x,
                _ => {
                    return;
                }
            },
            _ => {
                return;
            }
        };

        if self.mouse_state.is_dragging {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[changing_client_x, changing_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                        context.begin_path();
                        context.set_stroke_style(&line.pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(line.line_width);
                        context.move_to(a[0], a[1]);
                        context.line_to(b[0], b[1]);
                        context.stroke();
                    },
                );
            }
        } else if self.mouse_state.is_changed_dragging_state {
            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[last_changing_client_x, last_changing_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);

                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();

                        context.begin_path();
                        context.set_stroke_style(&line.pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(line.line_width);
                        context.move_to(a[0], a[1]);
                        context.line_to(b[0], b[1]);
                        context.stroke();
                    },
                );
            }
        }
    }

    fn update_tabletool_shape_rect(
        &mut self,
        client_x: f32,
        client_y: f32,
        changing_client_x: f32,
        changing_client_y: f32,
        last_changing_client_x: f32,
        last_changing_client_y: f32,
    ) {
        let rect = match self.table_tools.selected() {
            Some(TableTool::Shape(shape_tool)) => match shape_tool.selected() {
                Some(ShapeTool::Rect(x)) => x,
                _ => {
                    return;
                }
            },
            _ => {
                return;
            }
        };

        if self.mouse_state.is_dragging {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[changing_client_x, changing_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                        context.set_stroke_style(&rect.line_pallet.to_color().to_jsvalue());
                        context.set_fill_style(&rect.fill_pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(rect.line_width);
                        context.set_line_join("round");
                        context.fill_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);
                        context.stroke_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);
                    },
                );
            }
        } else if self.mouse_state.is_changed_dragging_state {
            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[last_changing_client_x, last_changing_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);

                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();
                        context.set_stroke_style(&rect.line_pallet.to_color().to_jsvalue());
                        context.set_fill_style(&rect.fill_pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(rect.line_width);
                        context.set_line_join("round");
                        context.fill_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);
                        context.stroke_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);
                    },
                );
            }
        }
    }

    fn update_tabletool_shape_ellipse(
        &mut self,
        client_x: f32,
        client_y: f32,
        changing_client_x: f32,
        changing_client_y: f32,
        last_changing_client_x: f32,
        last_changing_client_y: f32,
    ) {
        let ellipse = match self.table_tools.selected() {
            Some(TableTool::Shape(shape_tool)) => match shape_tool.selected() {
                Some(ShapeTool::Ellipse(x)) => x,
                _ => {
                    return;
                }
            },
            _ => {
                return;
            }
        };

        if self.mouse_state.is_dragging {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[changing_client_x, changing_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                        context.set_stroke_style(&ellipse.line_pallet.to_color().to_jsvalue());
                        context.set_fill_style(&ellipse.fill_pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(ellipse.line_width);
                        context.set_line_join("round");
                        context.begin_path();
                        let _ = context.ellipse(
                            a[0],
                            a[1],
                            (b[0] - a[0]).abs() * 2f64.sqrt(),
                            (b[1] - a[1]).abs() * 2f64.sqrt(),
                            0.0,
                            0.0,
                            2.0 * std::f64::consts::PI,
                        );
                        context.fill();
                        context.stroke();
                    },
                );
            }
        } else if self.mouse_state.is_changed_dragging_state {
            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                let a = self.camera_matrix.collision_point_on_xy_plane(
                    &self.canvas_size,
                    &[last_changing_client_x, last_changing_client_y],
                );
                let b = self
                    .camera_matrix
                    .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);

                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::table::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();
                        context.set_stroke_style(&ellipse.line_pallet.to_color().to_jsvalue());
                        context.set_fill_style(&ellipse.fill_pallet.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(ellipse.line_width);
                        context.set_line_join("round");
                        context.begin_path();
                        let _ = context.ellipse(
                            a[0],
                            a[1],
                            (b[0] - a[0]).abs() * 2f64.sqrt(),
                            (b[1] - a[1]).abs() * 2f64.sqrt(),
                            0.0,
                            0.0,
                            2.0 * std::f64::consts::PI,
                        );
                        context.fill();
                        context.stroke();
                    },
                );
            }
        }
    }
}
