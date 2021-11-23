uses! {
    super::BlockMut;
    super::Table;
    super::util::Pack;
}

block! {
    [pub Scene(constructor, pack)]
    selecting_table: BlockMut<Table> = BlockMut::<Table>::none();
    tables: Vec<BlockMut<Table>> = vec![];
    name: String = String::from("新規シーン");
}

impl Scene {
    pub fn selecting_table(&self) -> &BlockMut<Table> {
        &self.selecting_table
    }
    pub fn tables(&self) -> &Vec<BlockMut<Table>> {
        &self.tables
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn tables_push(&mut self, table: BlockMut<Table>) {
        if self.tables.len() == 0 {
            self.selecting_table = BlockMut::clone(&table);
        }

        self.tables.push(table);
    }
}
