#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::resource::ImageData;
use super::util::Pack;
use super::BlockRef;

block! {
    [pub Player(constructor, pack)]
    icon: Option<BlockRef<ImageData>> = None;
    name: String = String::from("プレイヤー");
}

impl Player {
    pub fn icon(&self) -> Option<&BlockRef<ImageData>> {
        self.icon.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
