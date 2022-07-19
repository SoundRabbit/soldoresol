use crate::libs::three;

pub fn box_geometry(uv_map: &mut dyn FnMut([f32; 2]) -> [f32; 2]) -> three::BoxGeometry {
    let geometry = three::BoxGeometry::new(1.0, 1.0, 1.0);
    let a_position = geometry
        .get_attribute("position")
        .array_as_f32array()
        .to_vec();
    let a_normal = geometry
        .get_attribute("normal")
        .array_as_f32array()
        .to_vec();
    let mut uv = vec![];

    for i in 0..(a_position.len() / 3) {
        let p_x = a_position[i * 3];
        let p_y = a_position[i * 3 + 1];
        let p_z = a_position[i * 3 + 2];

        let n_x = a_normal[i * 3];
        let n_y = a_normal[i * 3 + 1];
        let n_z = a_normal[i * 3 + 2];

        let [u, v] = uv_map(match factor(n_x, n_y, n_z) {
            0 => box_texture_coord(0, &[p_y * 2.0, p_z * 2.0]),
            1 => box_texture_coord(1, &[-p_x * 2.0, p_z * 2.0]),
            2 => box_texture_coord(2, &[p_x * 2.0, p_y * 2.0]),
            3 => box_texture_coord(3, &[-p_y * 2.0, p_z * 2.0]),
            4 => box_texture_coord(4, &[p_x * 2.0, p_z * 2.0]),
            5 => box_texture_coord(5, &[p_x * 2.0, p_y * 2.0]),
            _ => unreachable!(),
        });

        uv.push(u);
        uv.push(v);
    }

    geometry.set_attribute(
        "uv",
        &three::BufferAttribute::new_with_f32array(
            &js_sys::Float32Array::from(uv.as_slice()),
            2,
            false,
        ),
    );

    geometry
}

