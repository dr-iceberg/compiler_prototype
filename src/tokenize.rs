use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

use crate::tokenize::LexerError::{UnexepectedChar, UnexepectedEOF};

#[derive(Debug, Clone)]
pub enum TokenType {
    Unknown,

    // literals
    /// examples: asdf, _wersdf
    Id,
    /// examples 123, 1.5
    Num,
    /// examples: "sadfsadf"
    ///
    /// for now it doesnt handle escape charaters like '\n'
    Str,

    // single charater tokens
    /// `;`
    Semicolon,
    /// `=`
    Equal,
    /// `>`
    Greater,
    /// `<`
    Less,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Asterisk,
    /// `(`
    LPar,
    /// `)`
    RPar,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `/`
    ForwardSlash,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `!`
    Bang,

    // two charater tokens
    /// `!=`
    BangEqual,
    /// `==`
    EqualEqual,
    /// `>=`
    GreaterEqual,
    /// `<=`
    LessEqual,

    // Keywords
    Let,
    Fn,
    Struct,
    SElf,
    Super,

    For,
    While,

    Print,
    Return,
    If,
    Else,
    And,
    Or,
    True,
    False,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub st: String,
}

pub struct Lexer {
    buffer: String,
}

impl Lexer {
    pub fn new(buffer: String) -> Self {
        Self { buffer }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];
        let mut line: u32 = 1;

        let mut chars = self.buffer.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let token = Lexer::parse_id(ch, &mut chars);
                tokens.push(token);
            } else if ch.is_numeric() {
                let token = Lexer::parse_num(ch, &mut chars);
                tokens.push(token);
            } else if ch == '=' {
                if chars.peek() == Some(&ch) {
                    if ch == '=' {
                        tokens.push(Token {
                            kind: TokenType::EqualEqual,
                            st: String::from("=="),
                        });
                    }
                } else {
                    tokens.push(Token {
                        kind: TokenType::Equal,
                        st: String::from("="),
                    });
                }
            } else if ch == '+' {
                tokens.push(Token {
                    kind: TokenType::Plus,
                    st: String::from("+"),
                });
            } else if ch == '*' {
                tokens.push(Token {
                    kind: TokenType::Asterisk,
                    st: String::from("*"),
                });
            } else if ch == ';' {
                tokens.push(Token {
                    kind: TokenType::Semicolon,
                    st: String::from(";"),
                });
            } else if ch == '\n' {
                line += 1;
            } else if ch == '\r' || ch == ' ' {
            } else {
                return Err(LexerError::UnexepectedChar(line, ch));
            }
        }
        Ok(tokens)
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
            kind: TokenType::Id,
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
            kind: TokenType::Num,
            st: s,
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexepectedEOF,
    UnexepectedChar(u32, char), // line, char
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnexepectedEOF => {
                write!(f, "Unexpected End of File")
            }
            UnexepectedChar(line, ch) => {
                write!(f, "Line {line}, Unexpected character: {ch}")
            }
        }
    }
}

impl Error for LexerError {}
