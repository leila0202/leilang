use std::process::exit;
use std::fs::read_to_string;
use std::env;



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


struct Token {
    kind: TokenKind,
    row: usize,
    col: usize,
}


const KEYWORDS: [&str;7] = ["int", "float", "if", "while", "for", "return", "char"];


fn token_next(lexer: &mut Lexer, filepath: &String) -> Token {
    let row_old = lexer.row;
    let col_old = lexer.col;

    if let Some(c) = lexer.iter.next(){
        
        lexer.col += 1;
        match c {
            ' ' => {
                token_next(lexer, filepath)
            }

            ',' => {
                Token { kind: TokenKind::Colon, row: row_old, col: col_old }
            }

            '"' => {
                Token { kind: TokenKind::DQuote, row: row_old, col: col_old }
            }

             '\'' => {
                Token { kind: TokenKind::Quote, row: row_old, col: col_old }
            }

            '(' => {
                Token { kind: TokenKind::OParen, row: row_old, col: col_old }
            }

            ')' => {
                Token { kind: TokenKind::CParen, row: row_old, col: col_old }
            }

            '{' => {
                Token { kind: TokenKind::OCurly, row: row_old, col: col_old }
            }

            '}' => {
                Token { kind: TokenKind::CCurly, row: row_old, col: col_old }
            }


            '[' => {
                Token { kind: TokenKind::OSquare, row: row_old, col: col_old }
            }

            ']' => {
                Token { kind: TokenKind::CSquare, row: row_old, col: col_old }
            }

            '\r'  => {
                if lexer.iter.peek() == Some(&'\n') {
                    lexer.iter.next();
                    lexer.row += 1;
                    lexer.col =  1;
                        Token {
                            kind: TokenKind::Newline,
                            row: row_old,
                            col: col_old
                        }
                } else {
                    eprintln!("\x1b[31mError:\x1b[0m {}:{}:{}: invalid Newline token", filepath, row_old, col_old);
                    Token {
                            kind: TokenKind::Error,
                            row: row_old,
                            col: col_old
                        }
                }
            }

            '<' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Le),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Lt),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '>' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Ge),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Gt),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '!' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Neq),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Neg),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '=' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::Eq),
                            row: row_old,
                            col : col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Ass),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '+' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::AddAss),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Add),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '-' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::SubAss),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Sub),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '*' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::MulAss),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Mul),
                        row: row_old,
                        col: col_old
                    }
                }
            }

            '/' => {
                if lexer.iter.peek() == Some(&'=') {
                    lexer.iter.next();
                    lexer.col += 1;
                        Token {
                            kind: TokenKind::Op(OpKind::DivAss),
                            row: row_old,
                            col: col_old
                        }
                }
                else {
                    Token {
                        kind: TokenKind::Op(OpKind::Div),
                        row: row_old,
                        col: col_old
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
                            eprintln!("\x1b[31mError:\x1b[0m {}:{}:{} wrong formated float {}", filepath, row_old, col_old + final_digit.len() - 1, final_digit);
                            return Token {
                            kind: TokenKind::Error,
                            row: row_old,
                            col: col_old
                        }
                        }
                        if !digit.is_digit(10){
                            if digit == &'.'{
                                is_float = true;
                                dot_count += 1;
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
                lexer.col += final_digit.len() - 1;

                let mut digit_kind = TypeKind::Float(final_digit.parse::<f64>().unwrap());

                
                if !is_float {
                     digit_kind = TypeKind::Int(final_digit.parse::<i64>().unwrap());
                }
                Token { 
                    kind: TokenKind::Type(digit_kind), 
                    row: row_old,
                    col: col_old 
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
                lexer.col += final_char.len()-1;
                if KEYWORDS.contains(&final_char.as_str()) {
                    kind = TokenKind::Keyword(final_char);
                    
                } else {
                    kind = TokenKind::Ident(final_char);
                }
                
                Token { kind: kind, row: row_old, col: col_old }
            }
            _ => {
                Token { kind: TokenKind::Unknown(c.to_string()), row: row_old, col: col_old}
            }
        }
    } else {
        return Token { kind: TokenKind::Eof,  row: row_old, col: col_old}
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
        eprintln!("\x1b[33mInfo:\x1b[0m No input file was provided.");
        exit(-1);
    }
    let filepath  = &args[1].replace("\\", "/");
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
                println!("{:?} located at {}:{}:{}", token.kind, filepath, token.row, token.col);
            }
             
        }
    }
}

#[cfg(test)]
mod tests {
use std::fs::{read_dir, write};
use std::fmt::Write;
use std::iter::zip;

use super::*;
    const testpath: &str = "./tests";
    #[test]
    fn lex() {
        let mut failed_tests: Vec<String> = vec![];
        let mut result: String = "".to_string();
        for file in read_dir(testpath).unwrap() {
            
            let filepath = file.unwrap().path();
            let mut expected_path = filepath.clone();
            expected_path.set_extension("expected");
            
            if filepath.extension().unwrap().to_str() != Some("lei") {
                continue;
            }
            let filepath = filepath.into_string().unwrap().replace("\\", "/");
            
            let content = read_to_string(&filepath).unwrap();
            let content = content.trim();
            let mut lexer = Lexer {
                iter: content.chars().peekable(),
                col: 1,
                row: 1
            };
            
            loop {
                let token = token_next(&mut lexer, &filepath);
                match token.kind {
                    TokenKind::Eof => {
                        break;
                    }
                    _ => {
                        writeln!(result, "{:?} located at {}:{}:{}", token.kind, filepath, token.row, token.col).unwrap();
                    }
                    
                }
            }
            if !expected_path.exists() {
                write(&expected_path, &result).unwrap();
            }
            let expected_path = expected_path.into_string().unwrap().replace("\\", "/");
            let expected = read_to_string(&expected_path).unwrap();
            let mut err_msg: Vec<String> = vec![];
            if result != expected {
                let lines = zip(result.lines(), expected.lines());
                let mut i = 0;
                for (a, b) in lines {
                    i += 1;
                    if a != b {
                        err_msg.push(format!("\n\x1b[31mError:\x1b[0m {}:{}:{} expected: \n \"{}\" but got \n \"{}\"\n", &expected_path, i, 1, b, a));
                    }
                    
                }
                if !err_msg.is_empty() {

                    failed_tests.push(format!("-----------------Testcase----------------"));
                    failed_tests.append(&mut err_msg);
                }
            

            }

            result.clear();
        }
        if !failed_tests.is_empty() {
            for test in failed_tests.iter() {
                eprintln!("{}", test);
            }
            panic!();
        }
    }
}

