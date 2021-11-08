#[macro_use]
mod util;

pub mod block;
pub mod user;

arena! {
    pub block::Chat;
    pub block::ChatChannel;
    pub block::ChatMessage;
    pub user::Player;
}
