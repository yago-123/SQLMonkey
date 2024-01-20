use std::collections::HashMap;

const MAX_LENGTH_STRING: u32 = 100;

pub struct Row {
    Id: u32,
    Key: String,
    Value: String,
}

pub struct Table{
    Rows: Vec<Row>,
}

pub struct DB {
    pub Tables: HashMap<String, Table>,
}

impl Table {
    pub fn new() -> Table {
        Table{
            Rows: Vec::new(),
        }
    }

    pub fn insert_row() {

    }

    pub fn retrieve_row() {

    }
}

impl DB {
    pub fn new() -> DB {
        DB {
           Tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &String) -> bool {
        match self.Tables.contains_key(name) {
            true => return false,
            false => {
                // create new table
                self.Tables.insert(name.clone(), Table::new());
                return true
            }
        }
    }

    pub fn delete_table(&mut self, name: String) {
        if let Some(table) = self.Tables.get(&name) {
            self.Tables.remove_entry(&name);
        }
    }

    pub fn retrieve_table_names(&self) -> Vec<String> {
        return self.Tables.keys().cloned().collect()
    }

    pub fn drop_db(&mut self) {
        self.Tables = HashMap::new()
    }
}