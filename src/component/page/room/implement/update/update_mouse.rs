use super::{block, BlockId, Implement, ShapeTool, TableTool};

impl Implement {
    pub fn update_mouse(&mut self) -> bool {
        let mut need_update = false;

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

        let selecting_table_id = self
            .block_arena
            .map(&self.world_id, |world: &block::world::World| {
                BlockId::clone(&world.selecting_table())
            });
        let drawing_texture_id = selecting_table_id.as_ref().and_then(|b_id| {
            self.block_arena.map(&b_id, |table: &block::table::Table| {
                BlockId::clone(&table.drawing_texture_id())
            })
        });
        let drawed_texture_id = selecting_table_id.as_ref().and_then(|b_id| {
            self.block_arena.map(&b_id, |table: &block::table::Table| {
                BlockId::clone(&table.drawed_texture_id())
            })
        });

        match self.table_tools.selected() {
            Some(TableTool::Pen(pen)) => {
                if self.mouse_state.is_dragging {
                    if let Some(drawing_texture_id) = drawing_texture_id {
                        let a = self.camera_matrix.collision_point_on_xy_plane(
                            &self.canvas_size,
                            &[last_client_x, last_client_y],
                        );
                        let b = self
                            .camera_matrix
                            .collision_point_on_xy_plane(&self.canvas_size, &[client_x, client_y]);
                        let p = self.block_arena.map_mut(
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

                                need_update = true;

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
                } else if self.mouse_state.is_changed_dragging_state && self.drawing_line.len() >= 2
                {
                    let mut points = self
                        .drawing_line
                        .drain(..)
                        .collect::<std::collections::VecDeque<_>>();

                    if let Some((drawing_texture_id, drawed_texture_id)) =
                        join_some!(drawing_texture_id, drawed_texture_id)
                    {
                        self.block_arena.map_mut(
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

                        need_update = true;
                    }
                }
            }
            Some(TableTool::Shape(shape_tool)) => match shape_tool.selected() {
                Some(ShapeTool::Line(line)) => {
                    if self.mouse_state.is_dragging {
                        if let Some(drawing_texture_id) = drawing_texture_id {
                            let a = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[changing_client_x, changing_client_y],
                            );
                            let b = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[client_x, client_y],
                            );
                            self.block_arena.map_mut(
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

                                    need_update = true;
                                },
                            );
                        }
                    } else if self.mouse_state.is_changed_dragging_state {
                        if let Some((drawing_texture_id, drawed_texture_id)) =
                            join_some!(drawing_texture_id, drawed_texture_id)
                        {
                            let a = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[last_changing_client_x, last_changing_client_y],
                            );
                            let b = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[client_x, client_y],
                            );

                            self.block_arena.map_mut(
                                &drawing_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let context = texture.context();
                                    let sz = texture.buffer_size();
                                    context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                                    need_update = true;
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

                                    need_update = true;
                                },
                            );
                        }
                    }
                }
                Some(ShapeTool::Rect(rect)) => {
                    if self.mouse_state.is_dragging {
                        if let Some(drawing_texture_id) = drawing_texture_id {
                            let a = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[changing_client_x, changing_client_y],
                            );
                            let b = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[client_x, client_y],
                            );
                            self.block_arena.map_mut(
                                &drawing_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                                    let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                                    let context = texture.context();
                                    let sz = texture.buffer_size();
                                    context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                                    context.set_stroke_style(
                                        &rect.line_pallet.to_color().to_jsvalue(),
                                    );
                                    context
                                        .set_fill_style(&rect.fill_pallet.to_color().to_jsvalue());
                                    context.set_line_cap("round");
                                    context.set_line_width(rect.line_width);
                                    context.set_line_join("round");
                                    context.fill_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);
                                    context.stroke_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);

