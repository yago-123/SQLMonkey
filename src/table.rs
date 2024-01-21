use std::collections::BTreeMap;
use crate::pager::Pager;

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
    persistence: Pager,
}

impl Table {
    pub fn new() -> Table {
        Table{
            rows: Vec::new(),
            persistence: Pager::new(),
            btree: BTreeMap::new(),
        }
    }

    pub fn insert_row() {

    }

    pub fn retrieve_row() {

    }
}
