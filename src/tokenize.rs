use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

use crate::tokenize::LexerError::{UnexepectedChar, UnexepectedEOF};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Unknown,

    // literals
    /// examples: `asdf`, `_wersdf`
    Id,
    /// examples: `123`
    Int,
    /// examples: `1.5`, `53.05`
    Float,
    /// examples: `"sadfsadf"`
    ///
    /// for now it doesnt handle escape charaters like `'\n'`, `'\t'`, etc.
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
    /// `self`
    SElf,

    For,
    While,

    /// Its a keyword, for simplicity
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
    // later a span should be used here
    // it would hold the byte offset of the token in original source code
    // and could be used to access the original string from there
    // instead of holding it in a separate String and thus duplicating memory
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
            TokenType::ForwardSlash => "/".to_string(),
            TokenType::LPar => "(".to_string(),
            TokenType::RPar => ")".to_string(),
            TokenType::LBrace => "{".to_string(),
            TokenType::RBrace => "}".to_string(),
            TokenType::Less => "<".to_string(),
            TokenType::LessEqual => "<=".to_string(),
            TokenType::Greater => ">".to_string(),
            TokenType::GreaterEqual => ">=".to_string(),
            TokenType::Bang => "!".to_string(),
            _ => "".to_string(),
        };

        Self { ty, st }
    }
}

pub struct Lexer {
    buffer: String,
    line: u64,
}

impl Lexer {
    pub fn new(buffer: String) -> Self {
        Self { buffer, line: 1 }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Vec<LexerError>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<LexerError> = vec![];

        let mut chars = self.buffer.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                match self.read_str(ch, &mut chars) {
                    Ok(token) => {
                        tokens.push(token);
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                }
            } else if ch.is_numeric() {
                match self.read_num(ch, &mut chars) {
                    Ok(token) => {
                        tokens.push(token);
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                }
            } else if ch == '=' {
                if let Some(&ch2) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new_type(TokenType::EqualEqual));
                        chars.next();
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
            } else if ch == '!' {
                tokens.push(Token::new_type(TokenType::Bang));
            } else if ch == '(' {
                tokens.push(Token::new_type(TokenType::LPar));
            } else if ch == ')' {
                tokens.push(Token::new_type(TokenType::RPar));
            } else if ch == '{' {
                tokens.push(Token::new_type(TokenType::LBrace));
            } else if ch == '}' {
                tokens.push(Token::new_type(TokenType::RBrace));
            } else if ch == '<' {
                if let Some(&ch2) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new_type(TokenType::LessEqual));
                        chars.next();
                    }
                } else {
                    tokens.push(Token::new_type(TokenType::Less));
                }
            } else if ch == '>' {
                if let Some(&ch2) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new_type(TokenType::GreaterEqual));
                        chars.next();
                    }
                } else {
                    tokens.push(Token::new_type(TokenType::Greater));
                }
            } else if ch == '\n' {
                self.line += 1;
            } else if ch == '\r' || ch == ' ' || ch == '\t' {
            } else if ch == '/' {
                if let Some(&ch2) = chars.peek() {
                    // comment handling
                    if ch2 == '/' {
                        #[allow(clippy::while_let_on_iterator)]
                        while let Some(ch3) = chars.next() {
                            if ch3 == '\n' {
                                break;
                            }
                        }
                    }
                } else {
                    tokens.push(Token::new_type(TokenType::ForwardSlash));
                }
            } else {
                errors.push(LexerError::UnexepectedChar(self.line, ch));
            }
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn read_str(&self, ch: char, chars: &mut Peekable<Chars>) -> Result<Token, LexerError> {
        let mut s = String::from(ch);

        while let Some(next) = chars.peek().copied() {
            if next.is_alphanumeric() || next == '_' {
                s.push(next);
                chars.next();
            } else {
                return Err(LexerError::UnexepectedChar(self.line, next));
            }
        }

        // keyword recognition

        let ty = {
            if s == "fn" {
                TokenType::Fn
            } else if s == "let" {
                TokenType::Let
            } else if s == "struct" {
                TokenType::Struct
            } else if s == "self" {
                TokenType::SElf
            } else if s == "for" {
                TokenType::For
            } else if s == "while" {
                TokenType::While
            } else if s == "print" {
                TokenType::Print
            } else if s == "return" {
                TokenType::Return
            } else if s == "if" {
                TokenType::If
            } else if s == "else" {
                TokenType::Else
            } else if s == "and" {
                TokenType::And
            } else if s == "or" {
                TokenType::Or
            } else if s == "true" {
                TokenType::True
            } else if s == "false" {
                TokenType::False
            } else {
                TokenType::Id
            }
        };

        Ok(Token { ty, st: s })
    }

    fn read_num(&self, ch: char, chars: &mut Peekable<Chars>) -> Result<Token, LexerError> {
        let mut s = String::from(ch);
        let mut dot = false;

        while let Some(next) = chars.peek().copied() {
            if next == '.' && !dot {
                dot = true;
            } else if next.is_alphanumeric() || next == '_' {
                s.push(next);
                chars.next();
            } else {
                return Err(LexerError::UnexepectedChar(self.line, next));
            }
        }

        if dot {
            Ok(Token {
                ty: TokenType::Float,
                st: s,
            })
        } else {
            Ok(Token {
                ty: TokenType::Int,
                st: s,
            })
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexepectedEOF,
    UnexepectedChar(u64, char), // line, char
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
