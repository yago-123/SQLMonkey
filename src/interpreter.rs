
use iota::iota;

iota! {
    const SELECT: usize = 0 << iota;
        , INSERT
        , FROM
        , INTO
        , VACUUM
        , EQUAL
        , COMMA
        , END_OF_DIRECTIVE
}

const SQLKeysStr: &[&str] = &[
    "SELECT",
    "INSERT",
    "FROM",
    "INTO",
    "VACUUM",
    "=",
    ",",
    "\n"
];

pub fn tokenize(buffer: &String) -> Vec<&str> {
    let mut tokens: Vec<&str> = buffer.split(" ").collect();
    // tokens.split(",");

    for i in 0..tokens.len() {
        for directive in SQLKeysStr {
            if tokens.get(i) == Some(directive) {
                tokens[i] = directive;
            }
        }
    }

    return tokens
}

pub fn parse(tokens: Vec<&str>) {
    // dumb parsing
    if tokens[0] == SQLKeysStr[SELECT] && tokens.len().eq(&4) {
        // SELECT * FROM table

    } else if tokens[0] == SQLKeysStr[INSERT] && tokens[0] == SQLKeysStr[INTO] && tokens.len().eq(&5) {
        // INSERT key value INTO table

    }


    return
}

pub fn semantic_analysis(tokens: Vec<&str>) -> bool {


    return true
}