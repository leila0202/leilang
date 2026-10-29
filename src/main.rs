use std::{fmt::format, fs::read_to_string};
use std::collections::HashSet;

type CharStream<'b,'a> = &'a mut std::iter::Peekable<std::str::Chars<'b>>;



#[derive(Debug)]
pub enum OpKind {
    Equals,
    Minus,
    Plus,
    Mul,
    Div
}
#[derive(Debug)]
pub enum TokenKind {
    Eof,
    Error,
    Unknown(String),
    Op(OpKind),
    Colon,
    Ident(String),
    Space,
    Int(i64),
    Float(f64),
    Keyword(String),
    Newline
    
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    row: usize,
    col: usize,
}

const KEYWORDS: [&str;7] = ["int", "float", "if", "while", "for", "return", "char"];

fn token_next(lexer: CharStream) -> Token {

    if let Some(c) = lexer.next(){
        match c {
            ' ' => {
                token_next(lexer)
            }

            '\r'  => {
                if lexer.peek() == Some(&'\n') {
                    lexer.next();
                    return 
                        Token {
                            kind: TokenKind::Newline,
                            row: 0,
                            col : 0
                        }
                } else {
                    println!("Line: {} Col: {}: invalid Newline token", 0, 0);
                    Token {
                            kind: TokenKind::Error,
                            row: 0,
                            col : 0
                        }
                }
            }

            '=' => {
                Token {
                    kind: TokenKind::Op(OpKind::Equals),
                    row: 0,
                    col: 0
                }
            }

            '0'..='9' => {
                let mut final_digit;
                final_digit = c.to_string();
                loop {
                    if let Some(digit) = lexer.peek() {
                        if !digit.is_digit(10){
                            break;
                        }
                        final_digit += &digit.to_string();
                        lexer.next();

                    }
                    else{
                        break;
                    }
                }
                
                Token { kind: TokenKind::Int(final_digit.parse::<i64>().unwrap()), row: 0, col: 0 }
            }

            'A'..='Z' |'a'..='z' | '_'  => {
                let mut final_char;
                final_char = c.to_string();
                loop {
                    if let Some(char) = lexer.peek() {
                        if !char.is_alphanumeric() && char != &'_'{
                            break;
                        }
                        final_char += &char.to_string();
                        lexer.next();

                    }
                    else{
                        break;
                    }
                }
                let kind;
                if KEYWORDS.contains(&final_char.as_str()) {
                    kind = TokenKind::Keyword(final_char);
                } else {
                    kind = TokenKind::Ident(final_char);
                }
                Token { kind: kind, row: 0, col: 0 }
            }
            _ => {
                return Token { kind: TokenKind::Unknown(c.to_string()), row: 0, col:  0};
            }
        }
    } else {
        return Token { kind: TokenKind::Eof,  row: 0, col: 0}
    }

}



fn main() {
    let filepath  = "test.lei";
    let content= read_to_string(filepath).unwrap();
    let content = content.trim();
    let mut raw_lexer = content.chars().peekable();
    let lexer = &mut raw_lexer;
    loop {
        let token = token_next(lexer);
        match token.kind {
           TokenKind::Eof => {
                break;
            }
            _ => {
                println!("{:?}", token);
            }
             
        }
    }
}
