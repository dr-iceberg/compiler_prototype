use std::{error::Error, fmt::Display, str::FromStr};

use crate::{
    file::SourceFile,
    grammar::{
        BinOp, Decl, Expr, ExprId, ExprKind, Function, Literal, Parameter, Program, Stmt, Type,
        UnaryOp,
    },
    tokenize::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParserError<'a>>,
    source: &'a SourceFile,
    expr_id: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a SourceFile) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            source,
            expr_id: 0,
        }
    }

    fn next_expr_id(&mut self) -> ExprId {
        let id = ExprId::new(self.expr_id);
        self.expr_id += 1;
        id
    }

    fn make_expr(&mut self, kind: ExprKind) -> Expr {
        Expr::new(self.next_expr_id(), kind)
    }

    #[must_use]
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn match_token(&mut self, ty: TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, typ: TokenType) -> bool {
        self.current().is_some_and(|token| token.ty == typ)
    }

    // this function is a #[must_use] in order to avoid forgetting the ? operator
    #[must_use]
    fn expect(&mut self, ty: TokenType) -> Option<Token> {
        let token = match self.current() {
            Some(token) => token,
            None => {
                self.errors.push(ParserError::UnexpectedEOF);
                return None;
            }
        };

        if token.ty != ty {
            self.errors.push(ParserError::UnexpectedToken {
                token: *token,
                source: self.source,
            });
            return None;
        }

        let token = *token;
        self.advance();
        Some(token)
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParserError<'_>>> {
        let mut decls: Vec<Decl> = Vec::new();

        while let Some(token) = self.current() {
            match token.ty {
                TokenType::Fn => match self.parse_function() {
                    Some(function) => {
                        decls.push(Decl::Function(function));
                    }
                    None => {
                        self.skip_to_next_decl();
                    }
                },
                _ => {
                    self.errors.push(ParserError::UnexpectedToken {
                        token: *token,
                        source: self.source,
                    });
                    self.skip_to_next_decl();
                }
            }
        }
        if self.errors.is_empty() {
            Ok(Program {
                declarations: decls,
            })
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn skip_to_next_decl(&mut self) {
        while let Some(token) = self.current() {
            if token.ty == TokenType::Fn {
                return;
            }
            self.advance();
        }
    }

    #[must_use]
    fn parse_function(&mut self) -> Option<Function> {
        self.expect(TokenType::Fn)?;
        let id = self
            .expect(TokenType::Id)?
            .lexeme(self.source.source())
            .to_string();
        let parameters = self.parse_params()?;
        let return_type = self.parse_function_return_type()?;
        let body = self.parse_function_body()?;

        Some(Function {
            id,
            parameters,
            body,
            return_type,
        })
    }

    #[must_use]
    fn parse_params(&mut self) -> Option<Vec<Parameter>> {
        let mut params: Vec<Parameter> = Vec::new();
        self.expect(TokenType::LPar)?;
        while let Some(token) = self.current() {
            if token.ty != TokenType::RPar {
                let name = self
                    .expect(TokenType::Id)?
                    .lexeme(self.source.source())
                    .to_string();
                self.expect(TokenType::Colon)?;
                let ty = self.parse_type()?;
                let param = Parameter { name, ty };
                params.push(param);
            } else {
                self.advance();
                break;
            }
        }

        Some(params)
    }

    #[must_use]
    fn parse_type(&mut self) -> Option<Type> {
        match self.current()?.ty {
            TokenType::IntType => {
                self.advance();
                Some(Type::Int)
            }
            TokenType::FloatType => {
                self.advance();
                Some(Type::Float)
            }
            TokenType::BoolType => {
                self.advance();
                Some(Type::Bool)
            }
            TokenType::StringType => {
                self.advance();
                Some(Type::String)
            }
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    token: *self.current()?,
                    source: self.source,
                });
                None
            }
        }
    }

    #[must_use]
    fn parse_function_return_type(&mut self) -> Option<Type> {
        if self.match_token(TokenType::ArrowRight) {
            self.parse_type()
        } else {
            Some(Type::Void)
        }
    }

    #[must_use]
    fn parse_function_body(&mut self) -> Option<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();
        self.expect(TokenType::LBrace)?;
        while let Some(token) = self.current() {
            if token.ty == TokenType::RBrace {
                break;
            }
            statements.push(self.parse_stmt()?);
        }
        self.expect(TokenType::RBrace)?;

        Some(statements)
    }

    #[must_use]
    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current()?.ty {
            TokenType::Return => self.parse_return_stmt(),
            TokenType::Let => self.parse_let_stmt(),
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    token: *self.current()?,
                    source: self.source,
                });
                None
            }
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.expect(TokenType::Let)?;
        let id = self
            .expect(TokenType::Id)?
            .lexeme(self.source.source())
            .to_string();
        self.expect(TokenType::Equal)?;
        let expr = self.parse_expr()?;
        self.expect(TokenType::Semicolon)?;
        Some(Stmt::LetAssign { id, expr })
    }

    #[must_use]
    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.expect(TokenType::Return)?;
        let expr = self.parse_expr()?;
        self.expect(TokenType::Semicolon)?;
        Some(Stmt::Return(expr))
    }

    #[must_use]
    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_equality()
    }

    #[must_use]
    fn parse_equality(&mut self) -> Option<Expr> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = if self.match_token(TokenType::EqualEqual) {
                BinOp::Equal
            } else if self.match_token(TokenType::BangEqual) {
                BinOp::NotEqual
            } else {
                break;
            };
            let right = self.parse_comparison()?;

            let kind = ExprKind::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };

            expr = self.make_expr(kind);
        }

        Some(expr)
    }

    #[must_use]
    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_term()?;

        loop {
            let op = if self.match_token(TokenType::Less) {
                BinOp::Less
            } else if self.match_token(TokenType::LessEqual) {
                BinOp::LessEqual
            } else if self.match_token(TokenType::Greater) {
                BinOp::Greater
            } else if self.match_token(TokenType::GreaterEqual) {
                BinOp::GreaterEqual
            } else {
                break;
            };
            let right = self.parse_term()?;

            let kind = ExprKind::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };

            expr = self.make_expr(kind);
        }

        Some(expr)
    }

    #[must_use]
    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        loop {
            let op = if self.match_token(TokenType::Plus) {
                BinOp::Add
            } else if self.match_token(TokenType::Minus) {
                BinOp::Subtract
            } else {
                break;
            };
            let right = self.parse_factor()?;
            let kind = ExprKind::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };

            expr = self.make_expr(kind);
        }

        Some(expr)
    }

    #[must_use]
    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = if self.match_token(TokenType::Asterisk) {
                BinOp::Multiply
            } else if self.match_token(TokenType::ForwardSlash) {
                BinOp::Divide
            } else {
                break;
            };
            let right = self.parse_unary()?;
            let kind = ExprKind::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };

            expr = self.make_expr(kind);
        }

        Some(expr)
    }

    #[must_use]
    fn parse_unary(&mut self) -> Option<Expr> {
        if self.match_token(TokenType::Minus) {
            let kind = ExprKind::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(self.parse_unary()?),
            };
            return Some(self.make_expr(kind));
        }
        if self.match_token(TokenType::Bang) {
            let kind = ExprKind::Unary {
                op: UnaryOp::Not,
                expr: Box::new(self.parse_unary()?),
            };
            return Some(self.make_expr(kind));
        }

        self.parse_primary()
    }

    #[must_use]
    fn parse_primary(&mut self) -> Option<Expr> {
        match self.current()?.ty {
            TokenType::Int => {
                let token = self.expect(TokenType::Int)?;
                let val = self.get_token_value(&token)?;
                Some(self.make_expr(ExprKind::Literal(Literal::Interger(val))))
            }
            TokenType::Float => {
                let token = self.expect(TokenType::Float)?;
                let val = self.get_token_value(&token)?;
                Some(self.make_expr(ExprKind::Literal(Literal::Float(val))))
            }
            TokenType::True => {
                let token = self.expect(TokenType::True)?;
                let val = self.get_token_value(&token)?;
                Some(self.make_expr(ExprKind::Literal(Literal::Bool(val))))
            }
            TokenType::False => {
                let token = self.expect(TokenType::False)?;
                let val = self.get_token_value(&token)?;
                Some(self.make_expr(ExprKind::Literal(Literal::Bool(val))))
            }
            TokenType::Str => {
                let token = self.expect(TokenType::Str)?;
                let val = self.get_token_value(&token)?;
                Some(self.make_expr(ExprKind::Literal(Literal::String(val))))
            }
            TokenType::Id => {
                let token = self.expect(TokenType::Id)?;
                let id = self.get_token_value(&token)?;
                if self.current()?.ty == TokenType::LPar {
                    let mut args: Vec<Expr> = Vec::new();
                    // function call
                    self.expect(TokenType::LPar)?;

                    while self.current()?.ty != TokenType::RPar {
                        args.push(self.parse_expr()?);

                        if !self.match_token(TokenType::Colon) {
                            break;
                        }
                        self.advance();
                    }

                    self.expect(TokenType::RPar)?;
                    Some(self.make_expr(ExprKind::Call { id, args }))
                } else {
                    Some(self.make_expr(ExprKind::Identifier(id)))
                }
            }
            TokenType::LPar => {
                self.expect(TokenType::LPar)?;

                let expr = self.parse_expr()?;

                self.expect(TokenType::RPar)?;
                Some(expr)
            }
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    token: *self.current()?,
                    source: self.source,
                });
                None
            }
        }
    }

    #[must_use]
    fn get_token_value<T>(&mut self, token: &Token) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: 'static,
    {
        match token.lexeme(self.source.source()).parse::<T>() {
            Ok(val) => Some(val),
            Err(err) => {
                self.errors
                    .push(ParserError::LiteralParsingError(Box::new(err)));
                None
            }
        }
    }
}

#[derive(Debug)]
pub enum ParserError<'a> {
    UnexpectedEOF,
    LiteralParsingError(Box<dyn Error>),
    UnexpectedToken {
        token: Token,
        source: &'a SourceFile,
    },
}

impl<'a> Display for ParserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
            ParserError::UnexpectedToken { token, source } => {
                write!(
                    f,
                    "Unexpected token: '{}' at line: {} ",
                    token.lexeme(source.source()),
                    source.get_line_num(token.pos())
                )
            }
            ParserError::LiteralParsingError(err) => {
                write!(f, "Error when parsing a literal: {err}")
            }
        }
    }
}

impl<'a> Error for ParserError<'a> {}
