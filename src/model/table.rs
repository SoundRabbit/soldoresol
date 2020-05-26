use super::{color::Color, TexstureLayer};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Table {
    size: [f64; 2],
    pixel_ratio: f64,
    is_bind_to_grid: bool,
    drawing_texture: TexstureLayer,
    drawing_texture_is_changed: bool,
    measure_texture: TexstureLayer,
    measure_texture_is_changed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TableData {
    pub size: [f64; 2],
    pub is_bind_to_grid: bool,
    pub drawing_texture: String,
    pub pixel_ratio: f64,
}

impl Table {
    pub fn new(size: [f64; 2], pixel_ratio: f64) -> Self {
        let texture_width = (size[0] * pixel_ratio) as u32;
        let texture_height = (size[1] * pixel_ratio) as u32;
        Self {
            size,
            pixel_ratio,
            is_bind_to_grid: false,
            drawing_texture: TexstureLayer::new(&[texture_width, texture_height]),
            drawing_texture_is_changed: true,
            measure_texture: TexstureLayer::new(&[texture_width, texture_height]),
            measure_texture_is_changed: true,
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn set_is_bind_to_grid(&mut self, is_bind_to_grid: bool) {
        self.is_bind_to_grid = is_bind_to_grid;
    }

    pub fn is_bind_to_grid(&self) -> bool {
        self.is_bind_to_grid
    }

    pub fn drawing_texture_element(&self) -> Option<&web_sys::HtmlCanvasElement> {
        if self.drawing_texture_is_changed {
            Some(self.drawing_texture.element())
        } else {
            None
        }
    }

    pub fn measure_texture_element(&self) -> Option<&web_sys::HtmlCanvasElement> {
        if self.measure_texture_is_changed {
            Some(self.measure_texture.element())
        } else {
            None
        }
    }

    pub fn rendered(&mut self) {
        self.clear_measure();
        self.drawing_texture_is_changed = false;
        self.measure_texture_is_changed = false;
    }

    fn get_texture_position(&self, position: &[f64; 2]) -> [f64; 2] {
        let p = position;
        let p = if self.is_bind_to_grid {
            [(p[0] * 2.0).round() / 2.0, (p[1] * 2.0).round() / 2.0]
        } else {
            [p[0], p[1]]
        };

        [
            (p[0] + self.size[0] / 2.0) * self.pixel_ratio,
            (p[1] + self.size[1] / 2.0) * self.pixel_ratio,
        ]
    }

    pub fn draw_cursor(
        &self,
        position: &[f64; 2],
        _radius: f64,
        _outer_color: Color,
        _inner_color: Color,
    ) {
        let _context = self.drawing_texture.context();

        let [_px, _py] = self.get_texture_position(position);
    }

    pub fn draw_line(&mut self, begin: &[f64; 2], end: &[f64; 2], color: Color, line_width: f64) {
        let context = self.drawing_texture.context();

        let [bx, by] = self.get_texture_position(begin);
        let [ex, ey] = self.get_texture_position(end);

        context.set_line_width(line_width * self.pixel_ratio);
        context.set_line_cap("round");
        context.set_stroke_style(&color.to_jsvalue());
        context
            .set_global_composite_operation("source-over")
            .unwrap();
        context.begin_path();
        context.move_to(bx, by);
        context.line_to(ex, ey);
        context.fill();
        context.stroke();

        self.drawing_texture_is_changed = true;
    }

    pub fn erace_line(&mut self, begin: &[f64; 2], end: &[f64; 2], line_width: f64) {
        let context = self.drawing_texture.context();

        let [bx, by] = self.get_texture_position(begin);
        let [ex, ey] = self.get_texture_position(end);

        context
            .set_global_composite_operation("destination-out")
            .unwrap();
        context.set_line_width(line_width * self.pixel_ratio);
        context.set_line_cap("round");
        context.begin_path();
        context.move_to(bx, by);
        context.line_to(ex, ey);
        context.fill();
        context.stroke();
        context
            .set_global_composite_operation("source-over")
            .unwrap();

        self.drawing_texture_is_changed = true;
    }

    pub fn draw_measure(
        &mut self,
        begin: &[f64; 2],
        end: &[f64; 2],
        color: Color,
        line_width: f64,
    ) -> f64 {
        let context = self.measure_texture.context();

        let [bx, by] = self.get_texture_position(begin);
        let [ex, ey] = self.get_texture_position(end);

        let radious = ((ex - bx).powi(2) + (ey - by).powi(2)).sqrt();

        context.set_line_width(line_width);
        context.set_line_cap("round");
        context.set_stroke_style(&color.to_jsvalue());
        context.set_fill_style(&JsValue::from("transparent"));
        context
            .set_global_composite_operation("source-over")
            .expect("");
        context.begin_path();
        context.move_to(bx, by);
        context.line_to(ex, ey);
        context.move_to(bx + radious, by);
        let _ = context.arc(bx, by, radious, 0.0, 2.0 * std::f64::consts::PI);
        context.fill();
        context.stroke();

        self.measure_texture_is_changed = true;

        radious / self.pixel_ratio
    }

    pub fn clear_measure(&mut self) {
        let context = self.measure_texture.context();

        context.clear_rect(
            0.0,
            0.0,
            self.size[0] * self.pixel_ratio,
            self.size[1] * self.pixel_ratio,
        );

        self.measure_texture_is_changed = true;
    }

    pub fn to_data(&self) -> TableData {
        TableData {
            size: self.size.clone(),
            is_bind_to_grid: self.is_bind_to_grid.clone(),
            drawing_texture: self.drawing_texture.element().to_data_url().unwrap(),
            pixel_ratio: self.pixel_ratio,
        }
    }
}

impl From<TableData> for Rc<Table> {
    fn from(table_data: TableData) -> Self {
        let size = table_data.size;
        let pixel_ratio = table_data.pixel_ratio;
        let texture_width = (size[0] * pixel_ratio) as u32;
        let texture_height = (size[1] * pixel_ratio) as u32;
        let table = Rc::new(Table {
            size: size,
            pixel_ratio: pixel_ratio,
            is_bind_to_grid: table_data.is_bind_to_grid,
            drawing_texture: TexstureLayer::new(&[texture_width, texture_height]),
            drawing_texture_is_changed: true,
            measure_texture: TexstureLayer::new(&[texture_width, texture_height]),
            measure_texture_is_changed: true,
        });

        table
    }
}
