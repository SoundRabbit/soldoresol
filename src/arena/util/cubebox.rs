use super::Pack;
use async_trait::async_trait;
use wasm_bindgen::prelude::*;

pub struct Cubebox<T> {
    pub px: T,
    pub py: T,
    pub pz: T,
    pub nx: T,
    pub ny: T,
    pub nz: T,
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Cubebox<T> {
    async fn pack(&self, is_deep: bool) -> JsValue {
        (object! {
            "px": self.px.pack(is_deep).await,
            "py": self.py.pack(is_deep).await,
            "pz": self.pz.pack(is_deep).await,
            "nx": self.nx.pack(is_deep).await,
            "ny": self.ny.pack(is_deep).await,
            "nz": self.nz.pack(is_deep).await,
        })
        .into()
    }
}

impl<T> std::ops::Index<usize> for Cubebox<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx % 6 {
            0 => &self.px,
            1 => &self.py,
            2 => &self.pz,
            3 => &self.nx,
            4 => &self.ny,
            5 => &self.nz,
            _ => unreachable!(),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Cubebox<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx % 6 {
            0 => &mut self.px,
            1 => &mut self.py,
            2 => &mut self.pz,
            3 => &mut self.nx,
            4 => &mut self.ny,
            5 => &mut self.nz,
            _ => unreachable!(),
        }
    }
}

impl<T: Clone> Clone for Cubebox<T> {
    fn clone(&self) -> Self {
        Self::with(|idx| self[idx].clone())
    }
}

impl<T> Cubebox<T> {
    pub fn with(mut f: impl FnMut(usize) -> T) -> Self {
        Self {
            px: f(0),
            py: f(1),
            pz: f(2),
            nx: f(3),
            ny: f(4),
            nz: f(5),
        }
    }
}
