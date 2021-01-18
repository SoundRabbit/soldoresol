pub trait TryRef<T> {
    fn try_ref(&self) -> Option<&T>;
}

pub trait TryMut<T> {
    fn try_mut(&mut self) -> Option<&mut T>;
}
