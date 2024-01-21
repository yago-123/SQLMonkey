#[path = "../src/db.rs"] mod db;
#[path = "../src/pager.rs"] mod pager;
#[path = "../src/table.rs"] mod table;

#[cfg(test)]
mod tests {
    use crate::db::DB;

    #[test]
    fn test_create_and_drop_db() {
        let mut db = DB::new();

        // create table and drop database
        db.create_table(&"table-1".to_string());
        db.drop_db();
        assert_eq!(db.retrieve_table_names().len(), 0);
    }

    #[test]
    fn test_create_table() {
        let mut db = DB::new();

        // create one single table
        db.create_table(&"table-1".to_string());
        assert_eq!(db.retrieve_table_names().len(), 1);

        // try to create one table already existent
        let ret = db.create_table(&"table-1".to_string());
        assert_eq!(ret, false);
    }

    #[test]
    fn test_delete_table_non_existent() {
        let mut db = DB::new();

        // delete table non existent
        db.delete_table("non-existent".to_string());
        assert_eq!(db.retrieve_table_names().len(), 0);

        // delete inserted table two times
        db.create_table(&"table-1".to_string());
        db.delete_table("table-1".to_string());
        db.delete_table("table-1".to_string());
        assert_eq!(db.retrieve_table_names().len(), 0);

        // add and delete table
        db.create_table(&"table-2".to_string());
        db.delete_table("table-2".to_string());
        assert_eq!(db.retrieve_table_names().len(), 0);

        // create two tables and delete only one
        db.create_table(&"table-3".to_string());
        db.create_table(&"table-4".to_string());
        db.delete_table("table-3".to_string());
        assert_eq!(db.retrieve_table_names().len(), 1);
        assert_eq!(db.retrieve_table_names()[0], "table-4");
    }

}