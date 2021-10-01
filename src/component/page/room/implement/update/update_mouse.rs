use super::{
    block, BlockId, BoxblockTool, CharacterTool, CloneOf, Implement, ObjectId, PointlightTool,
    ResourceId, ShapeTool, TableTool, TerranblockTool,
};

impl Implement {
    pub fn update_mouse(&mut self) -> bool {
        if self.mouse_state.primary_btn().is_dragging() && self.key_state.space_key {
            let last = self.mouse_state.cursor().last().position_in_canvas();
            let now = self.mouse_state.cursor().now().position_in_canvas();
            let mov_x = now[0] - last[0];
            let mov_y = now[1] - last[1];
            let intensity = 0.05;
            let mov = self.camera_matrix.movement();
            let mov = [
                mov[0] - mov_x * intensity,
                mov[1] + mov_y * intensity,
                mov[2],
            ];
            self.camera_matrix.set_movement(mov);
        } else if self.mouse_state.primary_btn().is_dragging() && self.key_state.alt_key {
            let last = self.mouse_state.cursor().last().position_in_canvas();
            let now = self.mouse_state.cursor().now().position_in_canvas();
            let mov_x = now[0] - last[0];
            let mov_y = now[1] - last[1];
            let intensity = 0.005;
            let rot_x = self.camera_matrix.x_axis_rotation();
            let rot_z = self.camera_matrix.z_axis_rotation();

            self.camera_matrix
                .set_x_axis_rotation(rot_x - mov_y * intensity, true);
            self.camera_matrix
                .set_z_axis_rotation(rot_z - mov_x * intensity);
        } else {
            match self.table_tools.selected() {
                Some(TableTool::Selector) => {
                    if self.mouse_state.primary_btn().is_dragging() {
                        if self.grabbed_object_id.is_none() {
                            self.grabbed_object_id = self.focused_object_id();
                        } else if let Some(renderer) = &self.renderer {
                            let (mut p, n) = renderer.get_focused_position(
                                &self.block_arena,
                                &self.camera_matrix,
                                self.mouse_state.cursor().now().position_in_canvas()[0],
                                self.mouse_state.cursor().now().position_in_canvas()[1],
                            );

                            p[0] = (p[0] * 2.0).round() / 2.0;
                            p[1] = (p[1] * 2.0).round() / 2.0;
                            p[2] = (p[2] * 2.0).round() / 2.0;

                            match &self.grabbed_object_id {
                                ObjectId::Character(character_id, _) => {
                                    self.block_arena.map_mut(
                                        character_id,
                                        |character: &mut block::character::Character| {
                                            character.set_position(p);
                                        },
                                    );
                                }
                                ObjectId::Boxblock(boxblock_id, _) => {
                                    self.block_arena.map_mut(
                                        boxblock_id,
                                        |boxblock: &mut block::boxblock::Boxblock| {
                                            let s = boxblock.size();
                                            let p = [
                                                p[0] + s[0] * n[0] * 0.5,
                                                p[1] + s[1] * n[1] * 0.5,
                                                p[2] + s[2] * n[2] * 0.5,
                                            ];
                                            boxblock.set_position(p);
                                        },
                                    );
                                }
                                ObjectId::Pointlight(pointlight_id, _) => {
                                    self.block_arena.map_mut(
                                        pointlight_id,
                                        |pointlight: &mut block::pointlight::Pointlight| {
                                            let s = [1.0, 1.0, 1.0];
                                            let p = [
                                                p[0] + s[0] * n[0] * 0.5,
                                                p[1] + s[1] * n[1] * 0.5,
                                                p[2] + s[2] * n[2] * 0.5,
                                            ];
                                            pointlight.set_position(p);
                                        },
                                    );
                                }
                                _ => {}
                            }
                        }
                    } else {
                        self.grabbed_object_id = ObjectId::None;
                    }
                }
                Some(TableTool::Pen(_)) => self.update_tabletool_pen(),
                Some(TableTool::Shape(shape_tool)) => match shape_tool.selected() {
                    Some(ShapeTool::Line(_)) => self.update_tabletool_shape_line(),
                    Some(ShapeTool::Rect(_)) => self.update_tabletool_shape_rect(),
                    Some(ShapeTool::Ellipse(_)) => self.update_tabletool_shape_ellipse(),
                    _ => {}
                },
                Some(TableTool::Eraser(_)) => self.update_tabletool_eraser(),
                Some(TableTool::Character(_)) => self.update_tabletool_character(),
                Some(TableTool::Terranblock(_)) => self.update_tabletool_terranblock(),
                Some(TableTool::TerranblockEraser) => self.update_tabletool_terranblock_eraser(),
                Some(TableTool::Boxblock(_)) => self.update_tabletool_boxblock(),
                Some(TableTool::Pointlight(_)) => self.update_tabletool_pointlight(),
                _ => {}
            }
        }

        true
    }

