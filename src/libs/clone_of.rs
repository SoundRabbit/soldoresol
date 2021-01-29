pub trait CloneOf {
    fn clone_of(this: &Self) -> Self;
}

impl<T: Clone> CloneOf for T {
    fn clone_of(this: &Self) -> Self {
        Self::clone(this)
    }
}