                                    need_update = true;
                                },
                            );
                        }
                    } else if self.mouse_state.is_changed_dragging_state {
                        if let Some((drawing_texture_id, drawed_texture_id)) =
                            join_some!(drawing_texture_id, drawed_texture_id)
                        {
                            let a = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[last_changing_client_x, last_changing_client_y],
                            );
                            let b = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[client_x, client_y],
                            );

                            self.block_arena.map_mut(
                                &drawing_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let context = texture.context();
                                    let sz = texture.buffer_size();
                                    context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                                    need_update = true;
                                },
                            );

                            self.block_arena.map_mut(
                                &drawed_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                                    let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                                    let context = texture.context();
                                    context.set_stroke_style(
                                        &rect.line_pallet.to_color().to_jsvalue(),
                                    );
                                    context
                                        .set_fill_style(&rect.fill_pallet.to_color().to_jsvalue());
                                    context.set_line_cap("round");
                                    context.set_line_width(rect.line_width);
                                    context.set_line_join("round");
                                    context.fill_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);
                                    context.stroke_rect(a[0], a[1], b[0] - a[0], b[1] - a[1]);

                                    need_update = true;
                                },
                            );
                        }
                    }
                }
                Some(ShapeTool::Ellipse(ellipse)) => {
                    if self.mouse_state.is_dragging {
                        if let Some(drawing_texture_id) = drawing_texture_id {
                            let a = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[changing_client_x, changing_client_y],
                            );
                            let b = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[client_x, client_y],
                            );
                            self.block_arena.map_mut(
                                &drawing_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                                    let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                                    let context = texture.context();
                                    let sz = texture.buffer_size();
                                    context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                                    context.set_stroke_style(
                                        &ellipse.line_pallet.to_color().to_jsvalue(),
                                    );
                                    context.set_fill_style(
                                        &ellipse.fill_pallet.to_color().to_jsvalue(),
                                    );
                                    context.set_line_cap("round");
                                    context.set_line_width(ellipse.line_width);
                                    context.set_line_join("round");
                                    context.begin_path();
                                    let _ = context.ellipse(
                                        a[0],
                                        a[1],
                                        (b[0] - a[0]).abs(),
                                        (b[1] - a[1]).abs(),
                                        0.0,
                                        0.0,
                                        2.0 * std::f64::consts::PI,
                                    );
                                    context.fill();
                                    context.stroke();

                                    need_update = true;
                                },
                            );
                        }
                    } else if self.mouse_state.is_changed_dragging_state {
                        if let Some((drawing_texture_id, drawed_texture_id)) =
                            join_some!(drawing_texture_id, drawed_texture_id)
                        {
                            let a = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[last_changing_client_x, last_changing_client_y],
                            );
                            let b = self.camera_matrix.collision_point_on_xy_plane(
                                &self.canvas_size,
                                &[client_x, client_y],
                            );

                            self.block_arena.map_mut(
                                &drawing_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let context = texture.context();
                                    let sz = texture.buffer_size();
                                    context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                                    need_update = true;
                                },
                            );

                            self.block_arena.map_mut(
                                &drawed_texture_id,
                                |texture: &mut block::table::texture::Texture| {
                                    let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                                    let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                                    let context = texture.context();
                                    context.set_stroke_style(
                                        &ellipse.line_pallet.to_color().to_jsvalue(),
                                    );
                                    context.set_fill_style(
                                        &ellipse.fill_pallet.to_color().to_jsvalue(),
                                    );
                                    context.set_line_cap("round");
                                    context.set_line_width(ellipse.line_width);
                                    context.set_line_join("round");
                                    context.begin_path();
                                    let _ = context.ellipse(
                                        a[0],
                                        a[1],
                                        (b[0] - a[0]).abs(),
                                        (b[1] - a[1]).abs(),
                                        0.0,
                                        0.0,
                                        2.0 * std::f64::consts::PI,
                                    );
                                    context.fill();
                                    context.stroke();

                                    need_update = true;
                                },
                            );
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        need_update
    }
}
