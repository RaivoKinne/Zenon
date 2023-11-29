use crate::tokens::{Token, TokenType};

pub fn lexer(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_numeric() {
            tokens.push(Token::new(TokenType::Number, c.to_string()));
            continue;
        }
        if c.is_alphabetic() {
            let mut ident = c.to_string();
            while let Some(c) = chars.peek() {
                if c.is_alphanumeric() {
                    ident.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }

            let token_type = match ident.as_str() {
                "let" => TokenType::Let,
                "fn" => TokenType::Fn,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "return" => TokenType::Return,
                _ => TokenType::Identifier,
            };
            tokens.push(Token::new(token_type, ident));
        }

        match c {
            '=' => tokens.push(Token::new(TokenType::Equal, c.to_string())),
            '>' => tokens.push(Token::new(TokenType::Greater, c.to_string())),
            '<' => tokens.push(Token::new(TokenType::Less, c.to_string())),
            '!' => tokens.push(Token::new(TokenType::Bang, c.to_string())),
            ',' => tokens.push(Token::new(TokenType::Comma, c.to_string())),
            '.' => tokens.push(Token::new(TokenType::Dot, c.to_string())),
            ';' => tokens.push(Token::new(TokenType::Semicolon, c.to_string())),
            ':' => tokens.push(Token::new(TokenType::Colon, c.to_string())),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, c.to_string())),
            '}' => tokens.push(Token::new(TokenType::RightBrace, c.to_string())),
            '+' => tokens.push(Token::new(TokenType::Plus, c.to_string())),
            '-' => tokens.push(Token::new(TokenType::Minus, c.to_string())),
            '*' => tokens.push(Token::new(TokenType::Star, c.to_string())),
            '/' => tokens.push(Token::new(TokenType::Slash, c.to_string())),
            '(' => tokens.push(Token::new(TokenType::LeftParen, c.to_string())),
            ')' => tokens.push(Token::new(TokenType::RightParen, c.to_string())),
            _ => {}
        }
    }

    tokens
}
