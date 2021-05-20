use std::rc::Rc;

pub type Gen<T> = Rc<dyn Fn() -> T>;

macro_rules! gen {
    {$value:expr} => {{
        Box::new(move || $value) as Box<dyn Fn>
    }};
    {$($env:stmt);* ; $value:expr} => {{
        $($env;)*
        gen!($value)
    }};
}
