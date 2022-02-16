#[macro_use]
mod util;

pub mod block;
pub mod resource;
pub mod user;

#[allow(unused_imports)]
use util::prelude::*;

arena! {
    pub block::Boxblock;
    pub block::CanvasTexture;
    pub block::Character;
    pub block::Chat;
    pub block::ChatChannel;
    pub block::ChatMessage;
    pub block::Craftboard;
    pub block::LayerGroup;
    pub block::Scene;
    pub block::Table;
    pub block::World;
    pub resource::ImageData;
    pub resource::BlockTexture;
    pub user::Player;
}
