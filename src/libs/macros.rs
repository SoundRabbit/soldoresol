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

macro_rules! map {
    { $( $n:ident : $v:expr ),* } => {
        {
            use std::collections::HashMap;

            #[allow(unused_mut)]
            let mut tmp = HashMap::new();
            $(
                tmp.insert($n, $v);
            )*
            tmp
        }
    };
}

macro_rules! join_some {
    ($x:expr; $($ys:expr), +) => {{
        if let Some(y) = $x {
            Some(($($ys,)+ y))
        } else {
            None
        }
    }};
    ($x:expr, $($xs:expr), + ; $($ys:expr), +) => {
        if let Some(y) = $x {
            join_some!($($xs),+ ; $($ys,)+ y)
        } else {
            None
        }
    };
    ($x:expr, $($xs:expr), +) => {
        if let Some(y) = $x {
            join_some!($($xs),+ ; y)
        } else {
            None
        }
    };
}

macro_rules! unwrap {
    ($x:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return None;
        }
    };
}

macro_rules! unwrap_or {
    ($x:expr ; $y:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return $y;
        }
    };
}

macro_rules! first_of {
    ($x:expr) => {
        if let Some(x) = $x {
            Some(x)
        } else {
            None
        }
    };

    ($x:expr,$($xs:expr),*) => {
        if let Some(x) = $x {
            Some(x)
        } else {
            first_of!($($xs:expr),*)
        }
    };
}
