use std::{error::Error, fmt::Display, iter::Peekable, str::CharIndices};

use crate::{
    file::SourceFile,
    tokenize::LexerError::{UnexepectedChar, UnexepectedEOF},
};

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
    /// `:`
    Colon,
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
    /// `->`
    ArrowRight,

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

    // Types treated as keywords
    /// `int`
    IntType,
    /// `float`
    FloatType,
    /// `string`
    StringType,
    /// `bool`
    BoolType,

    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub fn new_single_char(pos: usize) -> Self {
        Self::new(pos, pos)
    }

    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self {
            start: start as u32,
            end: end as u32,
        }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }
    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        let start = self.span.start() as usize;
        let end = self.span.end() as usize;
        &source[start..=end]
    }
    pub fn pos(&self) -> u32 {
        self.span.start()
    }
}

pub struct Lexer<'a> {
    source_file: &'a SourceFile,
    line: u64,
}

impl<'a> Lexer<'a> {
    pub fn new(source_file: &'a SourceFile) -> Self {
        Self {
            source_file,
            line: 1,
        }
    }

    #[allow(unused)]
    fn skip_to_next_whitespace(&mut self, chars: &mut Peekable<CharIndices>) {
        todo!(
            "Skip to next whitespace after finding unexpected character in the middle of parsing a string or a number"
        )
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Vec<LexerError>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<LexerError> = vec![];

        let mut chars = self.source_file.source().char_indices().peekable();

        while let Some((pos, ch)) = chars.next() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                match self.read_str(pos, ch, &mut chars) {
                    Ok(token) => {
                        tokens.push(token);
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                }
            } else if ch.is_numeric() {
                match self.read_num(pos, ch, &mut chars) {
                    Ok(token) => {
                        tokens.push(token);
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                }
            } else if ch == '=' {
                if let Some(&(pos2, ch2)) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new(TokenType::EqualEqual, Span::new(pos, pos2)));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenType::Equal, Span::new_single_char(pos)));
                    }
                }
            } else if ch == '+' {
                tokens.push(Token::new(TokenType::Plus, Span::new_single_char(pos)));
            } else if ch == '-' {
                if let Some(&(pos2, ch2)) = chars.peek() {
                    if ch2 == '>' {
                        tokens.push(Token::new(TokenType::ArrowRight, Span::new(pos, pos2)));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenType::Minus, Span::new_single_char(pos)));
                    }
                }
            } else if ch == '*' {
                tokens.push(Token::new(TokenType::Asterisk, Span::new_single_char(pos)));
            } else if ch == ';' {
                tokens.push(Token::new(TokenType::Semicolon, Span::new_single_char(pos)));
            } else if ch == ':' {
                tokens.push(Token::new(TokenType::Colon, Span::new_single_char(pos)));
            } else if ch == '!' {
                if let Some(&(pos2, ch2)) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new(TokenType::BangEqual, Span::new(pos, pos2)));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenType::Bang, Span::new_single_char(pos)));
                    }
                }
            } else if ch == '(' {
                tokens.push(Token::new(TokenType::LPar, Span::new_single_char(pos)));
            } else if ch == ')' {
                tokens.push(Token::new(TokenType::RPar, Span::new_single_char(pos)));
            } else if ch == '{' {
                tokens.push(Token::new(TokenType::LBrace, Span::new_single_char(pos)));
            } else if ch == '}' {
                tokens.push(Token::new(TokenType::RBrace, Span::new_single_char(pos)));
            } else if ch == '<' {
                if let Some(&(pos2, ch2)) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new(TokenType::LessEqual, Span::new(pos, pos2)));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenType::Less, Span::new_single_char(pos)));
                    }
                }
            } else if ch == '>' {
                if let Some(&(pos2, ch2)) = chars.peek() {
                    if ch2 == '=' {
                        tokens.push(Token::new(TokenType::GreaterEqual, Span::new(pos, pos2)));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenType::Greater, Span::new_single_char(pos)));
                    }
                }
            } else if ch == '\n' {
                self.line += 1;
            } else if ch == '\r' || ch == ' ' || ch == '\t' {
            } else if ch == '/' {
                if let Some(&(_, ch2)) = chars.peek() {
                    // comment handling
                    if ch2 == '/' {
                        #[allow(clippy::while_let_on_iterator)]
                        while let Some((_, ch3)) = chars.next() {
                            if ch3 == '\n' {
                                break;
                            }
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::ForwardSlash,
                            Span::new_single_char(pos),
                        ));
                    }
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

    fn read_str(
        &self,
        pos: usize,
        ch: char,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Token, LexerError> {
        let mut st = String::from(ch);
        let mut pos2 = pos;

        while let Some(&(p, next)) = chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                st.push(next);
                chars.next();
            } else {
                //return Err(LexerError::UnexepectedChar(self.line, next));
                pos2 = p - 1; // p - 1 because we are at the next character
                break;
            }
        }

        // keyword recognition

        let ty = {
            if st == "fn" {
                TokenType::Fn
            } else if st == "let" {
                TokenType::Let
            } else if st == "struct" {
                TokenType::Struct
            } else if st == "self" {
                TokenType::SElf
            } else if st == "for" {
                TokenType::For
            } else if st == "while" {
                TokenType::While
            } else if st == "print" {
                TokenType::Print
            } else if st == "return" {
                TokenType::Return
            } else if st == "if" {
                TokenType::If
            } else if st == "else" {
                TokenType::Else
            } else if st == "and" {
                TokenType::And
            } else if st == "or" {
                TokenType::Or
            } else if st == "true" {
                TokenType::True
            } else if st == "false" {
                TokenType::False
            } else if st == "int" {
                TokenType::IntType
            } else if st == "float" {
                TokenType::FloatType
            } else if st == "string" {
                TokenType::StringType
            } else if st == "bool" {
                TokenType::BoolType
            } else {
                TokenType::Id
            }
        };

        Ok(Token::new(ty, Span::new(pos, pos2)))
    }

    fn read_num(
        &self,
        pos: usize,
        ch: char,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Token, LexerError> {
        let mut s = String::from(ch);
        let mut dot = false;
        let mut pos2 = pos;

        while let Some(&(p, next)) = chars.peek() {
            if next == '.' {
                if !dot {
                    s.push(next);
                    dot = true;
                    chars.next();
                } else {
                    return Err(LexerError::UnexepectedChar(self.line, next));
                }
            } else if next.is_alphanumeric() || next == '_' {
                s.push(next);
                chars.next();
            } else {
                //return Err(LexerError::UnexepectedChar(self.line, next));
                pos2 = p - 1; // p - 1 because we are at the next character
                break;
            }
        }

        let ty = {
            if dot {
                TokenType::Float
            } else {
                TokenType::Int
            }
        };

        Ok(Token::new(ty, Span::new(pos, pos2)))
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
                write!(f, "Line {line}, Unexpected character: '{ch}'")
            }
        }
    }
}

impl Error for LexerError {}
