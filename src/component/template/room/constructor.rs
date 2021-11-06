use super::*;

impl Constructor for Room {
    fn constructor(props: &Props) -> Self {
        Self {
            block_arena: block::Arena::new(),
            local_block_arena: block::Arena::new(),

            craftboard: Craftboard::new(),
            modeless_container: TabModelessContainer::new(),
        }
    }
}
