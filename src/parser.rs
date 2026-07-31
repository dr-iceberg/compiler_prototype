use std::{error::Error, fmt::Display};

use crate::{
    grammar::{Decl, Expr, Function, Literal, Parameter, Program, Stmt, Type},
    tokenize::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        Some(token)
    }

    fn check(&self, typ: TokenType) -> Result<&Token, ParserError> {
        let token = self.current().ok_or(ParserError::UnexpectedEOF)?;
        if token.ty == typ {
            Ok(token)
        } else {
            Err(ParserError::UnexpectedToken(token.ty))
        }
    }

    fn check_any(&self, types: &[TokenType]) -> Result<&Token, ParserError> {
        let token = self.current().ok_or(ParserError::UnexpectedEOF)?;
        for typ in types {
            if token.ty == *typ {
                return Ok(token);
            }
        }
        Err(ParserError::UnexpectedToken(token.ty))
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut decls: Vec<Decl> = Vec::new();
        let mut errors: Vec<ParserError> = Vec::new();

        while let Some(token) = self.current() {
            match token.ty {
                TokenType::Fn => match self.parse_function() {
                    Ok(function) => {
                        decls.push(Decl::Function(function));
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                },
                _ => {
                    errors.push(ParserError::UnexpectedToken(token.ty));
                }
            }
        }

        Ok(Program {
            declarations: decls,
        })
    }

    fn parse_function(&mut self) -> Result<Function, ParserError> {
        // skip the fn keyword
        self.advance();

        // check if next token is an id
        let id = self.check(TokenType::Id)?.st.clone();
        self.advance();

        let parameters = self.parse_params()?;
        let return_type = self.parse_function_return_type()?;
        let body = self.parse_function_body()?;

        Ok(Function {
            id,
            parameters,
            body,
            return_type,
        })
    }

    fn convert_token_to_type(token: &Token) -> Result<Type, ParserError> {
        match token.ty {
            TokenType::IntType => Ok(Type::Int),
            TokenType::FloatType => Ok(Type::Float),
            TokenType::StringType => Ok(Type::String),
            TokenType::BoolType => Ok(Type::Bool),
            _ => Err(ParserError::UnexpectedToken(token.ty)),
        }
    }

    fn parse_params(&mut self) -> Result<Vec<Parameter>, ParserError> {
        let mut params: Vec<Parameter> = Vec::new();
        // left parenthesis must be present
        let _ = self.check(TokenType::LPar)?;
        self.advance();
        while let Some(token) = self.current() {
            if token.ty != TokenType::RPar {
                let name = self.check(TokenType::Id)?.st.clone();
                self.advance();
                let _ = self.check(TokenType::Colon)?;
                self.advance();
                let ty_token = self.check_any(&[
                    TokenType::IntType,
                    TokenType::FloatType,
                    TokenType::StringType,
                    TokenType::BoolType,
                ])?;
                let ty = Self::convert_token_to_type(ty_token)?;
                let param = Parameter { name, ty };
                self.advance();
                params.push(param);
            } else {
                self.advance();
                break;
            }
        }

        Ok(params)
    }

    fn parse_function_return_type(&mut self) -> Result<Type, ParserError> {
        match self.check(TokenType::ArrowRight) {
            Ok(_) => {
                self.advance();
                let ty_token = self.check_any(&[
                    TokenType::IntType,
                    TokenType::FloatType,
                    TokenType::StringType,
                    TokenType::BoolType,
                ])?;
                let ty = Self::convert_token_to_type(ty_token)?;
                self.advance();
                Ok(ty)
            }
            Err(err) => match err {
                ParserError::UnexpectedEOF => Err(err),
                ParserError::UnexpectedToken(_) => Ok(Type::Void),
            },
        }
    }

    fn parse_function_body(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();
        self.check(TokenType::LBrace)?;
        self.advance();
        while let Some(token) = self.current() {
            match token.ty {
                TokenType::Return => {
                    self.advance();
                    let expr: Expr = self.parse_expr()?;
                    self.advance();
                    let stmt = Stmt::Return(expr);
                    statements.push(stmt);
                }
                TokenType::RBrace => {
                    self.advance();
                    break;
                }
                _ => return Err(ParserError::UnexpectedToken(token.ty)),
            }
        }
        Ok(statements)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        let token = self.advance().ok_or(ParserError::UnexpectedEOF)?;

        match token.ty {
            TokenType::Int => Ok(Expr::Literal(Literal::Interger(token.st.parse().unwrap()))),
            _ => Err(ParserError::UnexpectedToken(token.ty)),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnexpectedToken(TokenType),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
            ParserError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
        }
    }
}

impl Error for ParserError {}
