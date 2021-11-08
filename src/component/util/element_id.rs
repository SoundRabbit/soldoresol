macro_rules! ElementId {
    {$($name:ident),*} => {
        struct ElementId {
            $($name: String,)*
        }

        impl ElementId {
            fn new() -> Self {
                Self {
                    $($name: crate::libs::random_id::u32val().to_string(),)*
                }
            }
        }
    };
}
