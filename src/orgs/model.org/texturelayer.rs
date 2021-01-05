use wasm_bindgen::JsCast;

pub struct TexstureLayer {
    element: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

#[allow(dead_code)]
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
