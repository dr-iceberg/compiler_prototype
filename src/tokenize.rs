use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub enum TokenKind {
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
pub struct Token {
    pub kind: TokenKind,
    pub st: String,
}

pub struct Lexer {
    pub buffer: String,
}

impl Lexer {
    pub fn tokenize(&mut self) -> Vec<Token> {
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
