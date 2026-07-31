use std::{error::Error, fmt::Display};

use crate::{
    grammar::{BinOp, Decl, Expr, Literal, Program, Stmt},
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

    fn get_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    #[allow(unused)]
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    pub fn parse_input(&mut self) -> Program {
        let mut decls: Vec<Decl> = Vec::new();

        while let Some(token1) = self.get_token() {
            let token1_st = token1.st.clone();
            match token1.ty {
                TokenType::Id => {
                    self.next();

                    let token2 = self.get_token().unwrap();
                    match token2.ty {
                        TokenType::Equal => {
                            self.next();
                            let stmt = Stmt::Assign {
                                id: token1_st,
                                expr: self.parse_expr(),
                            };
                            //stmts.push(stmt);
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

        Program {
            declarations: decls,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        
        if let Some(token1) = self.get_token() {
            let lnum = token1.st.parse::<i64>().unwrap();
            #[allow(clippy::single_match)]
            match token1.ty {
                TokenType::Int => {
                    self.next();
                    if let Some(token2) = self.get_token() {
                        match token2.ty {
                            TokenType::Plus => {
                                self.next();
                                let lexpr = Expr::Literal(Literal::Interger(lnum as i128));
                                let left = Box::new(lexpr);
                                let right = Box::new(self.parse_expr());
                                Expr::BinOp {
                                    left,
                                    op: BinOps::Add,
                                    right,
                                }
                            }
                            TokenType::Semicolon => {
                                Expr::Literal(Literal::Interger(lnum as i128)
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexepectedEOF,
    UnexpectedToken(TokenType),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexepectedEOF => {
                write!(f, "Unexpected end of file")
            }
            ParserError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
        }
    }
}

impl Error for ParserError {}
