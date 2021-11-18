#[macro_use]
mod util;

pub mod block;
pub mod resource;
pub mod user;

arena! {
    pub block::Boxblock;
    pub block::Chat;
    pub block::ChatChannel;
    pub block::ChatMessage;
    pub block::Character;
    pub block::Craftboard;
    pub block::Scene;
    pub block::Table;
    pub block::World;
    pub resource::ImageData;
    pub user::Player;
}
