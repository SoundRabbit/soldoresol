use super::Color;

pub fn gray(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xfa, 0xfb, 0xfc, alpha]),
        1 => Color::from([0xfa, 0xfb, 0xfc, alpha]),
        2 => Color::from([0xf6, 0xf8, 0xfa, alpha]),
        3 => Color::from([0xd1, 0xd5, 0xda, alpha]),
        4 => Color::from([0x95, 0x9d, 0xa5, alpha]),
        5 => Color::from([0x6a, 0x73, 0x7d, alpha]),
        6 => Color::from([0x58, 0x60, 0x69, alpha]),
        7 => Color::from([0x44, 0x4d, 0x56, alpha]),
        8 => Color::from([0x2f, 0x36, 0x3d, alpha]),
        9 => Color::from([0x24, 0x29, 0x2e, alpha]),
        _ => gray(alpha, 5),
    }
}

pub fn blue(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xf1, 0xf8, 0xff, alpha]),
        1 => Color::from([0xdb, 0xed, 0xff, alpha]),
        2 => Color::from([0xc8, 0xe1, 0xff, alpha]),
        3 => Color::from([0x79, 0xb8, 0xff, alpha]),
        4 => Color::from([0x21, 0x88, 0xff, alpha]),
        5 => Color::from([0x03, 0x66, 0xd6, alpha]),
        6 => Color::from([0x00, 0x5c, 0xc5, alpha]),
        7 => Color::from([0x04, 0x42, 0x89, alpha]),
        8 => Color::from([0x03, 0x2f, 0x62, alpha]),
        9 => Color::from([0x05, 0x26, 0x4c, alpha]),
        _ => blue(alpha, 5),
    }
}

pub fn green(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xf0, 0xff, 0xf4, alpha]),
        1 => Color::from([0xdc, 0xff, 0xe4, alpha]),
        2 => Color::from([0xbe, 0xf5, 0xcb, alpha]),
        3 => Color::from([0x85, 0xe8, 0x9d, alpha]),
        4 => Color::from([0x34, 0xd0, 0x58, alpha]),
        5 => Color::from([0x28, 0xa7, 0x45, alpha]),
        6 => Color::from([0x22, 0x86, 0x3a, alpha]),
        7 => Color::from([0x17, 0x6f, 0x2c, alpha]),
        8 => Color::from([0x16, 0x5c, 0x26, alpha]),
        9 => Color::from([0x14, 0x46, 0x20, alpha]),
        _ => green(alpha, 5),
    }
}

pub fn purple(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xf5, 0xf0, 0xff, alpha]),
        1 => Color::from([0xe6, 0xdc, 0xfd, alpha]),
        2 => Color::from([0xd1, 0xbc, 0xf9, alpha]),
        3 => Color::from([0xb3, 0x92, 0xf0, alpha]),
        4 => Color::from([0x8a, 0x63, 0xd2, alpha]),
        5 => Color::from([0x6f, 0x42, 0xc1, alpha]),
        6 => Color::from([0x5a, 0x32, 0xa3, alpha]),
        7 => Color::from([0x4c, 0x28, 0x89, alpha]),
        8 => Color::from([0x3a, 0x1d, 0x6e, alpha]),
        9 => Color::from([0x29, 0x13, 0x4e, alpha]),
        _ => purple(alpha, 5),
    }
}

pub fn yellow(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xff, 0xfd, 0xef, alpha]),
        1 => Color::from([0xff, 0xfb, 0xdd, alpha]),
        2 => Color::from([0xff, 0xf5, 0xb1, alpha]),
        3 => Color::from([0xff, 0xea, 0x7f, alpha]),
        4 => Color::from([0xff, 0xdf, 0x5d, alpha]),
        5 => Color::from([0xff, 0xd3, 0x3d, alpha]),
        6 => Color::from([0xf9, 0xc5, 0x13, alpha]),
        7 => Color::from([0xdb, 0xab, 0x09, alpha]),
        8 => Color::from([0xb0, 0x88, 0x00, alpha]),
        9 => Color::from([0x73, 0x5c, 0x0f, alpha]),
        _ => yellow(alpha, 5),
    }
}

pub fn orange(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xff, 0xf8, 0xf2, alpha]),
        1 => Color::from([0xff, 0xeb, 0xda, alpha]),
        2 => Color::from([0xff, 0xd1, 0xac, alpha]),
        3 => Color::from([0xff, 0xab, 0x70, alpha]),
        4 => Color::from([0xfb, 0x85, 0x32, alpha]),
        5 => Color::from([0xf6, 0x6a, 0x0a, alpha]),
        6 => Color::from([0xe3, 0x62, 0x09, alpha]),
        7 => Color::from([0xd1, 0x57, 0x04, alpha]),
        8 => Color::from([0xc2, 0x4e, 0x00, alpha]),
        9 => Color::from([0xa0, 0x41, 0x00, alpha]),
        _ => orange(alpha, 5),
    }
}

pub fn red(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xff, 0xee, 0xf0, alpha]),
        1 => Color::from([0xff, 0xdc, 0xe0, alpha]),
        2 => Color::from([0xfd, 0xae, 0xb7, alpha]),
        3 => Color::from([0xf9, 0x75, 0x83, alpha]),
        4 => Color::from([0xea, 0x4a, 0x5a, alpha]),
        5 => Color::from([0xd7, 0x3a, 0x49, alpha]),
        6 => Color::from([0xcb, 0x24, 0x31, alpha]),
        7 => Color::from([0xb3, 0x1d, 0x28, alpha]),
        8 => Color::from([0x9e, 0x1c, 0x23, alpha]),
        9 => Color::from([0x86, 0x18, 0x1d, alpha]),
        _ => red(alpha, 5),
    }
}

pub fn pink(alpha: u8, idx: usize) -> Color {
    match idx {
        0 => Color::from([0xff, 0xee, 0xf8, alpha]),
        1 => Color::from([0xfe, 0xdb, 0xf0, alpha]),
        2 => Color::from([0xf9, 0xb3, 0xdd, alpha]),
        3 => Color::from([0xf6, 0x92, 0xce, alpha]),
        4 => Color::from([0xec, 0x6c, 0xb9, alpha]),
        5 => Color::from([0xea, 0x4a, 0xaa, alpha]),
        6 => Color::from([0xd0, 0x35, 0x92, alpha]),
        7 => Color::from([0xb9, 0x3a, 0x86, alpha]),
        8 => Color::from([0x99, 0x30, 0x6f, alpha]),
        9 => Color::from([0x6d, 0x22, 0x4f, alpha]),
        _ => pink(alpha, 5),
    }
}
