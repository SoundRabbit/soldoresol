pub fn u8vec(len: usize) -> Vec<u8> {
    let mut id: Vec<u8> = Vec::new();
    id.resize(len, 0);
    web_sys::window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut id)
        .expect("");
    id
}

pub fn hex(len: usize) -> String {
    hex::encode(&u8vec(len))
}

pub fn base64url() -> String {
    base64::encode(&u8vec(3 * 5))
        .replace("+", r"@")
        .replace("/", r"#")
}

pub fn u128val() -> u128 {
    let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let id = u8vec(16);
    for i in 0..16 {
        buf[i] = id[i];
    }
    u128::from_be_bytes(buf)
}

#[allow(dead_code)]
pub fn u32val() -> u32 {
    let mut buf = [0, 0, 0, 0];
    let id = u8vec(4);
    for i in 0..4 {
        buf[i] = id[i];
    }
    u32::from_be_bytes(buf)
}

#[allow(dead_code)]
pub fn u32color() -> u32 {
    let mut buf = [255, 255, 255, 255];
    let id = u8vec(3);
    for i in 0..3 {
        buf[i + 1] = id[i];
    }
    u32::from_be_bytes(buf)
}
