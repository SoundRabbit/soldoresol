#[macro_use]
mod util;

pub mod block;
pub mod component;
pub mod resource;
pub mod user;

#[allow(unused_imports)]
use util::prelude::*;

pub use util::{Pack, PackDepth};

arena! {
    pub block::Boxblock;
    pub block::CanvasTexture;
    pub block::Character;
    pub block::Chat;
    pub block::ChatChannel;
    pub block::ChatMessage;
    pub block::Craftboard;
    pub block::LayerGroup;
    pub block::Property;
    pub block::Scene;
    pub block::Table;
    pub block::TerranTexture;
    pub block::Terran;
    pub block::Textboard;
    pub block::World;
    pub component::BoxblockComponent;
    pub component::CraftboardComponent;
    pub component::TextboardComponent;
    pub resource::ImageData;
    pub resource::BlockTexture;
    pub user::Player;
}
