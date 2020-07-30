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

    pub fn none() -> Self {
        Self::new(|resolve| resolve(None))
    }

    pub fn some(v: T) -> Self {
        Self::new(move |resolve| resolve(Some(v)))
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
    pub fn all(tasks: Vec<Self>) -> Promise<Vec<T>> {
        let mut promise = Promise::some(vec![]);
        for task in tasks {
            promise = promise.and_then(move |x| match x {
                Some(mut xs) => task.map(move |y| {
                    y.map(|y| {
                        xs.push(y);
                        xs
                    })
                }),
                None => Promise::none(),
            });
        }
        promise
    }
}
