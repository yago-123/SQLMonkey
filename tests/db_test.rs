#[path = "../src/db.rs"] mod db;


#[cfg(test)]
mod tests {
    use crate::db::DB;

    #[test]
    fn test_create_table() {
        let mut db = DB::new();
        db.create_table("hello".to_string());
    }

    #[test]
    fn test_delete_table() {

    }

    #[test]
    fn test_drop_db() {

    }
}