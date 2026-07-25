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
    pub ty: TokenType,
    pub st: String,
}

impl Token {
    pub fn new_type(ty: TokenType) -> Self {
        let st: String = match ty {
            TokenType::Equal => "=".to_string(),
            TokenType::EqualEqual => "==".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Asterisk => "*".to_string(),
            TokenType::Semicolon => ";".to_string(),
            _ => "".to_string(),
        };

        Self { ty, st }
    }
}

pub struct Lexer {
    buffer: String,
}

impl Lexer {
    pub fn new(buffer: String) -> Self {
        Self { buffer }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Vec<LexerError>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<LexerError> = vec![];
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
                        tokens.push(Token::new_type(TokenType::EqualEqual));
                    }
                } else {
                    tokens.push(Token::new_type(TokenType::Equal));
                }
            } else if ch == '+' {
                tokens.push(Token::new_type(TokenType::Plus));
            } else if ch == '-' {
                tokens.push(Token::new_type(TokenType::Minus));
            } else if ch == '*' {
                tokens.push(Token::new_type(TokenType::Asterisk));
            } else if ch == ';' {
                tokens.push(Token::new_type(TokenType::Semicolon));
            } else if ch == '\n' {
                line += 1;
            } else if ch == '\r' || ch == ' ' || ch == '\t' {
            } else {
                errors.push(LexerError::UnexepectedChar(line, ch));
            }
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
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
            ty: TokenType::Id,
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
            ty: TokenType::Num,
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
