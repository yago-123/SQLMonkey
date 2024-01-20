
const MAX_LENGTH_STRING: u32 = 100;

pub struct Row {
    Id: u32,
    Key: String,
    Value: String,
}

pub struct Table{
    Name: String,
    Rows: Vec<Row>,
}

pub struct DB {
    Tables: Vec<Table>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table{
            Name: name,
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
           Tables: Vec::new(),
        }
    }

    pub fn create_table(mut self, name: String) {
        self.Tables.push(Table::new(name));
    }

    pub fn delete_table(mut self, name: String) {

    }

    pub fn drop_db(mut self) {
        self.Tables = Vec::new()
    }
}