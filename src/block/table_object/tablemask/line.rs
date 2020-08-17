use crate::Color;

#[derive(Clone)]
pub struct Line {
    org: [f32; 2],
    vec: [f32; 2],
    color: Color,
    is_inved: bool,
    is_fixed: bool,
}
