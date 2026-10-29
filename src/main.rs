use std::{fmt::format, fs::read_to_string};

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
    Ident,
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
                            kind: TokenKind::Newline,
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
                    let digit = lexer.peek();
                    if !digit.unwrap().is_digit(10){
                        break;
                    }
                    final_digit += &digit.unwrap().to_string();
                    lexer.next();
                }
                
                Token { kind: TokenKind::Int(final_digit.parse::<i64>().unwrap().clone()), row: 0, col: 0 }
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
