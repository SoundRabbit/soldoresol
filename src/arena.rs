pub mod block;
pub mod player;
pub mod resource;

pub trait Insert<T> {
    type Id;
    fn insert(&mut self, block: T) -> Self::Id;
}
