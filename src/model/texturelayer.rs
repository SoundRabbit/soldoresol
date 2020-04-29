use super::color::Color;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct TexstureLayer {
    element: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

pub struct TexstureLayerCollection {
    layers: Vec<TexstureLayer>,
    integrated: TexstureLayer,
    background_color: JsValue,
}

impl TexstureLayer {
    pub fn new(size: &[u32; 2]) -> Self {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        element.set_width(size[0]);
        element.set_height(size[1]);
        let context = Self::get_context2d_from_canvas(&element);
        Self { element, context }
    }

    fn get_context2d_from_canvas(
        canvas: &web_sys::HtmlCanvasElement,
    ) -> web_sys::CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }

    pub fn reset_context(&mut self) {
        self.context = Self::get_context2d_from_canvas(&self.element);
    }

    pub fn set_size(&mut self, size: &[u32; 2]) {
        self.element.set_width(size[0]);
        self.element.set_height(size[1]);
        self.reset_context();
    }

    pub fn element(&self) -> &web_sys::HtmlCanvasElement {
        &self.element
    }

    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }
}

impl TexstureLayerCollection {
    pub fn new(size: &[u32; 2], texture_size: &[u32; 2], layer_num: usize) -> Self {
        let mut layers = Vec::new();
        for _ in 0..layer_num {
            layers.push(TexstureLayer::new(size));
        }
        Self {
            layers: layers,
            integrated: TexstureLayer::new(texture_size),
            background_color: Color::from([0.0, 0.0, 0.0, 0.0]).to_jsvalue(),
        }
    }

    pub fn set_size(&mut self, size: &[u32; 2]) {
        for layer in &mut self.layers {
            layer.set_size(size);
        }
        self.integrated.set_size(&[2048, 2048]);
    }

    pub fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color.to_jsvalue();
    }

    pub fn element(&self, layer: usize) -> &web_sys::HtmlCanvasElement {
        self.layers[layer].element()
    }

    pub fn context(&self, layer: usize) -> &web_sys::CanvasRenderingContext2d {
        self.layers[layer].context()
    }

    pub fn to_element(&mut self) -> &web_sys::HtmlCanvasElement {
        self.integrated.reset_context();

        let element = self.integrated.element();
        let width = element.width() as f64;
        let height = element.height() as f64;
        let context = self.integrated.context();

        context
            .set_global_composite_operation("source-over")
            .unwrap();
        context.begin_path();

        context.set_fill_style(&self.background_color);
        context.fill_rect(0.0, 0.0, width, height);
        for layer in &self.layers {
            let layer_element = layer.element();
            context
                .draw_image_with_html_canvas_element_and_dw_and_dh(
                    layer_element,
                    0.0,
                    0.0,
                    width,
                    height,
                )
                .unwrap();
        }

        context.fill();
        context.stroke();

        element
    }
}
