use super::*;

impl Renderer {
    pub fn reset_size(&mut self) {
        let canvas_size = Self::reset_canvas_size(&self.canvas, self.device_pixel_ratio);
        let sw = canvas_size[0] as i32;
        let sh = canvas_size[1] as i32;

        self.gl.viewport(0, 0, sw, sh);
        self.screen_frame
            .reset_size(&self.gl, sw, sh, &mut self.tex_table);
        self.idmap_frame
            .reset_size(&self.gl, sw, sh, &mut self.tex_table);
        self.craftboard_idmap_frame
            .reset_size(&self.gl, sw, sh, &mut self.tex_table);
        self.canvas_size = canvas_size;
    }

    pub fn get_object_id(&self, x: f64, y: f64) -> ObjectId {
        self.idmap_frame.bind_self(&self.gl);
        self.impl_get_object_id(x, y)
    }

    pub fn get_craftboard_object_id(&self, x: f64, y: f64) -> ObjectId {
        self.craftboard_idmap_frame.bind_self(&self.gl);
        self.impl_get_object_id(x, y)
    }

    pub fn get_focused_position(
        &self,
        camera: &CameraMatrix,
        x: f64,
        y: f64,
    ) -> ([f64; 3], [f64; 3]) {
        self.idmap_frame.bind_self(&self.gl);
        self.impl_get_focused_position(camera, x, y)
    }

    pub fn get_focused_craftboard_position(
        &self,
        camera: &CameraMatrix,
        x: f64,
        y: f64,
    ) -> ([f64; 3], [f64; 3]) {
        self.craftboard_idmap_frame.bind_self(&self.gl);
        self.impl_get_focused_position(camera, x, y)
    }

    fn impl_get_object_id(&self, x: f64, y: f64) -> ObjectId {
        let x = x * self.device_pixel_ratio;
        let y = self.canvas_size[1] - y * self.device_pixel_ratio;
        let mut table_id = [0, 0, 0, 0];
        let res = self.gl.read_pixels_with_opt_u8_array(
            x as i32,
            y as i32,
            1,
            1,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            Some(&mut table_id),
        );

        if res.is_ok() {
            let r = table_id[0] as f32 / 255.0;
            let g = table_id[1] as f32 / 255.0;
            let b = table_id[2] as f32 / 255.0;
            let a = table_id[3] as f32 / 255.0;
            let table_id = (r + g / 255.0 + b / (255.0 * 255.0) + a / (255.0 * 255.0 * 255.0))
                * (0x1000000 as f32 - 1.0);
            let table_id = ((table_id / 2.0).round() * 2.0) as u32;
            self.id_table
                .object_id(&IdColor::from(table_id))
                .map(|x| ObjectId::clone(x))
                .unwrap_or(ObjectId::None)
        } else {
            ObjectId::None
        }
    }

    fn impl_get_focused_position(
        &self,
        camera: &CameraMatrix,
        x: f64,
        y: f64,
    ) -> ([f64; 3], [f64; 3]) {
        let focused = self.impl_get_object_id(x, y);

        let x = x * self.device_pixel_ratio;
        let y = y * self.device_pixel_ratio;

        let cs = [self.canvas_size[0] as f32, self.canvas_size[1] as f32];
        let sp = [x as f32, y as f32];

        match focused {
            ObjectId::Character(_, srfs) => {
                let (r, s, t) = Self::rst(&srfs);
                let cp = camera.collision_point(&cs, &sp, &r, &s, &t);
                (as_f64a![cp[0], cp[1], cp[2]], srfs.n())
            }
            ObjectId::Boxblock(_, srfs) => {
                let (r, s, t) = Self::rst(&srfs);
                let cp = camera.collision_point(&cs, &sp, &r, &s, &t);
                (as_f64a![cp[0], cp[1], cp[2]], srfs.n())
            }
            ObjectId::Pointlight(_, srfs) => {
                let (r, s, t) = Self::rst(&srfs);
                let cp = camera.collision_point(&cs, &sp, &r, &s, &t);
                (as_f64a![cp[0], cp[1], cp[2]], srfs.n())
            }
            ObjectId::Terran(_, srfs) => {
                let (r, s, t) = Self::rst(&srfs);
                let cp = camera.collision_point(&cs, &sp, &r, &s, &t);
                (as_f64a![cp[0], cp[1], cp[2]], srfs.n())
            }
            ObjectId::Craftboard(_, srfs) => {
                let (r, s, t) = Self::rst(&srfs);
                let cp = camera.collision_point(&cs, &sp, &r, &s, &t);
                (as_f64a![cp[0], cp[1], cp[2]], srfs.n())
            }
            _ => {
                let p = camera.collision_point_on_xy_plane(&cs, &sp);
                (as_f64a![p[0], p[1], p[2]], [0.0, 0.0, 1.0])
            }
        }
    }

    fn rst(srfs: &Surface) -> ([f32; 3], [f32; 3], [f32; 3]) {
        let r = as_f32a![srfs.r[0], srfs.r[1], srfs.r[2]];
        let s = as_f32a![srfs.s[0], srfs.s[1], srfs.s[2]];
        let t = as_f32a![srfs.t[0], srfs.t[1], srfs.t[2]];
        let n = srfs;
        (r, s, t)
    }
}
