#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Pack;

block! {
    [pub Textboard(constructor, pack)]
    (position): [f64; 3];
    name: String = String::new();
    text: String = String::new();
    font_size: f64 = 0.5;
    size: [f64; 2] = [3.0, 4.0];
}
