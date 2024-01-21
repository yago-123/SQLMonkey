mod repl;
mod interpreter;
mod table;
mod db;
mod encoding;
mod pager;

use std::io;
use crate::interpreter::{parse, semantic_analysis, tokenize};
use crate::repl::{handle_input};
use crate::db::{DB};
use crate::pager::Pager;
use crate::table::Table;

enum MetaCommands {
    META_COMMAND_EXIT,
    META_COMMAND_CONTINUE,
}

fn main() {
    /*
    let mut db = DB::new();
    db.create_table(&"table".to_string());

    let mut buffer = String::new();

    println!("Welcome to SQLMonkey!");
    while true {
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                match handle_input(&buffer) {
                    MetaCommands::META_COMMAND_CONTINUE => {
                        let tokenized = tokenize(&buffer);
                        //let parsed = parse(tokenized);
                        // println!("{:?}", parsed);
                        // let analyzed = semantic_analysis(parsed);
                        // println!("{:?}", analyzed);
                    }
                    MetaCommands::META_COMMAND_EXIT => return,
                }
            },
            Err(error) => {
                println!("Error while retrieving input: {}", error);
                return
            },
        }

        buffer = String::new();
    }

    db.create_table(&"table".to_string());
     */

    let mut pager = Pager::new();
    match pager.insert_row(vec![b'a', b'b', b'c']) {
        Ok(value) => println!("Inserted row correctly!"),
        Err(error) => println!("Error inserting row: {}", error)
    }

    match pager.insert_row_in_position(vec![b'a', b'b', b'c'], 100) {
        Ok(value) => println!("Inserted row correctly!"),
        Err(error) => println!("Error inserting row: {}", error)
    }
}
