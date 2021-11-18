use super::*;

impl Constructor for Room {
    fn constructor(props: &Props) -> Self {
        Self {
            arena: ArenaMut::clone(&props.arena),
            local_arena: Arena::new(),

            chat: BlockMut::<block::Chat>::none(),
            world: BlockMut::<block::World>::none(),

            table: Table::new(),
            modeless_container: TabModelessContainer::new(),
        }
    }
}
