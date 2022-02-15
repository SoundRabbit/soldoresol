uses! {
    super::BlockMut;
    super::Craftboard;
    super::Boxblock;
    super::util::Pack;
}

block! {
    [pub Table(constructor, pack)]
    boxblocks: Vec<BlockMut<Boxblock>> = vec![];
    craftboards: Vec<BlockMut<Craftboard>> = vec![];
    default_is_bind_to_grid: bool = true;
}

impl Table {
    pub fn boxblocks(&self) -> &Vec<BlockMut<Boxblock>> {
        &self.boxblocks
    }
    pub fn push_boxblock(&mut self, boxblock: BlockMut<Boxblock>) {
        self.boxblocks.push(boxblock);
    }
    pub fn craftboards(&self) -> &Vec<BlockMut<Craftboard>> {
        &self.craftboards
    }
    pub fn push_craftboard(&mut self, craftboard: BlockMut<Craftboard>) {
        self.craftboards.push(craftboard);
    }
    pub fn default_is_bind_to_grid(&self) -> bool {
        self.default_is_bind_to_grid
    }
}
