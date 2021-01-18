pub mod message;
pub mod tab;

use super::BlockId;

pub struct Chat {
    tabs: Vec<BlockId>,
}
