use std::{collections::HashMap, sync::RwLock};

use crate::services::table_traits::AnyTable;

pub struct DbData {
    pub data: RwLock<HashMap<String, Box<dyn AnyTable>>>,
    pub memory_limit: u32,
    pub current_memory: RwLock<u32>,
    pub least_used_table: RwLock<HashMap<String, u32>>,
}

impl DbData {
    pub fn new(memory_limit: u32) -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
            memory_limit: memory_limit,
            current_memory: RwLock::new(0),
            least_used_table: RwLock::new(HashMap::new()),
        }
    }

    fn get_least_used_table(&self) -> Option<String> {
        self.least_used_table
            .read()
            .unwrap()
            .iter()
            .min_by_key(|(_, v)| *v)
            .map(|(k, _)| k.clone())
    }

    pub fn clear_memory_if_full(&self, new_data_size: u32) {
        let mut current = self.current_memory.write().unwrap();

        while *current + new_data_size > self.memory_limit {
            let table = match self.get_least_used_table() {
                Some(t) => t,
                None => break,
            };

            if let Some(removed) = self.data.write().unwrap().remove(&table) {
                *current -= removed.memory_used();
            }
        }
    }
}
