
use iota::iota;

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

pub fn tokenize(buffer: &String) -> Vec<&str> {
    return buffer.split(" ").collect()
}

pub fn parse(mut tokens: Vec<&str>) -> Vec<&str> {
    for i in 0..tokens.len() {
        for directive in SQLKeysStr {
            if tokens.get(i) == Some(directive) {
                tokens[i] = directive;

            }
        }
    }

    return tokens
}

pub fn semantic_analysis(tokens: Vec<&str>) -> bool {


    return true
}