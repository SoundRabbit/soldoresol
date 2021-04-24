use std::{
    iter::Iterator,
    ops::{Deref, DerefMut},
    rc::Rc,
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
    grabbed: Option<Loc<f64>>,
    movable: Loc<bool>,
    parent: Option<Rc<web_sys::Element>>,
    payload: T,
}

pub struct Collection<T> {
    modelesses: Vec<Option<Modeless<T>>>,
    max_z_index: i32,
    parent: Option<Rc<web_sys::Element>>,
}

fn bind(n: f64, m: f64, v: f64) -> f64 {
    if m < v {
        if m < n {
            v
        } else {
            n
        }
    } else if m > v {
        if m > n {
            v
        } else {
            n
        }
    } else {
        n
    }
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
            grabbed: None,
            movable: Loc::new(false, false, false, false),
            parent: None,
            payload,
        }
    }

    fn window_pos(&self, pos: &[f64; 2]) -> [f64; 2] {
        let (vw, vh) = if let Some(element) = &self.parent {
            let cr = element.get_bounding_client_rect();
            (cr.width(), cr.height())
        } else {
            let window = web_sys::window().unwrap();
            let vw = window.inner_width().unwrap().as_f64().unwrap();
            let vh = window.inner_height().unwrap().as_f64().unwrap();
            (vw, vh)
        };
        let x = pos[0] / vw * 100.0;
        let y = pos[1] / vh * 100.0;
        [x, y]
    }

    fn set_parent(&mut self, element: Option<Rc<web_sys::Element>>) {
        self.parent = element;
    }

    pub fn position(&self) -> [f64; 2] {
        let r = self.position.right;
        let b = self.position.bottom;
        [
            bind(self.position.left + bind(r, 99.0, 100.0) - r, 1.0, 0.0),
            bind(self.position.top + bind(b, 99.0, 100.0) - b, 1.0, 0.0),
        ]
    }

    pub fn size(&self) -> [f64; 2] {
        let l = self.position.left;
        let t = self.position.top;
        [
            bind(self.position.right + bind(l, 1.0, 0.0) - l, 99.0, 100.0) - self.position()[0],
            bind(self.position.bottom + bind(t, 1.0, 0.0) - t, 99.0, 100.0) - self.position()[1],
        ]
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        let [x, y] = self.window_pos(&[x, y]);
        self.set_position_r(x, y);
    }

    pub fn set_position_r(&mut self, x: f64, y: f64) {
        let dx = x - self.position.left;
        let dy = y - self.position.top;

        self.position.left += dx;
        self.position.right += dx;
        self.position.top += dy;
        self.position.bottom += dy;
    }

    pub fn set_size_r(&mut self, w: f64, h: f64) {
        self.position.right = self.position.left + w;
        self.position.bottom = self.position.top + h;
    }

    pub fn z_index(&self) -> i32 {
        self.z_index
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    pub fn is_grabbed(&self) -> bool {
        self.grabbed.is_some()
    }

    pub fn grab(&mut self, x: f64, y: f64) {
        let [x, y] = self.window_pos(&[x, y]);
        self.grabbed = Some(Loc::new(
            self.position.top - y,
            self.position.left - x,
            self.position.bottom - y,
            self.position.right - x,
        ));
    }

    pub fn drop(&mut self) {
        self.grabbed = None;
        self.movable = Loc::new(false, false, false, false);
    }

    pub fn set_movable(&mut self, top: bool, left: bool, bottom: bool, right: bool) {
        self.movable = Loc::new(top, left, bottom, right);
    }

    pub fn move_with_mouse_pos(&mut self, x: f64, y: f64) {
        if let Some(grabbed) = &self.grabbed {
            let pos = self.window_pos(&[x, y]);

            let top = pos[1] + grabbed.top;
            let left = pos[0] + grabbed.left;
            let bottom = pos[1] + grabbed.bottom;
            let right = pos[0] + grabbed.right;

            let diff_top = if self.movable.top { -top.min(0.0) } else { 0.0 };
            let diff_left = if self.movable.left {
                -left.min(0.0)
            } else {
                0.0
            };
            let diff_bottom = if self.movable.bottom {
                -(bottom.max(100.0) - 100.0)
            } else {
                0.0
            };
            let diff_right = if self.movable.right {
                -(right.max(100.0) - 100.0)
            } else {
                0.0
            };

            if self.movable.top {
                self.position.top = top + diff_top + diff_bottom;
            }
            if self.movable.bottom {
                self.position.bottom = bottom + diff_top + diff_bottom;
            }

            if self.movable.left {
                self.position.left = left + diff_left + diff_right;
            }
            if self.movable.right {
                self.position.right = right + diff_left + diff_right;
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

impl<T> AsMut<T> for Modeless<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.payload
    }
}

impl<T> Collection<T> {
    pub fn new() -> Self {
        Self {
            modelesses: vec![],
            max_z_index: 0,
            parent: None,
        }
    }

    pub fn open(&mut self, mut modeless: Modeless<T>) -> ModelessId {
        modeless.set_z_index(self.max_z_index + 1);
        modeless.set_parent(self.parent.as_ref().map(|el| Rc::clone(&el)));
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

    pub fn get(&self, modeless_id: ModelessId) -> Option<&Modeless<T>> {
        self.modelesses.get(modeless_id).and_then(|x| x.as_ref())
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

    pub fn grabbed(&self) -> Option<ModelessId> {
        self.modelesses
            .iter()
            .position(|m| m.as_ref().map(|m| m.is_grabbed()).unwrap_or(false))
    }

    pub fn set_parent(&mut self, element: Option<web_sys::Element>) {
        self.parent = element.map(|el| Rc::new(el));
    }
}
