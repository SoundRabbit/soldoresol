use super::CameraMatrix;
use crate::arena::{block, BlockMut, BlockRef};
use crate::libs::three;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub mod camera;
pub mod table_object;

pub use camera::Camera;

pub struct Three {
    canvas: Rc<web_sys::HtmlCanvasElement>,
    camera: Camera,
    scene: three::Scene,
    renderer: three::WebGLRenderer,
    object_boxblock: table_object::Boxblock,
    ambient_light: three::AmbientLight,
    device_pixel_ratio: f64,
    canvas_size: [f64; 2],
}

impl Three {
    pub fn new() -> Self {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let canvas = Rc::new(canvas);

        let camera = Camera::new();
        camera.rotation().set_order("ZXY");

        let scene = three::Scene::new();

        let renderer = three::WebGLRenderer::new(&object! {
            "canvas": canvas.as_ref()
        });

        let ambient_light = three::AmbientLight::new();
        scene.add(&ambient_light);

        let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio();

        Self {
            canvas,
            camera,
            scene,
            renderer,
            object_boxblock: table_object::Boxblock::new(),
            ambient_light,
            device_pixel_ratio,
            canvas_size: [1.0, 1.0],
        }
    }

    pub fn reset_size(&mut self) {
        let bb = self.canvas.get_bounding_client_rect();
        let w = bb.width();
        let h = bb.height();

        self.renderer.set_pixel_ratio(self.device_pixel_ratio);
        self.renderer.set_size(w, h);

        self.canvas_size = [w, h];
    }

    pub fn canvas(&self) -> Rc<web_sys::HtmlCanvasElement> {
        Rc::clone(&self.canvas)
    }

    pub fn render(&mut self, world: BlockRef<block::World>, camera_matrix: &CameraMatrix) {
        let table = world
            .map(|world| {
                let scene = world.selecting_scene().as_ref();
                let table = scene
                    .map(|scene: &block::Scene| scene.selecting_table().as_ref())
                    .unwrap_or(BlockRef::<block::Table>::none());

                table
            })
            .unwrap_or(BlockRef::<block::Table>::none());

        table.map(|table| {
            self.object_boxblock.update(
                &self.scene,
                table.boxblocks().iter().map(|block| block.as_ref()),
            );
        });

        let [cx, cy, cz] = camera_matrix.position();
        self.camera.position().set(cx as f64, cy as f64, cz as f64);

        self.camera
            .rotation()
            .set_x(camera_matrix.x_axis_rotation() as f64);
        self.camera
            .rotation()
            .set_z(camera_matrix.z_axis_rotation() as f64);

        self.camera
            .set_aspect(self.canvas_size[0] / self.canvas_size[1]);

        self.renderer.render(&self.scene, &self.camera);
    }
}
