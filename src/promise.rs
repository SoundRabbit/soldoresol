type Resolve<T, E> = Box<dyn FnOnce(Result<T, E>)>;
type Task<T, E> = Box<dyn FnOnce(Resolve<T, E>)>;

pub struct Promise<T: 'static, E: 'static> {
    task: Task<T, E>,
}

impl<T, E> Promise<T, E> {
    pub fn new(task: impl FnOnce(Resolve<T, E>) + 'static) -> Self {
        Self {
            task: Box::new(task),
        }
    }

    pub fn then(self, resolve: impl FnOnce(Result<T, E>) + 'static) {
        (self.task)(Box::new(resolve))
    }

    pub fn map<U, F>(
        self,
        f: impl FnOnce(Result<T, E>) -> Result<U, F> + 'static,
    ) -> Promise<U, F> {
        Promise::new(move |resolve| {
            self.then(|result| resolve(f(result)));
        })
    }
}

impl<T, E> Promise<T, E> {
    pub fn some(tasks: Vec<Self>) -> Promise<Vec<Option<T>>, ()> {
        Promise::new(move |resolve| {
            use std::{cell::RefCell, rc::Rc};
            let results = Rc::new(RefCell::new(vec![]));
            let mut resolve = Rc::new(Some(Box::new(resolve)));
            let mut idx = 0;
            for task in tasks {
                results.borrow_mut().push(None);
                task.then({
                    let mut resolve = Rc::clone(&resolve);
                    move |result| {
                        results.borrow_mut()[idx] = result.ok();
                        if let Some(resolve) = Rc::get_mut(&mut resolve).and_then(|r| r.take()) {
                            resolve(Ok(results.borrow_mut().drain(..).collect()));
                        }
                    }
                });
                idx += 1;
            }
            if let Some(resolve) = Rc::get_mut(&mut resolve).and_then(|r| r.take()) {
                resolve(Ok(results.borrow_mut().drain(..).collect()));
            }
        })
    }
}
