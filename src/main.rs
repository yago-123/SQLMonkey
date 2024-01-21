mod repl;
mod interpreter;
mod table;
mod db;
mod encoding;
mod pager;

use crate::pager::Pager;

enum MetaCommands {
    META_COMMAND_EXIT,
    META_COMMAND_CONTINUE,
}

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

    match pager.insert_row_in_position(vec![b'a', b'b', b'c'], 100) {
        Ok(()) => (),
        Err(error) => println!("Error inserting row: {}", error)
    }
}
