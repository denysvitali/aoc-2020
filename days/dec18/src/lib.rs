use std::io::{BufReader, Error, BufRead, Read};
use std::fs::File;
use std::iter::Peekable;
use std::slice::Iter;
use crate::TokenType::{EOF, ParClose};
use std::str::Chars;

extern crate utils;

mod test;

#[derive(PartialEq, Clone, Debug)]
enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    ParOpen,
    ParClose,
    Num,
    EOF
}

#[derive(Clone, Debug)]
struct Token {
    t: TokenType,
    v: i32
}

fn get_token(buf: &mut Peekable<Chars>) -> Token {
    if buf.peek() == None {
        return Token {
            t: EOF,
            v: -1,
        }
    }

    let c = buf.next().unwrap();
    return match c {
        '+' => Token{ t: TokenType::Plus, v: 0 },
        '-' => Token{ t: TokenType::Minus, v: 0},
        '*' => Token{ t: TokenType::Star, v: 0},
        '(' => Token{ t: TokenType::ParOpen, v: 0},
        ')' => Token{ t: TokenType::ParClose, v: 0},
        _ => {
            if c.is_numeric() {
                return Token {t: TokenType::Num, v: c as i32 - '0' as i32}
            }
            if c == ' ' {
                return get_token(buf);
            }

            panic!("unknown token {}", c);
        }
    };
}

fn tokenize(input_str: &str) -> Vec<Token> {
    let mut tokens : Vec<Token> = Vec::new();
    let mut iter = input_str.chars().peekable();
    loop {
        let token = get_token(&mut iter);
        tokens.push(token.clone());
        if token.t == EOF {
            return tokens;
        }
    }
}

fn solve_puzzle(path: &str) -> Result<i32, Error>{
    let f = BufReader::new(File::open(path)?);
    let b = f.lines().map(|a|a.unwrap())
        .collect::<Vec<String>>();

    Ok(b.iter().map(|a|evaluate_expr(a.as_str()))
        .sum())
}

fn evaluate_expr(i: &str) -> i32 {
    let tokens = tokenize(i);
    let v = parse_expr(&mut tokens.iter().peekable());
    return v
}

fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> i32 {
    let value = num_or_par(tokens);
    let op = oper(tokens);
    if op.is_none() {
        let tp = tokens.peek().unwrap();
        if tp.t == EOF || tp.t == ParClose {
            return value.unwrap_or(0);
        }
        panic!("invalid operator")
    }
    tokens.next();
    let value = op.unwrap()(value.unwrap(), parse_expr(tokens));
    return value;
}



fn oper(tokens: &mut Peekable<Iter<Token>>) -> Option<Box<dyn Fn(i32, i32) -> i32>> {
    let t = tokens.peek()?;

    return match t.t {
        TokenType::Plus => Some(Box::new(|a,b| a+b)),
        TokenType::Star => Some(Box::new(|a,b| a*b)),
        _ => None
    }
}

fn num_or_par(p: &mut Peekable<Iter<Token>>) -> Option<i32> {
    let t = p.peek()?;
    return match t.t {
        TokenType::ParOpen => {
            paren_expr(p)
        },
        _ => num_expr(p)
    };
}

fn num_expr(tokens: &mut Peekable<Iter<Token>>) -> Option<i32> {
    let mut t = tokens.peek();
    if t.is_none() {
        return None;
    }
    let mut num = 0;
    let mut i = 0;
    loop {
        if t.is_none() || t.unwrap().t != TokenType::Num {
            return Some(num)
        }
        num += t.unwrap().v * 10_i32.pow(i);
        i += 1;
        tokens.next();
        t = tokens.peek();
    }
}

fn paren_expr(tokens: &mut Peekable<Iter<Token>>) -> Option<i32> {
    if tokens.peek().is_none() {
        return None
    }

    let t = tokens.peek().unwrap();
    if t.t == TokenType::ParOpen {
        tokens.next();
        let expr = parse_expr(tokens);
        let t = tokens.next();
        if t.is_none() || t.unwrap().t != TokenType::ParClose {
            panic!("unclosed paren");
        }
        return Some(expr)
    }

    return None
}