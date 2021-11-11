use super::*;

impl Constructor for Room {
    fn constructor(props: &Props) -> Self {
        Self {
            arena: ArenaMut::clone(&props.arena),
            local_arena: Arena::new(),

            chat: BlockMut::none(),

            craftboard: Craftboard::new(),
            modeless_container: TabModelessContainer::new(),
        }
    }
}
