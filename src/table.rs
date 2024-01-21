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
    btree: BTreeMap<u64, u64>,
    persistence: Pager,
}

impl Table {
    pub fn new() -> Table {
        Table{
            persistence: Pager::new(),
            btree: BTreeMap::new(),
        }
    }

    pub fn insert_row() {

    }

    pub fn retrieve_row() {
        //
    }

    pub fn modify_record() {

    }
}
