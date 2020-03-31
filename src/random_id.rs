pub fn hex(len: usize) -> String {
    let mut id: Vec<u8> = Vec::new();
    id.resize(len, 0);
    web_sys::window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut id)
        .expect("");
    hex::encode(&id)
}
