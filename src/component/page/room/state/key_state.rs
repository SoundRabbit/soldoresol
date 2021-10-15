pub struct KeyState {
    space_key: bool,
    alt_key: bool,
    ctrl_key: bool,
    shift_key: bool,
}

impl KeyState {
    fn update(&mut self, e: web_sys::KeyboardEvent, is_key_down: bool) {
        let alt_key = e.alt_key();
        let ctrl_key = e.ctrl_key() || e.meta_key();
        let shift_key = e.shift_key();
        let space_key = if e.code() == "Space" {
            is_key_down
        } else {
            self.space_key
        };

        self.alt_key = alt_key;
        self.ctrl_key = ctrl_key;
        self.shift_key = shift_key;
        self.space_key = space_key;
    }
}
