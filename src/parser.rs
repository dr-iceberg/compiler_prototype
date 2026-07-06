use crate::{
    grammar::{BinOps, Expr, Stmt},
    tokenize::{Token, TokenKind},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
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

    pub fn parse_input(&mut self) -> Vec<Stmt> {
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
                                expr: self.parse_expr(),
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
                                let right = Box::new(self.parse_expr());
                                expr = Expr::BinOp {
                                    left: left,
                                    op: BinOps::Add,
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
