use super::color::Color;
use super::TexstureLayerCollection;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const PEN_LAYER: usize = 0;
const GRID_LAYER: usize = 1;
const MEASURE_LAYER: usize = 2;
const CURSOR_LAYER: usize = 3;

pub struct Table {
    size: [f64; 2],
    pixel_ratio: f64,
    is_bind_to_grid: bool,
    layers: TexstureLayerCollection,
}

impl Table {
    pub fn new(size: [f64; 2], pixel_ratio: f64) -> Self {
        let texture_width = (size[0] * pixel_ratio) as u32;
        let texture_height = (size[1] * pixel_ratio) as u32;
        let mut layers =
            TexstureLayerCollection::new(&[texture_width, texture_height], &[2048, 2048], 4);
        layers.set_background_color(Color::from([255, 255, 255, 255]));
        let mut table = Self {
            size,
            pixel_ratio,
            is_bind_to_grid: false,
            layers,
        };
        table.draw_grid();
        table
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
        self.draw_grid();
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

    pub fn texture_element(&mut self) -> &web_sys::HtmlCanvasElement {
        self.layers.to_element()
    }

    pub fn flip(&self) {
        let texture_width = self.size[0] * self.pixel_ratio;
        let texture_height = self.size[1] * self.pixel_ratio;
        self.layers
            .context(MEASURE_LAYER)
            .clear_rect(0.0, 0.0, texture_width, texture_height);
        self.layers
            .context(CURSOR_LAYER)
            .clear_rect(0.0, 0.0, texture_width, texture_height);
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

    fn draw_grid(&mut self) {
        let texture_width = (self.size[0] * self.pixel_ratio) as u32;
        let texture_height = (self.size[1] * self.pixel_ratio) as u32;

        self.layers.set_size(&[texture_width, texture_height]);

        let context = self.layers.context(GRID_LAYER);

        context.clear_rect(0.0, 0.0, texture_width as f64, texture_height as f64);

        context.begin_path();

        for x in 0..(self.size[0] as u32 + 1) {
            let x = x as f64 * self.pixel_ratio;
            context.move_to(x, 0.0);
            context.line_to(x, texture_height as f64);
        }

        for y in 0..(self.size[1] as u32 + 1) {
            let y = y as f64 * self.pixel_ratio;
            context.move_to(0.0, y);
            context.line_to(texture_width as f64, y);
        }

        context.stroke();
    }

    pub fn draw_cursor(
        &self,
        position: &[f64; 2],
        radius: f64,
        outer_color: Color,
        inner_color: Color,
    ) {
        let context = self.layers.context(CURSOR_LAYER);

        let [px, py] = self.get_texture_position(position);

        let radious = radius * self.pixel_ratio;

        context.set_stroke_style(&outer_color.to_jsvalue());
        context.set_fill_style(&inner_color.to_jsvalue());
        context.set_line_width(self.pixel_ratio / 16.0);
        context.set_line_cap("round");
        context.begin_path();
        context
            .arc(px, py, radious, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        context.stroke();
        context.fill();
    }

    pub fn draw_line(&self, begin: &[f64; 2], end: &[f64; 2], color: Color, line_width: f64) {
        let context = self.layers.context(PEN_LAYER);

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
    }

    pub fn erace_line(&self, begin: &[f64; 2], end: &[f64; 2], line_width: f64) {
        let context = self.layers.context(PEN_LAYER);

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
    }

    pub fn draw_measure(&self, begin: &[f64; 2], end: &[f64; 2], color: Color, line_width: f64) {
        let context = self.layers.context(MEASURE_LAYER);

        let [bx, by] = self.get_texture_position(begin);
        let [ex, ey] = self.get_texture_position(end);

        let radious = ((ex - bx).powi(2) + (ey - by).powi(2)).sqrt();

        context.set_line_width(8.0);
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
        context.arc(bx, by, radious, 0.0, 2.0 * std::f64::consts::PI);
        context.fill();
        context.stroke();
    }
}
