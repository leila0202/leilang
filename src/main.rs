use std::collections::HashMap;
use std::process::exit;
use std::{fmt::format, fs::read_to_string};
use std::env;

type CharStream<'b,'a> = &'a mut std::iter::Peekable<std::str::Chars<'b>>;



#[derive(Debug)]
enum OpKind {
    Ass,
    Add,
    AddAss,
    Sub,
    SubAss,
    Mul,
    MulAss,
    Div,
    DivAss,
    Eq,
    Neq,
    Neg,
    Gt,
    Lt,
    Ge,
    Le
}

#[derive(Debug)]
enum TypeKind {
    Int(i64),
    Float(f64),
    Char(char),
    String(String)
}
#[derive(Debug)]
enum TokenKind {
    Eof,
    Error,
    Unknown(String),
    Op(OpKind),
    Type(TypeKind),
    Colon,
    Ident(String),
    Space,
    Keyword(String),
    Newline,
    OParen,
    CParen,
    OCurly,
    CCurly,
    OSquare,
    CSquare,
    Quote,
    DQuote
    
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    row: usize,
    col: usize,
}


const KEYWORDS: [&str;7] = ["int", "float", "if", "while", "for", "return", "char"];


fn token_next(lexer: &mut Lexer, filepath: &String) -> Token {
    //println!("token_next aufgerufen");

    if let Some(c) = lexer.iter.next(){
        let row_old = lexer.row;
        let col_old = lexer.col;
        //println!("lexer next aufgerufen");
        lexer.row += 1;
        match c {
            ' ' => {
                lexer.row += 1;
                token_next(lexer, filepath)
            }

            ',' => {
                Token { kind: TokenKind::Colon, row: lexer.row, col: lexer.col }
            }

            '"' => {
                Token { kind: TokenKind::DQuote, row: lexer.row, col: lexer.col }
            }

             '\'' => {
                Token { kind: TokenKind::Quote, row: lexer.row, col: lexer.col }
            }

            '(' => {
                Token { kind: TokenKind::OParen, row: lexer.row, col: lexer.col }
            }

            ')' => {
                Token { kind: TokenKind::CParen, row: lexer.row, col: lexer.col }
            }

            '{' => {
                Token { kind: TokenKind::OCurly, row: lexer.row, col: lexer.col }
            }

            '}' => {
                Token { kind: TokenKind::CCurly, row: lexer.row, col: lexer.col }
            }


            '[' => {
                Token { kind: TokenKind::OSquare, row: lexer.row, col: lexer.col }
            }

            ']' => {
                Token { kind: TokenKind::CSquare, row: lexer.row, col: lexer.col }
            }

            '\r'  => {
                if lexer.iter.peek() == Some(&'\n') {
                    lexer.iter.next();
                    lexer.col +=1;
                    lexer.row = 1;
                        Token {
                            kind: TokenKind::Newline,
                            row: lexer.row,
                            col: lexer.col
                        }
                } else {
                    eprintln!("Error: {}:{}:{}: invalid Newline token", filepath, lexer.col, lexer.row);
                    Token {
                            kind: TokenKind::Error,
                            row: lexer.row,
                            col: lexer.col
                        }
                }
            }

            '<' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.row += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Le),
                            row: lexer.row,
                            col: lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Lt),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '>' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.row += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Ge),
                            row: lexer.row,
                            col: lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Gt),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '!' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.row += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Neq),
                            row: lexer.row,
                            col: lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Neg),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '=' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.row += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Eq),
                            row: lexer.row,
                            col : lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Ass),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '+' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.row += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::AddAss),
                            row: lexer.row,
                            col: lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Add),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '-' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.row += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::SubAss),
                            row: lexer.row,
                            col: lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Sub),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '*' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                        Token {
                            kind: TokenKind::Op(OpKind::MulAss),
                            row: lexer.row,
                            col : lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Mul),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '/' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                        Token {
                            kind: TokenKind::Op(OpKind::DivAss),
                            row: lexer.row,
                            col : lexer.col
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Div),
                        row: lexer.row,
                        col: lexer.col
                    }
                }
            }

            '0'..='9' => {
                let mut is_float = false;
                let mut dot_count = 0;
                let mut final_digit;
                final_digit = c.to_string();
                loop {
                    
                    if let Some(digit) = lexer.iter.peek() {
                        if dot_count > 1 {
                            eprintln!("Error: {}:{}:{} wrong formated float {}", filepath, lexer.row, lexer.col, final_digit);
                            exit(-1);
                        }
                        if !digit.is_digit(10){
                            if digit == &'.'{
                                is_float = true;
                                //dot_count += 1;
                            }
                            else {
                                break;
                            }
                        }
                        final_digit += &digit.to_string();
                        
                        lexer.iter.next();

                    }
                    else{
                        break;
                    }
                }

                let mut digit_kind = TypeKind::Float(final_digit.parse::<f64>().unwrap());

                
                if !is_float {
                     digit_kind = TypeKind::Int(final_digit.parse::<i64>().unwrap());
                }
                Token { 
                    kind: TokenKind::Type(digit_kind), 
                    row: lexer.row,
                    col: lexer.col 
                }
            }

            'A'..='Z' |'a'..='z' | '_'  => {
                let mut final_char;
                final_char = c.to_string();
                loop {
                    if let Some(char) = lexer.iter.peek() {
                        if !char.is_alphanumeric() && char != &'_'{
                            break;
                        }
                        final_char += &char.to_string();
                        lexer.iter.next();

                    }
                    else {
                        break;
                    }
                }
                let kind;
                lexer.row += final_char.len()-1;
                if KEYWORDS.contains(&final_char.as_str()) {
                    kind = TokenKind::Keyword(final_char);
                    
                } else {
                    kind = TokenKind::Ident(final_char);
                }
                
                Token { kind: kind, row: lexer.row, col: lexer.col }
            }
            _ => {
                Token { kind: TokenKind::Unknown(c.to_string()), row: lexer.row, col: lexer.col}
            }
        }
    } else {
        return Token { kind: TokenKind::Eof,  row: lexer.row, col: lexer.col}
    }

}


struct Lexer <'a>{
     iter: std::iter::Peekable<std::str::Chars<'a>>,
     col: usize,
     row: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file was provided.");
        exit(-1);
    }
    let filepath  = &args[1];
    let content= read_to_string(filepath).unwrap();
    let content = content.trim();
    let mut lexer = Lexer {
        iter: content.chars().peekable(),
        col: 1,
        row: 1
    };
    //let lexer = &mut raw_lexer;
   
    loop {
        let token = token_next(&mut lexer, filepath);
        match token.kind {
           TokenKind::Eof => {
                break;
            }
            _ => {
                println!("{:?} located at {}:{}:{}", token.kind, filepath, token.col, token.row);
            }
             
        }
    }
}
