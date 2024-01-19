use std::io;
use iota::iota;

enum MetaCommands {
    META_COMMAND_EXIT,
    META_COMMAND_CONTINUE,
}

iota! {
    const SELECT: u8 = 0 << iota;
        , INSERT
        , FROM
        , END_OF_DIRECTIVE
}

const SQLKeysStr: &[&str] = &[
    "SELECT",
    "INSERT",
    "FROM",
    "\n"
];

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

fn handle_input(buffer: &String) -> MetaCommands {
    if buffer.contains(".exit") {
        return MetaCommands::META_COMMAND_EXIT
    }

    return MetaCommands::META_COMMAND_CONTINUE
}

fn tokenize(buffer: &String) -> Vec<&str> {
    return buffer.split(" ").collect()
}

fn parse(mut tokens: Vec<&str>) -> Vec<&str> {
    for i in 0..tokens.len() {
        for directive in SQLKeysStr {
            if tokens.get(i) == Some(directive) {
                tokens[i] = directive;

            }
        }
    }

    return tokens
}

fn semantic_analysis(tokens: Vec<&str>) -> bool {


    return true
}