use std::sync::{Arc, RwLock};

use crate::services::{
    db_data::DbData,
    table_traits::{AnyTable, TableSchema},
};

pub struct Table<T> {
    data: RwLock<Vec<T>>,
    table_name: String,
    table_id: String,
    memory_used: RwLock<u32>,
    db_data: Arc<DbData>,
}

impl<T: TableSchema> Table<T> {
    pub fn new(db_data: Arc<DbData>) -> Self {
        let table_name = T::table_name();
        let table_id = T::table_id();

        Self {
            data: RwLock::new(vec![]),
            table_name,
            table_id,
            db_data,
            memory_used: RwLock::new(0),
        }
    }

    fn insert(&self, value: T) {
        self.db_data.clear_memory_if_full(10);
        let mut data = self.data.write().unwrap();
        data.push(value);
    }
}

impl<T: TableSchema + Send + Sync> AnyTable for Table<T> {
    fn table_id(&self) -> &str {
        &self.table_id
    }

    fn table_name(&self) -> &str {
        &self.table_name
    }

    fn memory_used(&self) -> u32 {
        *self.memory_used.read().unwrap()
    }
}