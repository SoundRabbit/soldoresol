use super::{color::Color, TexstureLayer};
use crate::JsObject;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct Table {
    size: [f64; 2],
    pixel_ratio: [f64; 2],
    is_bind_to_grid: bool,
    drawing_texture: TexstureLayer,
    drawing_texture_is_changed: bool,
    measure_texture: TexstureLayer,
    measure_texture_is_changed: bool,
    image_texture_id: Option<u128>,
}

pub struct TableData {
    pub size: [f64; 2],
    pub is_bind_to_grid: bool,
    pub drawing_texture: String,
    pub image_texture_id: Option<u128>,
}

impl Table {
    pub fn new() -> Self {
        let texture_width = 4096;
        let texture_height = 4096;

        let size = [1.0, 1.0];
        let pixel_ratio = [1.0, 1.0];
        Self {
            size,
            pixel_ratio,
            is_bind_to_grid: false,
            drawing_texture: TexstureLayer::new(&[texture_width, texture_height]),
            drawing_texture_is_changed: true,
            measure_texture: TexstureLayer::new(&[texture_width, texture_height]),
            measure_texture_is_changed: true,
            image_texture_id: None,
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        let new_pixel_ratio = [4096.0 / size[0], 4096.0 / size[1]];

        let _ = self.drawing_texture.context().scale(
            new_pixel_ratio[0] / self.pixel_ratio[0],
            new_pixel_ratio[1] / self.pixel_ratio[1],
        );

        let _ = self.measure_texture.context().scale(
            new_pixel_ratio[0] / self.pixel_ratio[0],
            new_pixel_ratio[1] / self.pixel_ratio[1],
        );

        self.pixel_ratio = new_pixel_ratio;
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

    pub fn image_texture_id(&self) -> Option<&u128> {
        self.image_texture_id.as_ref()
    }

    pub fn set_image_texture_id(&mut self, data_id: u128) {
        self.image_texture_id = Some(data_id);
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

        [(p[0] + self.size[0] / 2.0), -(p[1] - self.size[1] / 2.0)]
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

        context.set_line_width(line_width);
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
        context.set_line_width(line_width);
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

        radious
    }

    pub fn clear_measure(&mut self) {
        let context = self.measure_texture.context();

        context.clear_rect(0.0, 0.0, self.size[0], self.size[1]);

        self.measure_texture_is_changed = true;
    }

    pub fn to_data(&self) -> TableData {
        TableData {
            size: self.size.clone(),
            is_bind_to_grid: self.is_bind_to_grid.clone(),
            drawing_texture: self.drawing_texture.element().to_data_url().unwrap(),
            image_texture_id: self.image_texture_id.clone(),
        }
    }
}

impl From<TableData> for Rc<Table> {
    fn from(table_data: TableData) -> Self {
        let size = table_data.size;
        let texture_width = 4096;
        let texture_height = 4096;

        let mut table = Table {
            size: [1.0, 1.0],
            pixel_ratio: [1.0, 1.0],
            is_bind_to_grid: table_data.is_bind_to_grid,
            drawing_texture: TexstureLayer::new(&[texture_width, texture_height]),
            drawing_texture_is_changed: true,
            measure_texture: TexstureLayer::new(&[texture_width, texture_height]),
            measure_texture_is_changed: true,
            image_texture_id: table_data.image_texture_id,
        };

        table.set_size(size);

        Rc::new(table)
    }
}

impl TableData {
    pub fn as_object(&self) -> JsObject {
        let is_bind_to_grid: bool = self.is_bind_to_grid;
        let drawing_texture = &self.drawing_texture;

        object! {
            size: array![self.size[0], self.size[1]],
            is_bind_to_grid: is_bind_to_grid,
            drawing_texture: drawing_texture,
            image_texture_id: self.image_texture_id.map(|x| x.to_string())
        }
    }
}

impl From<JsObject> for TableData {
    fn from(obj: JsObject) -> Self {
        use js_sys::Array;
        use wasm_bindgen::JsCast;

        let size = obj.get("size").unwrap().dyn_into::<Array>().ok().unwrap();
        let size = [size.get(0).as_f64().unwrap(), size.get(1).as_f64().unwrap()];

        let drawing_texture = obj.get("drawing_texture").unwrap().as_string().unwrap();
        let is_bind_to_grid = obj.get("is_bind_to_grid").unwrap().as_bool().unwrap();
        let image_texture_id = obj
            .get("image_texture_id")
            .and_then(|x| x.as_string())
            .and_then(|x| x.parse().ok());

        Self {
            size,
            drawing_texture,
            is_bind_to_grid,
            image_texture_id,
        }
    }
}
