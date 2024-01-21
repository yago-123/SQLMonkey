
use crate::pager::Pager;
use crate::table::Table;
use std::collections::HashMap;


const MAX_LENGTH_STRING: u32 = 100;


pub struct MasterTable {
    // contains information of other tables, indexes...
}

pub struct DB {
    tables: HashMap<String, Table>,
}

impl DB {
    pub fn new() -> DB {
        DB {
           tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &String) -> Result<(), String> {
        if self.tables.contains_key(name) {
            return Err("".to_string())
        }

        match Table::new() {
            Ok(t) => {
                self.tables.insert(name.clone(), t);
                Ok(())
            },
            Err(error) => Err("".to_string()),
        }
    }

    pub fn delete_table(&mut self, name: String) {
        if let Some(table) = self.tables.get(&name) {
            self.tables.remove_entry(&name);
        }
    }

    pub fn retrieve_table_names(&self) -> Vec<String> {
        return self.tables.keys().cloned().collect()
    }

    pub fn drop_db(&mut self) {
        self.tables = HashMap::new()
    }
}