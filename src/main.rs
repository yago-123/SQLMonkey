mod repl;
mod interpreter;
mod table;
mod db;
mod encoding;
mod pager;
mod freelist;
mod cache;
mod encoder;
use crate::pager::Pager;


fn main() {
    let mut pager: Pager;
    match Pager::new(Some("random-datastore".to_string())) {
        Ok(p) => pager = p,
        Err(_) => panic!(""),
    }

    match pager.insert_row(vec![b'a', b'b', b'c']) {
        Ok(i) => println!("Written cursor to {}", i),
        Err(error) => println!("Error inserting row: {}", error)
    }
}
