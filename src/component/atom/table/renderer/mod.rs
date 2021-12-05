use std::rc::Rc;

macro_rules! as_f32a {
    [$($v:expr),*$(,)?] => {
        [$(($v) as f32,)*]
    };
}

macro_rules! as_f64a {
    [$($v:expr),*$(,)?] => {
        [$(($v) as f64,)*]
    };
}

mod framebuffer;
mod libs;
mod mesh;

mod assist;
pub mod new;
pub mod render;
pub mod util;

use libs::*;

use id_table::IdTable;
use tex_table::TexTable;
use webgl::WebGlRenderingContext;

pub use id_table::{IdColor, ObjectId, Surface};
pub use matrix::camera::CameraMatrix;

pub struct Renderer {
    canvas: Rc<web_sys::HtmlCanvasElement>,
    gl: WebGlRenderingContext,

    canvas_size: [f64; 2],
    device_pixel_ratio: f64,

    tex_table: TexTable,
    id_table: IdTable,

    view_frame: framebuffer::View,
    screen_frame: framebuffer::Screen,
    idmap_frame: framebuffer::Idmap,
    craftboard_idmap_frame: framebuffer::Idmap,
    shadomap_frame: framebuffer::Shadowmap,

    screen_mesh: mesh::Screen,
    craftboard_grid_mesh: mesh::CraftboardGrid,
    boxblock_mesh: mesh::Boxblock,
    nameplate_mesh: mesh::Nameplate,
    character_mesh: mesh::Character,
    character_base_mesh: mesh::CharacterBase,
}
