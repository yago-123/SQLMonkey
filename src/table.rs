use std::collections::BTreeMap;
use crate::pager::{Page, Pager};

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
    pub fn new() -> Result<Table, String> {
        let persistence = Pager::new(None).map_err(|error| format!("error creating Pager: {}", error))?;

        Ok(Table {
            persistence,
            btree: BTreeMap::new(),
        })
    }

    pub fn insert_row() {

    }

    pub fn retrieve_row() {
        //
    }

    pub fn modify_record() {

    }
}
