#![allow(dead_code, unused_variables)]

use std::io::BufReader;
use std::{fs::File, io::Read, iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
enum TokenKind {
    Unknown,
    Semicolon,
    Id,
    Num,
    Equal,
    Plus,
    Minus,
    Asterisk,
    LPar,
    RPar,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    st: String,
}

#[derive(Debug)]
enum Expr {
    Empty,
    Number(i64),
    Identifier(String),
    BinOp {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
enum Stmt {
    Empty,
    Assign { id: String, value: Expr },
}

#[derive(Debug)]
enum Scope {
    Scope,
    Function(Vec<Stmt>),
}

struct Lexer {
    buffer: String,
}

impl Lexer {
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut chars = self.buffer.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let token = Lexer::parse_id(ch, &mut chars);
                tokens.push(token);
            } else if ch.is_numeric() {
                let token = Lexer::parse_num(ch, &mut chars);
                tokens.push(token);
            } else if ch == '=' {
                tokens.push(Token {
                    kind: TokenKind::Equal,
                    st: String::from("="),
                });
            } else if ch == '+' {
                tokens.push(Token {
                    kind: TokenKind::Plus,
                    st: String::from("+"),
                });
            } else if ch == '*' {
                tokens.push(Token {
                    kind: TokenKind::Asterisk,
                    st: String::from("*"),
                });
            } else if ch == ';' {
                tokens.push(Token {
                    kind: TokenKind::Semicolon,
                    st: String::from(";"),
                });
            }
        }
        tokens
    }

    fn parse_id(ch: char, chars: &mut Peekable<Chars>) -> Token {
        let mut s = String::from(ch);

        while let Some(next) = chars.peek().copied() {
            if next.is_alphanumeric() || next == '_' {
                s.push(next);
                chars.next();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Id,
            st: s,
        }
    }

    fn parse_num(ch: char, chars: &mut Peekable<Chars>) -> Token {
        let mut s = String::from(ch);
        let mut dot = false;

        while let Some(next) = chars.peek().copied() {
            if next == '.' && !dot {
                dot = true;
            } else if next.is_alphanumeric() || next == '_' {
                s.push(next);
                chars.next();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Num,
            st: s,
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn get_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn parse_input(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = vec![];

        while let Some(token1) = self.get_token() {
            let token1_st = token1.st.clone();
            match token1.kind {
                TokenKind::Id => {
                    self.next();

                    let token2 = self.get_token().unwrap();
                    match token2.kind {
                        TokenKind::Equal => {
                            self.next();
                            let stmt = Stmt::Assign {
                                id: token1_st,
                                value: self.parse_expr(),
                            };
                            stmts.push(stmt);
                        }
                        _ => {
                            self.next();
                        }
                    }
                }
                _ => {
                    self.next();
                }
            }
        }

        stmts
    }

    fn parse_expr(&mut self) -> Expr {
        let mut expr = Expr::Empty;
        if let Some(token1) = self.get_token() {
            let lnum = token1.st.parse::<i64>().unwrap();
            match token1.kind {
                TokenKind::Num => {
                    self.next();
                    if let Some(token2) = self.get_token() {
                        match token2.kind {
                            TokenKind::Plus => {
                                self.next();
                                let lexpr = Expr::Number(lnum);
                                let left = Box::new(lexpr);
                                let op = '+';
                                let right = Box::new(self.parse_expr());
                                expr = Expr::BinOp {
                                    left: left,
                                    op: op,
                                    right: right,
                                }
                            }
                            TokenKind::Semicolon => {
                                expr = Expr::Number(lnum);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        expr
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("src.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let mut lexer = Lexer { buffer: buffer };

    let tokens = lexer.tokenize();

    let mut parser = Parser {
        tokens: tokens.clone(),
        pos: 0,
    };
    let stmts = parser.parse_input();

    for token in tokens {
        println!("{:?}", token);
    }

    for stmt in stmts {
        println!("{:?}", stmt);
    }

    Ok(())
}
