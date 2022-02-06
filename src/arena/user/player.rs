uses! {
    super::util::Pack;
    super::BlockRef;
    super::super::resource::ImageData;
}

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
