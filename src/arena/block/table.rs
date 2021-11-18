uses! {
    super::BlockMut;
    super::Craftboard;
    super::Boxblock;
    super::util::Pack;
}

packable! {
    [pub Table]
    boxblocks: Vec<BlockMut<Boxblock>> = vec![];
    craftboards: Vec<BlockMut<Craftboard>> = vec![];
}

impl Table {
    pub fn boxblocks(&self) -> &Vec<BlockMut<Boxblock>> {
        &self.boxblocks
    }
    pub fn craftboards(&self) -> &Vec<BlockMut<Craftboard>> {
        &self.craftboards
    }
    pub fn craftboards_push(&mut self, craftboard: BlockMut<Craftboard>) {
        self.craftboards.push(craftboard);
    }
}
