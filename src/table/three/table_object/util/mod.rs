use crate::libs::three;

pub mod block_geometry;
pub mod nameplate;
pub mod rounded_rectangle;
pub use nameplate::Nameplate;
pub use rounded_rectangle::{
    BasicRoundedRectangleGeometry, CornerRoundedRectangleGeometry, RoundedRectangleGeometry,
    RoundedRectangleMesh,
};

pub fn stand<T>(geometry: &T, offset_z: f32)
where
    T: std::ops::Deref<Target = three::BufferGeometry>,
{
    let a_position = geometry
        .get_attribute("position")
        .array_as_f32array()
        .to_vec();

    let a_normal = geometry
        .get_attribute("normal")
        .array_as_f32array()
        .to_vec();

    let mut normal = vec![];
    let mut position = vec![];

    for i in 0..(a_position.len() / 3) {
        let p_x = a_position[i * 3];
        let p_y = a_position[i * 3 + 1];
        let p_z = a_position[i * 3 + 2];

        let n_x = a_normal[i * 3];
        let n_y = a_normal[i * 3 + 1];
        let n_z = a_normal[i * 3 + 2];

        position.push(p_x);
        position.push(-p_z);
        position.push(p_y + offset_z);

        normal.push(n_x);
        normal.push(-n_z);
        normal.push(n_y + offset_z);
    }

    geometry.set_attribute(
        "position",
        &three::BufferAttribute::new_with_f32array(
            &js_sys::Float32Array::from(position.as_slice()),
            3,
            false,
        ),
    );

    geometry.set_attribute(
        "normal",
        &three::BufferAttribute::new_with_f32array(
            &js_sys::Float32Array::from(normal.as_slice()),
            3,
            false,
        ),
    );
}
