use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct State<T> {
    payload: Rc<RefCell<T>>,
}

pub struct Prop<T> {
    payload: Rc<RefCell<T>>,
}

impl<T> State<T> {
    pub fn new(payload: T) -> Self {
        Self {
            payload: Rc::new(RefCell::new(payload)),
        }
    }

    pub fn as_prop(&self) -> Prop<T> {
        let payload = Rc::clone(&self.payload);
        Prop { payload }
    }
}

impl<T> Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.payload.as_ptr().as_ref().unwrap() }
    }
}

impl<T> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.payload.as_ptr().as_mut().unwrap() }
    }
}

impl<T> Deref for Prop<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.payload.as_ptr().as_ref().unwrap() }
    }
}

impl<T> Clone for Prop<T> {
    fn clone(&self) -> Self {
        let payload = Rc::clone(&self.payload);
        Self { payload }
    }
}

impl<T> AsRef<T> for Prop<T> {
    fn as_ref(&self) -> &T {
        &self
    }
}
