use std::{
    iter::Iterator,
    ops::{Deref, DerefMut},
};

pub type ModelessId = usize;

struct Loc<T> {
    top: T,
    left: T,
    bottom: T,
    right: T,
}

pub struct Modeless<T> {
    z_index: i32,
    position: Loc<f64>,
    grubbed: Option<[f64; 2]>,
    movable: Loc<bool>,
    payload: T,
}

pub struct Collection<T> {
    modelesses: Vec<Option<Modeless<T>>>,
    max_z_index: i32,
}

fn window_pos(pos: &[f64; 2]) -> [f64; 2] {
    let window = web_sys::window().unwrap();
    let vw = window.inner_width().unwrap().as_f64().unwrap();
    let vh = window.inner_height().unwrap().as_f64().unwrap();
    let x = pos[0] / vw * 100.0;
    let y = pos[1] / vh * 100.0;
    [x, y]
}

impl<T> Loc<T> {
    fn new(top: T, left: T, bottom: T, right: T) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<T> Modeless<T> {
    pub fn new(payload: T) -> Self {
        Self {
            z_index: 0,
            position: Loc::new(20.0, 20.0, 60.0, 60.0),
            grubbed: None,
            movable: Loc::new(false, false, false, false),
            payload,
        }
    }

    pub fn position(&self) -> [f64; 2] {
        [self.position.left, self.position.top]
    }

    pub fn size(&self) -> [f64; 2] {
        [
            self.position.right - self.position.left,
            self.position.bottom - self.position.top,
        ]
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        let [x, y] = window_pos(&[x, y]);
        let dx = x - self.position.left;
        let dy = y - self.position.top;

        self.position.left += dx;
        self.position.right += x;
        self.position.top += y;
        self.position.bottom += y;
    }

    pub fn set_size(&mut self, w: f64, h: f64) {
        let [w, h] = window_pos(&[w, h]);
        self.position.right = self.position.left + w;
        self.position.bottom = self.position.top + h;
    }

    pub fn z_index(&self) -> i32 {
        self.z_index
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    pub fn is_grubbed(&self) -> bool {
        self.grubbed.is_some()
    }

    pub fn grub(&mut self, x: f64, y: f64) {
        self.grubbed = Some(window_pos(&[x, y]));
    }

    pub fn drop(&mut self) {
        self.grubbed = None;
        self.movable = Loc::new(false, false, false, false);
    }

    pub fn set_movable(&mut self, top: bool, left: bool, bottom: bool, right: bool) {
        self.movable = Loc::new(top, left, bottom, right);
    }

    pub fn move_with_mouse_pos(&mut self, x: f64, y: f64) {
        if let Some(grubbed) = &self.grubbed {
            let pos = window_pos(&[x, y]);

            let dx = pos[0] - grubbed[0];
            let dy = pos[1] - grubbed[1];

            if self.movable.top {
                self.position.top += dy;
            }
            if self.movable.left {
                self.position.left += dx;
            }
            if self.movable.bottom {
                self.position.bottom += dy;
            }
            if self.movable.right {
                self.position.right += dx;
            }
        }
    }
}

impl<T> Deref for Modeless<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<T> DerefMut for Modeless<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payload
    }
}

impl<T> AsRef<T> for Modeless<T> {
    fn as_ref(&self) -> &T {
        &self.payload
    }
}

impl<T> Collection<T> {
    pub fn new() -> Self {
        Self {
            modelesses: vec![],
            max_z_index: 0,
        }
    }

    pub fn open(&mut self, mut modeless: Modeless<T>) -> ModelessId {
        modeless.set_z_index(self.max_z_index + 1);
        self.max_z_index += 1;

        if let Some(insert_point) = self.modelesses.iter().position(|x| x.is_none()) {
            self.modelesses[insert_point] = Some(modeless);
            insert_point
        } else {
            self.modelesses.push(Some(modeless));
            self.modelesses.len() - 1
        }
    }

    pub fn focus(&mut self, modeless_id: ModelessId) {
        if let Some(Some(modeless)) = self.modelesses.get_mut(modeless_id) {
            modeless.set_z_index(self.max_z_index + 1);
            self.max_z_index += 1;
        }
    }

    pub fn close(&mut self, modeless_id: ModelessId) {
        if let Some(modeless) = self.modelesses.get_mut(modeless_id) {
            *modeless = None;
        }
    }

    pub fn get_mut(&mut self, modeless_id: ModelessId) -> Option<&mut Modeless<T>> {
        self.modelesses
            .get_mut(modeless_id)
            .and_then(|x| x.as_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (ModelessId, &Option<Modeless<T>>)> {
        self.modelesses.iter().enumerate()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ModelessId, &mut Option<Modeless<T>>)> {
        self.modelesses.iter_mut().enumerate()
    }
}
