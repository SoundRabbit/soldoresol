macro_rules! mods {
    {
        $(pub $m:ident::$b:ident;)*
    } => {
        $(pub mod $m;)*
        $(pub use $m::$b;)*

        pub enum Resource {
            $($b(Rc<RefCell<$b>>),)*
        }

        impl std::hash::Hash for Resource {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                match self {
                    $(Self::$b(x) => {x.borrow().block_id().hash(state);})*
                }
            }
        }
    }
}
