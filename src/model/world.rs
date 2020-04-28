use super::table::Table;
use crate::random_id;

pub struct World {
    table_id: u32,
    table: Table,
}

impl World {
    pub fn new(table_size: [f64; 2]) -> Self {
        Self {
            table_id: random_id::u32val(),
            table: Table::new(table_size, 64.0),
        }
    }

    pub fn table_id(&self) -> u32 {
        self.table_id
    }

    pub fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
}
