pub mod boxblock;
pub mod character;
pub mod craftboard;
pub mod terran;
pub mod textboard;
mod util;

pub use boxblock::Boxblock;
pub use character::Character;
pub use craftboard::Craftboard;
pub use terran::Terran;
pub use textboard::Textboard;

const ORDER_BOXBLOCK: f64 = 1.0;
const ORDER_CRAFTBOARD: f64 = 10.0;
const ORDER_CHARACTER: f64 = 100.0;
