use crate::arena::{block, BlockMut, BlockRef};
use crate::libs::js_object::Object;
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub mod camera;
pub mod raycaster;
pub mod table_object;

pub use camera::Camera;
pub use raycaster::Raycaster;

pub struct Three {
    canvas: Rc<web_sys::HtmlCanvasElement>,
    camera: Camera,
    raycaster: Raycaster,
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
        (&camera as &three::Camera).rotation().set_order("ZXY");

        let raycaster = Raycaster::new();

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
            raycaster,
            scene,
            renderer,
            object_boxblock: table_object::Boxblock::new(),
            ambient_light,
            device_pixel_ratio,
            canvas_size: [1.0, 1.0],
        }
    }

    fn coords(&self, coords_px: &[f64; 2]) -> [f64; 2] {
        let coord_x = coords_px[0] * 2.0 / self.canvas_size[0] - 1.0;
        let coord_y = coords_px[1] * 2.0 / self.canvas_size[1] - 1.0;
        let coord_y = -coord_y;
        [coord_x, coord_y]
    }

    fn intersect_objects(&mut self, coords: &[f64; 2], ignored_id: &U128Id) -> Vec<Object> {
        self.raycaster.set_from_camera(coords, &self.camera);
        self.raycaster
            .intersect_objects(&self.scene.children())
            .to_vec()
            .into_iter()
            .filter_map(|object| object.dyn_into::<Object>().ok())
            .filter(|object| {
                object
                    .get("object")
                    .and_then(|object| object.dyn_into::<three::Object3D>().ok())
                    .and_then(|object| U128Id::from_jsvalue(&object.user_data()))
                    .map(|block_id| block_id != *ignored_id)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn get_focused_object(&mut self, coords_px: &[f64; 2], ignored_id: &U128Id) -> U128Id {
        let coords = self.coords(coords_px);
        let objects = self.intersect_objects(&coords, ignored_id);

        for object in objects {
            let block_id = object
                .get("object")
                .and_then(|x| x.dyn_into::<three::Object3D>().ok())
                .and_then(|x| U128Id::from_jsvalue(&x.user_data()));

            if let Some(block_id) = block_id {
                return block_id;
            }
        }

        U128Id::none()
    }

    pub fn get_focused_position(
        &mut self,
        coords_px: &[f64; 2],
        ignored_id: &U128Id,
    ) -> ([f64; 3], [f64; 3]) {
        let coords = self.coords(coords_px);
        let objects = self.intersect_objects(&coords, ignored_id);

        for object in objects {
            let point = object
                .get("point")
                .and_then(|x| x.dyn_into::<three::Vector3>().ok());
            let face = object
                .get("face")
                .and_then(|x| x.dyn_into::<three::Vector3>().ok());

            if let Some((point, face)) = join_some!(point, face) {
                return (
                    [point.x(), point.y(), point.z()],
                    [face.x(), face.y(), face.z()],
                );
            }
        }

        let ray = self.raycaster.ray();
        let origin = ray.origin();
        let direction = ray.direction();

        let scale = -origin.z() / direction.z();
        let x = origin.x() + direction.x() * scale;
        let y = origin.y() + direction.y() * scale;
        let z = 0.0;
        ([x, y, z], [0.0, 0.0, 1.0])
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

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn render(&mut self, world: BlockRef<block::World>) {
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

        self.camera
            .set_aspect(self.canvas_size[0] / self.canvas_size[1]);

        self.renderer.render(&self.scene, &self.camera);
    }
}