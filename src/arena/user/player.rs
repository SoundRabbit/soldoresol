uses! {
    super::util::Pack;
    super::BlockRef;
    super::super::resource::ImageData;
}

block! {
    [pub Player(constructor, pack)]
    icon: Option<BlockRef<ImageData>> = None;
}
