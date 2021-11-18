uses! {
    super::util::Pack;
    super::BlockRef;
    super::super::resource::ImageData;
}

packable! {
    [pub Player]
    icon: Option<BlockRef<ImageData>> = None;
}
