macro_rules! ElementId {
    {$($name:ident),*} => {
        #[allow(dead_code)]
        struct ElementId {
            $($name: String,)*
        }

        impl ElementId {
            #[allow(dead_code)]
            fn new() -> Self {
                Self {
                    $($name: crate::libs::random_id::u32val().to_string(),)*
                }
            }
        }
    };
}