pub fn cylinder_geometry(uv_map: &mut dyn FnMut([f32; 2]) -> [f32; 2]) -> three::CylinderGeometry {
    let geometry = three::CylinderGeometry::new(0.5, 0.5, 1.0, 48);

    let a_position = geometry
        .get_attribute("position")
        .array_as_f32array()
        .to_vec();
    let a_normal = geometry
        .get_attribute("normal")
        .array_as_f32array()
        .to_vec();
    let a_uv = geometry.get_attribute("uv").array_as_f32array().to_vec();
    let mut position = vec![];
    let mut normal = vec![];
    let mut uv = vec![];

    for i in 0..(a_position.len() / 3) {
        let p_x = a_position[i * 3];
        let p_y = a_position[i * 3 + 1];
        let p_z = a_position[i * 3 + 2];

        let n_x = a_normal[i * 3];
        let n_y = a_normal[i * 3 + 1];
        let n_z = a_normal[i * 3 + 2];

        let u = a_uv[i * 2];

        let [u, v] = uv_map(match factor(n_x, n_y, n_z) {
            0 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
            1 => box_texture_coord(2, &[p_x * 2.0, p_z * 2.0]),
            2 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
            3 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
            4 => box_texture_coord(5, &[p_x * 2.0, p_z * 2.0]),
            5 => [u, p_y * 0.5 + 0.25 + 0.5 / 2.0],
            _ => unreachable!(),
        });

        position.push(p_x);
        position.push(-p_z);
        position.push(p_y);

        normal.push(n_x);
        normal.push(-n_z);
        normal.push(n_y);

        uv.push(u);
        uv.push(v);
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

    geometry.set_attribute(
        "uv",
        &three::BufferAttribute::new_with_f32array(
            &js_sys::Float32Array::from(uv.as_slice()),
            2,
            false,
        ),
    );

    geometry
}

pub fn icosahedron_geometry(
    uv_map: &mut dyn FnMut([f32; 2]) -> [f32; 2],
) -> three::IcosahedronGeometry {
    let geometry = three::IcosahedronGeometry::new(0.5, 5);

    super::stand(&geometry, 0.0);

    let a_uv = geometry.get_attribute("uv").array_as_f32array().to_vec();
    let mut uv = vec![];

    for i in 0..(a_uv.len() / 2) {
        let u = a_uv[i * 2];
        let v = a_uv[i * 2 + 1];
        let [u, v] = uv_map([u, v]);
        uv.push(u);
        uv.push(v);
    }

    geometry.set_attribute(
        "uv",
        &three::BufferAttribute::new_with_f32array(
            &js_sys::Float32Array::from(uv.as_slice()),
            2,
            false,
        ),
    );

    geometry
}

pub fn slope_geometry(uv_map: &mut dyn FnMut([f32; 2]) -> [f32; 2]) -> three::BufferGeometry {
    let points = js_sys::Float32Array::from(
        [
            // PY
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            // NY
            [-0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            // NX
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            // NZ
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            // 斜面
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
        ]
        .concat()
        .as_slice(),
    );
    let normal = js_sys::Float32Array::from(
        [
            // PY
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // NY
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // NX
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // NZ
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            // 斜面
            [1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
        ]
        .concat()
        .as_slice(),
    );
    let uv = js_sys::Float32Array::from(
        [
            //PY
            uv_map(box_texture_coord(1, &[-1.0, -1.0])),
            uv_map(box_texture_coord(1, &[1.0, -1.0])),
            uv_map(box_texture_coord(1, &[1.0, 1.0])),
            //NY
            uv_map(box_texture_coord(4, &[-1.0, 1.0])),
            uv_map(box_texture_coord(4, &[-1.0, -1.0])),
            uv_map(box_texture_coord(4, &[1.0, -1.0])),
            // NX
            uv_map(box_texture_coord(3, &[1.0, -1.0])),
            uv_map(box_texture_coord(3, &[-1.0, -1.0])),
            uv_map(box_texture_coord(3, &[-1.0, 1.0])),
            uv_map(box_texture_coord(3, &[1.0, 1.0])),
            // NZ
            uv_map(box_texture_coord(3, &[1.0, 1.0])),
            uv_map(box_texture_coord(3, &[-1.0, 1.0])),
            uv_map(box_texture_coord(3, &[-1.0, -1.0])),
            uv_map(box_texture_coord(3, &[1.0, -1.0])),
            // 斜面
            uv_map(box_texture_coord(2, &[-1.0, 1.0])),
            uv_map(box_texture_coord(2, &[-1.0, -1.0])),
            uv_map(box_texture_coord(2, &[1.0, -1.0])),
            uv_map(box_texture_coord(2, &[1.0, 1.0])),
        ]
        .concat()
        .as_slice(),
    );
    let index = js_sys::Uint16Array::from(
        [
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8, 8, 9, 6],
            vec![10, 11, 12, 12, 13, 10],
            vec![14, 15, 16, 16, 17, 14],
        ]
        .concat()
        .as_slice(),
    );

    let geometry = three::BufferGeometry::new();

    geometry.set_attribute(
        "position",
        &three::BufferAttribute::new_with_f32array(&points, 3, false),
    );
    geometry.set_attribute(
        "normal",
        &three::BufferAttribute::new_with_f32array(&normal, 3, true),
    );
    geometry.set_attribute(
        "uv",
        &three::BufferAttribute::new_with_f32array(&uv, 2, false),
    );
    geometry.set_index(&three::BufferAttribute::new_with_u16array(&index, 1, false));

    geometry
}

pub fn factor(x: f32, y: f32, z: f32) -> usize {
    let ax = x.abs();
    let ay = y.abs();
    let az = z.abs();

    if ax > ay && ax > az {
        if x > 0.0 {
            0
        } else {
            3
        }
    } else if ay > ax && ay > az {
        if y > 0.0 {
            1
        } else {
            4
        }
    } else {
        if z > 0.0 {
            2
        } else {
            5
        }
    }
}

// 0: px
// 1: py
// 2: pz
// 3: nx
// 4: ny
// 5: nz
pub fn box_texture_coord(surface: usize, coord: &[f32; 2]) -> [f32; 2] {
    let [ox, oy] = match surface % 6 {
        0 => [0.25, 0.25],
        1 => [0.50, 0.25],
        2 => [0.00, 0.75],
        3 => [0.75, 0.25],
        4 => [0.00, 0.25],
        5 => [0.00, 0.00],
        _ => unreachable!(),
    };

    let [w, h] = match surface % 6 {
        0 => [0.25, 0.50],
        1 => [0.25, 0.50],
        2 => [1.00, 0.25],
        3 => [0.25, 0.50],
        4 => [0.25, 0.50],
        5 => [1.00, 0.25],
        _ => unreachable!(),
    };

    [
        coord[0].clamp(-1.0, 1.0) * (w / 2.0) + ox + (w / 2.0),
        coord[1].clamp(-1.0, 1.0) * (h / 2.0) + oy + (h / 2.0),
    ]
}
