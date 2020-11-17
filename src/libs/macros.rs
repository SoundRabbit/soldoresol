macro_rules! set {
    { $( $n:expr ),* } => {
        {
            use std::collections::HashSet;

            #[allow(unused_mut)]
            let mut tmp = HashSet::new();
            $(
                tmp.insert($n);
            )*
            tmp
        }
    };
}

macro_rules! join_some {
    ($($x:expr), *) => {{
        #[allow(unused_mut)]
        let mut all_is_some = true;

        $(
            if $x.is_none() {
                all_is_some = false;
            }
        )*

        if all_is_some {
            Some(($($x.unwrap()),*))
        } else {
            None
        }
    }};
}
