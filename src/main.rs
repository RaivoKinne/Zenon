mod lexer;
mod parser;
mod tokens;

use std::fs::File;
use std::io::Read;

use lexer::lexer;
use parser::parser;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let tokens = lexer(content);
    parser(tokens);
    Ok(())
}
