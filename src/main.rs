mod repl;
mod interpreter;

use std::io;
use iota::iota;
use crate::interpreter::{parse, semantic_analysis, tokenize};
use crate::repl::{handle_input};

enum MetaCommands {
    META_COMMAND_EXIT,
    META_COMMAND_CONTINUE,
}

fn main() {
    let mut buffer = String::new();
    println!("Welcome to StellarStore!");
    while true {
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                match handle_input(&buffer) {
                    MetaCommands::META_COMMAND_CONTINUE => {
                        let tokenized = tokenize(&buffer);
                        let parsed = parse(tokenized);
                        println!("{:?}", parsed);
                        let analyzed = semantic_analysis(parsed);
                        println!("{:?}", analyzed);
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
}

