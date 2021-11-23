use super::super::organism::table_menu::TableMenu;
use super::*;

impl Constructor for Room {
    fn constructor(props: &Props) -> Self {
        let world = BlockMut::<block::World>::none();
        Self {
            arena: ArenaMut::clone(&props.arena),
            local_arena: Arena::new(),

            chat: BlockMut::<block::Chat>::none(),
            world: BlockMut::clone(&world),

            table: Table::new(ArenaMut::clone(&props.arena), BlockMut::clone(&world)),
            modeless_container: TabModelessContainer::new(),

            table_tool: TableMenu::initial_selected(),
            ok_to_catch_file: true,
        }
    }
}
