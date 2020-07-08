type Resolve<T> = Box<dyn FnOnce(Option<T>) -> ()>;
type Task<T> = Box<dyn FnOnce(Resolve<T>) -> ()>;

pub struct Promise<T: 'static> {
    task: Task<T>,
}

impl<T> Promise<T> {
    pub fn new(task: impl FnOnce(Resolve<T>) -> () + 'static) -> Self {
        Self {
            task: Box::new(task),
        }
    }

    pub fn then(self, resolve: impl FnOnce(Option<T>) -> () + 'static) {
        (self.task)(Box::new(resolve))
    }

    pub fn map<U: 'static>(self, f: impl FnOnce(Option<T>) -> Option<U> + 'static) -> Promise<U> {
        Promise::new(move |resolve| {
            self.then(|result| resolve(f(result)));
        })
    }
    pub fn and_then<U: 'static>(
        self,
        f: impl FnOnce(Option<T>) -> Promise<U> + 'static,
    ) -> Promise<U> {
        Promise::new(move |resolve| {
            self.then(|result| f(result).then(resolve));
        })
    }
}

impl<T> Promise<T> {
    pub fn some(tasks: Vec<Self>) -> Promise<Vec<Option<T>>> {
        Promise::new(move |resolve| {
            use std::{cell::RefCell, rc::Rc};
            let results = Rc::new(RefCell::new(vec![]));
            let mut resolve = Rc::new(Some(Box::new(resolve)));
            let mut idx = 0;
            for task in tasks {
                results.borrow_mut().push(None);
                task.then({
                    let mut resolve = Rc::clone(&resolve);
                    let results = Rc::clone(&results);
                    move |result| {
                        results.borrow_mut()[idx] = result;
                        if let Some(resolve) = Rc::get_mut(&mut resolve).and_then(|r| r.take()) {
                            resolve(Some(results.borrow_mut().drain(..).collect()));
                        }
                    }
                });
                idx += 1;
            }
            if let Some(resolve) = Rc::get_mut(&mut resolve).and_then(|r| r.take()) {
                resolve(Some(results.borrow_mut().drain(..).collect()));
            }
        })
    }
}
