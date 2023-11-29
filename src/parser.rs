use crate::tokens::Token;

pub fn parser(tokens: Vec<Token>) {
    for token in tokens {
        println!("{:?}", token);
    }
}