    fn focused_object_id(&self) -> ObjectId {
        self.renderer
            .as_ref()
            .map(|x| {
                x.get_object_id(
                    self.mouse_state.cursor().now().position_in_canvas()[0],
                    self.mouse_state.cursor().now().position_in_canvas()[1],
                )
            })
            .unwrap_or(ObjectId::None)
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

    fn update_tabletool_pen(&mut self) {
        let pen = match self.table_tools.selected() {
            Some(TableTool::Pen(x)) => x,
            _ => {
                return;
            }
        };

        if self.mouse_state.primary_btn().is_dragging() {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self.mouse_state.cursor().last().position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();
                let p = self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
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

                        texture.set_is_mask(false);

                        (a, b)
                    },
                );

                if let Some((a, b)) = p {
                    if self.mouse_state.primary_btn().is_downed() {
                        self.drawing_line = vec![a, b];
                    } else {
                        self.drawing_line.push(a);
                    }
                }
            }
        } else if self.mouse_state.primary_btn().is_upped() && self.drawing_line.len() >= 2 {
            let mut points = self
                .drawing_line
                .drain(..)
                .collect::<std::collections::VecDeque<_>>();

            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::texture::Texture| {
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

    fn update_tabletool_shape_line(&mut self) {
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

        if self.mouse_state.primary_btn().is_dragging() {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self
                    .mouse_state
                    .primary_btn()
                    .drag_start()
                    .position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
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
        } else if self.mouse_state.primary_btn().is_upped() {
            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                let a = self
                    .mouse_state
                    .primary_btn()
                    .drag_start()
                    .position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();

                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::texture::Texture| {
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

    fn update_tabletool_shape_rect(&mut self) {
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

        if self.mouse_state.primary_btn().is_dragging() {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self
                    .mouse_state
                    .primary_btn()
                    .drag_start()
                    .position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
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
        } else if self.mouse_state.primary_btn().is_upped() {
            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                let a = self
                    .mouse_state
                    .primary_btn()
                    .drag_start()
                    .position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();

                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::texture::Texture| {
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

    fn update_tabletool_shape_ellipse(&mut self) {
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

        if self.mouse_state.primary_btn().is_dragging() {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self
                    .mouse_state
                    .primary_btn()
                    .drag_start()
                    .position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
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
        } else if self.mouse_state.primary_btn().is_upped() {
            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                let a = self
                    .mouse_state
                    .primary_btn()
                    .drag_start()
                    .position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();

                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::texture::Texture| {
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

    fn update_tabletool_eraser(&mut self) {
        let eraser = match self.table_tools.selected() {
            Some(TableTool::Eraser(x)) => x,
            _ => {
                return;
            }
        };

        if self.mouse_state.primary_btn().is_dragging() {
            if let Some(drawing_texture_id) = self.drawing_texture_id() {
                let a = self.mouse_state.cursor().last().position_in_table();
                let b = self.mouse_state.cursor().now().position_in_table();
                let p = self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let a = texture.texture_position(&[a[0] as f64, a[1] as f64]);
                        let b = texture.texture_position(&[b[0] as f64, b[1] as f64]);
                        let context = texture.context();

                        context.begin_path();
                        let color = crate::libs::color::Pallet::gray(0).a(eraser.alpha);
                        context.set_stroke_style(&color.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(eraser.line_width);
                        context.move_to(a[0], a[1]);
                        context.line_to(b[0], b[1]);
                        context.stroke();

                        texture.set_is_mask(true);

                        (a, b)
                    },
                );

                if let Some((a, b)) = p {
                    if self.mouse_state.primary_btn().is_downed() {
                        self.drawing_line = vec![a, b];
                    } else {
                        self.drawing_line.push(a);
                    }
                }
            }
        } else if self.mouse_state.primary_btn().is_upped() && self.drawing_line.len() >= 2 {
            let mut points = self
                .drawing_line
                .drain(..)
                .collect::<std::collections::VecDeque<_>>();

            if let Some((drawing_texture_id, drawed_texture_id)) =
                join_some!(self.drawing_texture_id(), self.drawed_texture_id())
            {
                self.local_block_arena.map_mut(
                    &drawing_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let context = texture.context();
                        let sz = texture.buffer_size();
                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);

                        texture.set_is_mask(false);
                    },
                );

                self.block_arena.map_mut(
                    &drawed_texture_id,
                    |texture: &mut block::texture::Texture| {
                        let context = texture.context();

                        let a = points.pop_front().unwrap();

                        context.begin_path();
                        let color = crate::libs::color::Pallet::gray(0).a(eraser.alpha);
                        let _ = context.set_global_composite_operation("destination-out");
                        context.set_stroke_style(&color.to_color().to_jsvalue());
                        context.set_line_cap("round");
                        context.set_line_width(eraser.line_width);
                        context.set_line_join("round");
                        context.move_to(a[0], a[1]);

                        for b in points {
                            context.line_to(b[0], b[1]);
                        }

                        context.stroke();

                        let _ = context.set_global_composite_operation("source-over");
                    },
                );
            }
        }
    }

    fn update_tabletool_character(&mut self) {
        if self.mouse_state.primary_btn().is_clicked() {
            let character = if let Some(TableTool::Character(x)) = self.table_tools.selected() {
                CharacterTool::clone_of(x)
            } else {
                return;
            };
            let (p, _) = self.mouse_state.cursor().now().position_in_world();

            let p = [
                (p[0] * 2.0).round() / 2.0,
                (p[1] * 2.0).round() / 2.0,
                (p[2] * 2.0).round() / 2.0,
            ];

            self.create_new_character(
                Some(character.size as f32),
                Some(character.height as f32),
                character.tex_id.as_ref().map(|x| ResourceId::clone(x)),
                Some(character.name.clone()),
                Some(p),
            );
        }
    }

    fn update_tabletool_boxblock(&mut self) {
        if self.mouse_state.primary_btn().is_clicked() {
            let boxblock = if let Some(TableTool::Boxblock(x)) = self.table_tools.selected() {
                BoxblockTool::clone_of(x)
            } else {
                return;
            };

            let (p, n) = self.mouse_state.cursor().now().position_in_world();

            let p = [
                (p[0] * 2.0).round() / 2.0,
                (p[1] * 2.0).round() / 2.0,
                (p[2] * 2.0).round() / 2.0,
            ];

            let p = [
                p[0] + n[0] * boxblock.size[0] as f32 * 0.5,
                p[1] + n[1] * boxblock.size[1] as f32 * 0.5,
                p[2] + n[2] * boxblock.size[2] as f32 * 0.5,
            ];

            let s = boxblock.size;
            let s = [s[0] as f32, s[1] as f32, s[2] as f32];

            self.create_new_boxblock(p, s, boxblock.color, boxblock.shape);
        }
    }

    fn update_tabletool_terranblock(&mut self) {
        let terranblock = if let Some(TableTool::Terranblock(x)) = self.table_tools.selected() {
            TerranblockTool::clone_of(x)
        } else {
            return;
        };
        if terranblock.is_fillable {
            if self.mouse_state.primary_btn().is_clicked() {
                self.block_arena
                    .and_then(&self.world_id, |world: &block::world::World| {
                        self.block_arena.map(
                            world.selecting_table(),
                            |table: &block::table::Table| {
                                (
                                    BlockId::clone(table.drawed_terran_id()),
                                    table.size().clone(),
                                )
                            },
                        )
                    })
                    .map(|(drawed_terran_id, table_size)| {
                        let p = self.focused_terran_grid_area(
                            self.mouse_state.cursor().now().position_in_world(),
                        );
                        self.block_arena.map_mut(
                            &drawed_terran_id,
                            |terran: &mut block::terran::Terran| {
                                let mut stack = vec![p];
                                let table_size_offset = [
                                    table_size[0].floor() % 2.0 * 0.5 - 0.5,
                                    table_size[1].floor() % 2.0 * 0.5 - 0.5,
                                ];
                                while let Some(p) = stack.pop() {
                                    if (p[0] as f32 - table_size_offset[0]).abs()
                                        < table_size[0] * 0.5
                                        && (p[1] as f32 - table_size_offset[1]).abs()
                                            < table_size[1] * 0.5
                                        && (terran.is_adjasted(&p, &block::terran::Surface::NZ)
                                            || p[2] == 0)
                                    {
                                        if !terran.is_adjasted(&p, &block::terran::Surface::PX) {
                                            stack.push([p[0] + 1, p[1], p[2]]);
                                        }
                                        if !terran.is_adjasted(&p, &block::terran::Surface::PY) {
                                            stack.push([p[0], p[1] + 1, p[2]]);
                                        }
                                        if !terran.is_adjasted(&p, &block::terran::Surface::NX) {
                                            stack.push([p[0] - 1, p[1], p[2]]);
                                        }
                                        if !terran.is_adjasted(&p, &block::terran::Surface::NY) {
                                            stack.push([p[0], p[1] - 1, p[2]]);
                                        }
                                        terran.enqueue(
                                            p,
                                            block::terran::TerranBlock::new(
                                                terranblock.color.clone(),
                                            ),
                                        );
                                    }
                                }
                            },
                        );
                    });
            }
        } else {
            if self.mouse_state.primary_btn().is_dragging() {
                let focuesd = self
                    .focused_terran_grid_area(self.mouse_state.cursor().now().position_in_world());
                let last_focuesd = self
                    .focused_terran_grid_area(self.mouse_state.cursor().last().position_in_world());

                if focuesd != last_focuesd || self.mouse_state.primary_btn().is_downed() {
                    self.push_new_terranblock(focuesd, terranblock.color);
                }
            } else if self.mouse_state.primary_btn().is_upped() {
                self.flip_to_drawed_terran();
            }
        }
    }

    fn update_tabletool_terranblock_eraser(&mut self) {
        if self.mouse_state.primary_btn().is_clicked() {
            if let ObjectId::Terran(terran_id, _) = self.focused_object_id() {
                let (p, n) = self.mouse_state.cursor().now().position_in_world();
                let n = [-n[0], -n[1], -n[2]];
                let focused = self.focused_terran_grid_area((p, &n));

                self.block_arena
                    .map_mut(&terran_id, |terran: &mut block::terran::Terran| {
                        terran.remove_at(&focused);
                    });
            }
        }
    }

    fn focused_terran_grid_area(&self, (p, n): (&[f32; 3], &[f32; 3])) -> [i32; 3] {
        let (offset, height) = self
            .block_arena
            .map(&self.world_id, |world: &block::world::World| {
                BlockId::clone(world.selecting_table())
            })
            .and_then(|t_id| {
                self.block_arena.map(&t_id, |table: &block::table::Table| {
                    let sz = table.size();
                    (
                        [sz[0].floor() % 2.0 * 0.5, sz[1].floor() % 2.0 * 0.5],
                        table.terran_height(),
                    )
                })
            })
            .unwrap_or(([0.0, 0.0], 1.0));

        let p = [
            (p[0] + n[0] * 0.5 + offset[0]).floor(),
            (p[1] + n[1] * 0.5 + offset[1]).floor(),
            (p[2] / height + n[2] * 0.5).floor(),
        ];
        [p[0] as i32, p[1] as i32, p[2] as i32]
    }

    fn update_tabletool_pointlight(&mut self) {
        if self.mouse_state.primary_btn().is_clicked() {
            let pointlight = if let Some(TableTool::Pointlight(x)) = self.table_tools.selected() {
                PointlightTool::clone_of(x)
            } else {
                return;
            };

            let (p, n) = self.mouse_state.cursor().now().position_in_world();

            let p = [
                (p[0] * 2.0).round() / 2.0,
                (p[1] * 2.0).round() / 2.0,
                (p[2] * 2.0).round() / 2.0,
            ];

            let p = [p[0] + n[0] * 0.5, p[1] + n[1] * 0.5, p[2] + n[2] * 0.5];

            self.create_new_pointlight(
                p,
                pointlight.light_intensity as f32,
                pointlight.light_attenation as f32,
                pointlight.color,
            );
        }
    }
}
