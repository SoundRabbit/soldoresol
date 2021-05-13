pub struct Annotated<T, A> {
    pub value: T,
    pub annot: A,
}

impl<T: Clone, A: Clone> Clone for Annotated<T, A> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            annot: self.annot.clone(),
        }
    }
}
