use crate::pager::{Page, Pager};
use std::collections::HashMap;
use std::collections::BTreeMap;
use uuid::Uuid;

const MAX_LENGTH_STRING: u32 = 100;


pub struct Row {
    id: u32,
    key: String,
    value: String,
    // ...
    // Stored in pages
}

pub struct Table {
    rows: Vec<Row>,
    // contains table data sorted (primary key)
    btree: BTreeMap<String, String>,
}

pub struct MasterTable {
    // contains information of other tables, indexes...
}

pub struct FileHeader {
    path: Uuid,
    page_size: u32,
}

pub struct DB {
    tables: HashMap<String, Table>,
    persistence: FileHeader,
    pager: Pager,
}

impl Table {
    pub fn new() -> Table {
        Table{
            rows: Vec::new(),
            btree: BTreeMap::new(),
        }
    }

    pub fn insert_row() {

    }

    pub fn retrieve_row() {

    }
}

impl FileHeader {
    pub fn new() -> FileHeader {
        FileHeader {
            page_size: 0,
            path: Uuid::max(),
        }
    }
}

impl DB {
    pub fn new() -> DB {
        DB {
           tables: HashMap::new(),
            persistence: FileHeader::new(),
            pager: Pager::new(),
        }
    }

    pub fn create_table(&mut self, name: &String) -> bool {
        match self.tables.contains_key(name) {
            true => return false,
            false => {
                // create new table
                self.tables.insert(name.clone(), Table::new());
                return true
            }
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